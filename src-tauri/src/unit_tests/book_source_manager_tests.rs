#[cfg(test)]
mod tests {
    use crate::book_source_manager::{BookSourceManager, HtmlParser, UrlUtils};
    use crate::database::DatabaseManager;
    use std::path::PathBuf;
    use std::sync::Arc;
    use tempfile::TempDir;
    use tokio;

    /// 创建测试用的数据库管理器
    async fn create_test_db() -> (Arc<DatabaseManager>, TempDir) {
        let temp_dir = TempDir::new().expect("创建临时目录失败");
        let db_path = temp_dir.path().join("test.db");
        let db = Arc::new(DatabaseManager::new(&db_path).expect("创建测试数据库失败"));
        (db, temp_dir)
    }

    /// 创建测试用的书源管理器
    async fn create_test_source_manager() -> (BookSourceManager, TempDir) {
        let (db, temp_dir) = create_test_db().await;
        let plugin_dir = temp_dir.path().join("plugins");
        std::fs::create_dir(&plugin_dir).expect("创建插件目录失败");
        
        let source_manager = BookSourceManager::new(db, plugin_dir)
            .expect("创建书源管理器失败");
        
        (source_manager, temp_dir)
    }

    /// 创建测试用的插件文件
    fn create_test_plugin(temp_dir: &TempDir, id: &str, name: &str) -> (PathBuf, PathBuf) {
        // 创建插件目录
        let plugin_dir = temp_dir.path().join("plugins").join(id);
        std::fs::create_dir_all(&plugin_dir).expect("创建插件目录失败");
        
        // 创建插件元数据文件
        let metadata_content = format!(
            r#"{{
                "id": "{}",
                "name": "{}",
                "version": "1.0.0",
                "author": "测试作者",
                "description": "测试描述",
                "baseUrl": "https://example.com"
            }}"#,
            id, name
        );
        let metadata_path = plugin_dir.join("plugin.json");
        std::fs::write(&metadata_path, metadata_content).expect("创建元数据文件失败");
        
        // 创建插件代码文件
        let js_code = r#"
            // 搜索函数
            function search(keyword) {
                return [
                    { title: "测试书籍", author: "测试作者", url: "/book/1" }
                ];
            }
            
            // 获取章节列表
            function getChapters(bookUrl) {
                return [
                    { title: "第一章", url: "/book/1/1" },
                    { title: "第二章", url: "/book/1/2" }
                ];
            }
            
            // 获取章节内容
            function getChapterContent(chapterUrl) {
                return "这是章节内容";
            }
        "#;
        
        let js_path = plugin_dir.join("index.js");
        std::fs::write(&js_path, js_code).expect("创建插件代码文件失败");
        
        (metadata_path, js_path)
    }

    #[tokio::test]
    async fn test_load_source() {
        let (mut source_manager, temp_dir) = create_test_source_manager().await;
        
        // 创建测试插件
        let (_, js_path) = create_test_plugin(&temp_dir, "test-source", "测试书源");
        
        // 加载书源
        let result = source_manager.load_source(js_path).await;
        assert!(result.is_ok(), "加载书源应该成功");
        
        let source_info = result.unwrap();
        assert_eq!(source_info.id, "test-source");
        assert_eq!(source_info.name, "测试书源");
        assert_eq!(source_info.version, "1.0.0");
        assert_eq!(source_info.author, Some("测试作者".to_string()));
        assert!(source_info.enabled);
    }

    #[tokio::test]
    async fn test_load_nonexistent_source() {
        let (mut source_manager, _temp_dir) = create_test_source_manager().await;
        
        // 尝试加载不存在的插件
        let nonexistent_path = PathBuf::from("/nonexistent/plugin.js");
        let result = source_manager.load_source(nonexistent_path).await;
        
        assert!(result.is_err(), "加载不存在的插件应该失败");
        assert!(result.unwrap_err().to_string().contains("插件文件不存在"));
    }

    #[tokio::test]
    async fn test_get_sources() {
        let (mut source_manager, temp_dir) = create_test_source_manager().await;
        
        // 创建并加载多个测试插件
        let (_, js_path1) = create_test_plugin(&temp_dir, "source1", "书源1");
        let (_, js_path2) = create_test_plugin(&temp_dir, "source2", "书源2");
        
        let _ = source_manager.load_source(js_path1).await.unwrap();
        let _ = source_manager.load_source(js_path2).await.unwrap();
        
        // 获取所有书源
        let sources = source_manager.get_sources().await.unwrap();
        assert_eq!(sources.len(), 2, "应该有2个书源");
        
        // 验证书源信息
        let source_ids: Vec<String> = sources.iter().map(|s| s.id.clone()).collect();
        assert!(source_ids.contains(&"source1".to_string()));
        assert!(source_ids.contains(&"source2".to_string()));
    }

    #[tokio::test]
    async fn test_toggle_source() {
        let (mut source_manager, temp_dir) = create_test_source_manager().await;
        
        // 创建并加载测试插件
        let (_, js_path) = create_test_plugin(&temp_dir, "toggle-test", "切换测试");
        let source_info = source_manager.load_source(js_path).await.unwrap();
        
        // 初始状态应该是启用的
        assert!(source_info.enabled);
        
        // 禁用书源
        let result = source_manager.toggle_source("toggle-test", false).await;
        assert!(result.is_ok(), "禁用书源应该成功");
        
        // 验证书源已被禁用
        let sources = source_manager.get_sources().await.unwrap();
        let toggle_source = sources.iter().find(|s| s.id == "toggle-test").unwrap();
        assert!(!toggle_source.enabled, "书源应该被禁用");
        
        // 重新启用书源
        let result = source_manager.toggle_source("toggle-test", true).await;
        assert!(result.is_ok(), "启用书源应该成功");
        
        // 验证书源已被启用
        let sources = source_manager.get_sources().await.unwrap();
        let toggle_source = sources.iter().find(|s| s.id == "toggle-test").unwrap();
        assert!(toggle_source.enabled, "书源应该被启用");
    }

    #[tokio::test]
    async fn test_unload_source() {
        let (mut source_manager, temp_dir) = create_test_source_manager().await;
        
        // 创建并加载测试插件
        let (_, js_path) = create_test_plugin(&temp_dir, "unload-test", "卸载测试");
        let _ = source_manager.load_source(js_path).await.unwrap();
        
        // 卸载书源
        let result = source_manager.unload_source("unload-test").await;
        assert!(result.is_ok(), "卸载书源应该成功");
        
        // 验证书源已被卸载
        let sources = source_manager.get_sources().await.unwrap();
        let unloaded = sources.iter().find(|s| s.id == "unload-test");
        assert!(unloaded.is_none(), "书源应该已被卸载");
    }

    #[tokio::test]
    async fn test_get_plugin_code() {
        let (mut source_manager, temp_dir) = create_test_source_manager().await;
        
        // 创建并加载测试插件
        let (_, js_path) = create_test_plugin(&temp_dir, "code-test", "代码测试");
        let _ = source_manager.load_source(js_path).await.unwrap();
        
        // 获取插件代码
        let code = source_manager.get_plugin_code("code-test");
        assert!(code.is_some(), "应该能获取到插件代码");
        assert!(code.unwrap().contains("function search"));
    }

    #[tokio::test]
    async fn test_initialize() {
        let (mut source_manager, temp_dir) = create_test_source_manager().await;
        
        // 创建并加载多个测试插件
        let (_, js_path1) = create_test_plugin(&temp_dir, "init-test1", "初始化测试1");
        let (_, js_path2) = create_test_plugin(&temp_dir, "init-test2", "初始化测试2");
        
        let _ = source_manager.load_source(js_path1).await.unwrap();
        let _ = source_manager.load_source(js_path2).await.unwrap();
        
        // 禁用其中一个书源
        let _ = source_manager.toggle_source("init-test2", false).await;
        
        // 创建新的书源管理器（模拟重启）
        let plugin_dir = temp_dir.path().join("plugins");
        let db_path = temp_dir.path().join("test.db");
        let db = Arc::new(DatabaseManager::new(&db_path).expect("创建测试数据库失败"));
        let mut new_manager = BookSourceManager::new(db, plugin_dir)
            .expect("重新创建书源管理器失败");
        
        // 初始化（应该只加载启用的插件）
        let result = new_manager.initialize().await;
        assert!(result.is_ok(), "初始化应该成功");
        
        // 验证只有启用的插件被加载
        assert!(new_manager.get_plugin_code("init-test1").is_some());
        assert!(new_manager.get_plugin_code("init-test2").is_none());
    }

    #[test]
    fn test_html_parser() {
        let html = r#"
        <html>
            <body>
                <div class="book-list">
                    <div class="book-item">
                        <a href="/book/1" class="title">书籍1</a>
                        <span class="author">作者1</span>
                    </div>
                    <div class="book-item">
                        <a href="/book/2" class="title">书籍2</a>
                        <span class="author">作者2</span>
                    </div>
                </div>
            </body>
        </html>
        "#;
        
        // 解析标题元素
        let title_elements = HtmlParser::parse_elements(html, "a.title").unwrap();
        assert_eq!(title_elements.len(), 2);
        assert_eq!(title_elements[0].text, "书籍1");
        assert_eq!(title_elements[0].href, Some("/book/1".to_string()));
        assert_eq!(title_elements[1].text, "书籍2");
        
        // 解析作者元素
        let author_elements = HtmlParser::parse_elements(html, "span.author").unwrap();
        assert_eq!(author_elements.len(), 2);
        assert_eq!(author_elements[0].text, "作者1");
        assert_eq!(author_elements[1].text, "作者2");
    }

    #[test]
    fn test_url_utils() {
        // 测试URL解析
        let base_url = "https://example.com/books/";
        let relative_url = "novel/123";
        let resolved = UrlUtils::resolve_url(base_url, relative_url).unwrap();
        assert_eq!(resolved, "https://example.com/books/novel/123");
        
        // 测试URL编码
        let input = "小说 测试";
        let encoded = UrlUtils::encode_url(input);
        assert_eq!(encoded, "%E5%B0%8F%E8%AF%B4%20%E6%B5%8B%E8%AF%95");
    }
}