use crate::database::DatabaseManager;
use crate::errors::{BookSourceError, BookSourceResult};
use crate::models::{BookSourceInfo, HtmlElement};
use chrono::Utc;
use reqwest::Client;
use scraper::{Html, Selector};
use serde_json::Value;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs;
use url::Url;

/// 书源管理器
pub struct BookSourceManager {
    db: Arc<DatabaseManager>,
    plugin_dir: PathBuf,
    loaded_plugins: HashMap<String, String>, // source_id -> JavaScript code
    http_client: Client,
}

impl BookSourceManager {
    /// 创建新的书源管理器
    pub fn new(db: Arc<DatabaseManager>, plugin_dir: PathBuf) -> BookSourceResult<Self> {
        // 确保插件目录存在
        if !plugin_dir.exists() {
            std::fs::create_dir_all(&plugin_dir)
                .map_err(|e| BookSourceError::LoadFailed(format!("创建插件目录失败: {}", e)))?;
        }

        let http_client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .user_agent("NovelNest/1.0")
            .build()
            .map_err(|e| BookSourceError::NetworkError(format!("HTTP客户端初始化失败: {}", e)))?;

        Ok(Self {
            db,
            plugin_dir,
            loaded_plugins: HashMap::new(),
            http_client,
        })
    }

    /// 加载书源插件
    pub async fn load_source(&mut self, plugin_path: PathBuf) -> BookSourceResult<BookSourceInfo> {
        // 验证插件文件存在
        if !plugin_path.exists() {
            return Err(BookSourceError::NotFound("插件文件不存在".to_string()));
        }

        // 读取插件元数据
        let plugin_info = self.parse_plugin_metadata(&plugin_path).await?;

        // 读取插件JavaScript代码
        let js_code = self.read_plugin_code(&plugin_path).await?;

        // 验证插件代码格式
        self.validate_plugin_code(&js_code)?;

        // 将插件信息保存到数据库
        self.save_plugin_to_database(&plugin_info).await?;

        // 缓存插件代码
        self.loaded_plugins.insert(plugin_info.id.clone(), js_code);

        Ok(plugin_info)
    }

    /// 卸载书源插件
    pub async fn unload_source(&mut self, source_id: &str) -> BookSourceResult<()> {
        // 从数据库中删除插件信息
        self.remove_plugin_from_database(source_id).await?;

        // 从内存中移除插件代码
        self.loaded_plugins.remove(source_id);

        Ok(())
    }

    /// 获取所有书源
    pub async fn get_sources(&self) -> BookSourceResult<Vec<BookSourceInfo>> {
        self.load_sources_from_database().await
    }

    /// 切换书源启用状态
    pub async fn toggle_source(&mut self, source_id: &str, enabled: bool) -> BookSourceResult<()> {
        self.update_source_status(source_id, enabled).await
    }

    /// 获取插件代码
    pub fn get_plugin_code(&self, source_id: &str) -> Option<&String> {
        self.loaded_plugins.get(source_id)
    }

    /// 初始化书源管理器，加载所有已安装的插件
    pub async fn initialize(&mut self) -> BookSourceResult<()> {
        let sources = self.get_sources().await?;
        
        for source in sources {
            if source.enabled {
                let plugin_path = PathBuf::from(&source.plugin_path);
                if plugin_path.exists() {
                    match self.read_plugin_code(&plugin_path).await {
                        Ok(js_code) => {
                            self.loaded_plugins.insert(source.id.clone(), js_code);
                        }
                        Err(e) => {
                            eprintln!("加载插件 {} 失败: {}", source.id, e);
                            // 自动禁用有问题的插件
                            let _ = self.toggle_source(&source.id, false).await;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// 解析插件元数据
    async fn parse_plugin_metadata(&self, plugin_path: &Path) -> BookSourceResult<BookSourceInfo> {
        let plugin_dir = plugin_path.parent()
            .ok_or_else(|| BookSourceError::InvalidFormat("无效的插件路径".to_string()))?;

        let metadata_path = plugin_dir.join("plugin.json");
        
        if !metadata_path.exists() {
            return Err(BookSourceError::InvalidFormat("缺少plugin.json元数据文件".to_string()));
        }

        let metadata_content = fs::read_to_string(&metadata_path).await
            .map_err(|e| BookSourceError::LoadFailed(format!("读取元数据文件失败: {}", e)))?;

        let metadata: Value = serde_json::from_str(&metadata_content)
            .map_err(|e| BookSourceError::ParseError(format!("解析元数据失败: {}", e)))?;

        let id = metadata["id"].as_str()
            .ok_or_else(|| BookSourceError::InvalidFormat("缺少插件ID".to_string()))?
            .to_string();

        let name = metadata["name"].as_str()
            .ok_or_else(|| BookSourceError::InvalidFormat("缺少插件名称".to_string()))?
            .to_string();

        let version = metadata["version"].as_str()
            .ok_or_else(|| BookSourceError::InvalidFormat("缺少插件版本".to_string()))?
            .to_string();

        let author = metadata["author"].as_str().map(|s| s.to_string());
        let description = metadata["description"].as_str().map(|s| s.to_string());
        
        let base_url = metadata["baseUrl"].as_str()
            .ok_or_else(|| BookSourceError::InvalidFormat("缺少baseUrl".to_string()))?
            .to_string();

        Ok(BookSourceInfo {
            id,
            name,
            version,
            author,
            description,
            base_url,
            enabled: true,
            plugin_path: plugin_path.to_string_lossy().to_string(),
            installed_at: Utc::now(),
        })
    }

    /// 读取插件JavaScript代码
    async fn read_plugin_code(&self, plugin_path: &Path) -> BookSourceResult<String> {
        fs::read_to_string(plugin_path).await
            .map_err(|e| BookSourceError::LoadFailed(format!("读取插件代码失败: {}", e)))
    }

    /// 验证插件代码格式
    fn validate_plugin_code(&self, js_code: &str) -> BookSourceResult<()> {
        // 基本的JavaScript代码验证
        if js_code.trim().is_empty() {
            return Err(BookSourceError::InvalidFormat("插件代码为空".to_string()));
        }

        // 检查必需的方法是否存在
        let required_methods = ["search", "getChapters", "getChapterContent"];
        for method in &required_methods {
            if !js_code.contains(method) {
                return Err(BookSourceError::InvalidFormat(
                    format!("插件缺少必需的方法: {}", method)
                ));
            }
        }

        Ok(())
    }

    /// 保存插件信息到数据库
    async fn save_plugin_to_database(&self, plugin_info: &BookSourceInfo) -> BookSourceResult<()> {
        self.db.insert_book_source(plugin_info)
            .map_err(|e| BookSourceError::LoadFailed(format!("保存插件信息失败: {}", e)))
    }

    /// 从数据库中删除插件信息
    async fn remove_plugin_from_database(&self, source_id: &str) -> BookSourceResult<()> {
        self.db.delete_book_source(source_id)
            .map_err(|e| BookSourceError::LoadFailed(format!("删除插件信息失败: {}", e)))
    }

    /// 从数据库加载所有书源
    async fn load_sources_from_database(&self) -> BookSourceResult<Vec<BookSourceInfo>> {
        self.db.get_all_book_sources()
            .map_err(|e| BookSourceError::LoadFailed(format!("加载书源失败: {}", e)))
    }

    /// 更新书源启用状态
    async fn update_source_status(&self, source_id: &str, enabled: bool) -> BookSourceResult<()> {
        self.db.toggle_book_source(source_id, enabled)
            .map_err(|e| BookSourceError::LoadFailed(format!("更新书源状态失败: {}", e)))
    }
}

/// HTTP工具函数
pub struct HttpUtils {
    client: Client,
}

impl HttpUtils {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .user_agent("NovelNest/1.0")
            .build()
            .unwrap();
        
        Self { client }
    }

    /// GET请求
    pub async fn get(&self, url: &str) -> BookSourceResult<String> {
        let response = self.client.get(url)
            .send()
            .await
            .map_err(|e| BookSourceError::HttpRequestFailed(format!("GET请求失败: {}", e)))?;

        if !response.status().is_success() {
            return Err(BookSourceError::HttpRequestFailed(
                format!("HTTP错误: {}", response.status())
            ));
        }

        response.text()
            .await
            .map_err(|e| BookSourceError::HttpRequestFailed(format!("读取响应失败: {}", e)))
    }

    /// POST请求
    pub async fn post(&self, url: &str, data: &str) -> BookSourceResult<String> {
        let response = self.client.post(url)
            .header("Content-Type", "application/json")
            .body(data.to_string())
            .send()
            .await
            .map_err(|e| BookSourceError::HttpRequestFailed(format!("POST请求失败: {}", e)))?;

        if !response.status().is_success() {
            return Err(BookSourceError::HttpRequestFailed(
                format!("HTTP错误: {}", response.status())
            ));
        }

        response.text()
            .await
            .map_err(|e| BookSourceError::HttpRequestFailed(format!("读取响应失败: {}", e)))
    }
}

/// HTML解析工具
pub struct HtmlParser;

impl HtmlParser {
    /// 解析HTML并提取元素
    pub fn parse_elements(html: &str, selector: &str) -> BookSourceResult<Vec<HtmlElement>> {
        let document = Html::parse_document(html);
        let selector = Selector::parse(selector)
            .map_err(|e| BookSourceError::HtmlParseError(format!("选择器解析失败: {:?}", e)))?;

        let mut elements = Vec::new();
        for element in document.select(&selector) {
            let text = element.text().collect::<Vec<_>>().join("");
            let href = element.value().attr("href").map(|s| s.to_string());
            let src = element.value().attr("src").map(|s| s.to_string());
            let class = element.value().attr("class").map(|s| s.to_string());
            let id = element.value().attr("id").map(|s| s.to_string());

            elements.push(HtmlElement {
                text,
                href,
                src,
                class,
                id,
            });
        }

        Ok(elements)
    }
}

/// URL工具函数
pub struct UrlUtils;

impl UrlUtils {
    /// 解析相对URL
    pub fn resolve_url(base: &str, relative: &str) -> BookSourceResult<String> {
        let base_url = Url::parse(base)
            .map_err(|e| BookSourceError::ParseError(format!("基础URL解析失败: {}", e)))?;
        
        let resolved = base_url.join(relative)
            .map_err(|e| BookSourceError::ParseError(format!("URL解析失败: {}", e)))?;
        
        Ok(resolved.to_string())
    }

    /// URL编码
    pub fn encode_url(input: &str) -> String {
        urlencoding::encode(input).to_string()
    }
}