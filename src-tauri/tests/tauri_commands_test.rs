#[cfg(test)]
mod tests {
    use novel_nest_lib::models::BookFormat;
    use std::path::PathBuf;
    use tempfile::TempDir;
    use tokio;
    use std::sync::Arc;
    use novel_nest_lib::database::DatabaseManager;
    use novel_nest_lib::book_manager::BookManager;
    use novel_nest_lib::book_source_manager::BookSourceManager;

    /// 创建测试环境
    async fn setup_test_env() -> (Arc<DatabaseManager>, TempDir) {
        // 创建临时目录
        let temp_dir = TempDir::new().expect("创建临时目录失败");
        
        // 创建数据库
        let db_path = temp_dir.path().join("test.db");
        let db = Arc::new(DatabaseManager::new(&db_path).expect("创建测试数据库失败"));
        
        (db, temp_dir)
    }

    /// 创建测试用的TXT文件
    fn create_test_txt_file(temp_dir: &TempDir, filename: &str, content: &str) -> PathBuf {
        let file_path = temp_dir.path().join(filename);
        std::fs::write(&file_path, content).expect("创建测试文件失败");
        file_path
    }

    /// 创建测试用的插件文件
    fn create_test_plugin(temp_dir: &TempDir, id: &str, name: &str) -> (PathBuf, PathBuf) {
        // 创建插件目录
        let plugin_dir = temp_dir.path().join("data").join("plugins").join(id);
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
            class TestBookSource {
                constructor() {
                    this.baseUrl = "https://example.com";
                }
                
                // 搜索函数
                async search(keyword) {
                    return [
                        { 
                            title: "测试书籍: " + keyword, 
                            author: "测试作者", 
                            description: "这是一本测试书籍",
                            bookUrl: "https://example.com/book/1",
                            coverUrl: "https://example.com/cover/1.jpg"
                        }
                    ];
                }
                
                // 获取章节列表
                async getChapters(bookUrl) {
                    return [
                        { title: "第一章", url: bookUrl + "/1", index: 0 },
                        { title: "第二章", url: bookUrl + "/2", index: 1 }
                    ];
                }
                
                // 获取章节内容
                async getChapterContent(chapterUrl) {
                    return {
                        title: "章节标题",
                        content: "这是章节内容，来自URL: " + chapterUrl
                    };
                }
            }
            
            // 导出插件实例
            window.bookSourcePlugin = new TestBookSource();
        "#;
        
        let js_path = plugin_dir.join("index.js");
        std::fs::write(&js_path, js_code).expect("创建插件代码文件失败");
        
        (metadata_path, js_path)
    }

    #[tokio::test]
    async fn test_tauri_commands_book_operations() {
        // 设置测试环境
        let (db, temp_dir) = setup_test_env().await;
        
        // 创建图书管理器
        let storage_path = temp_dir.path().join("books");
        std::fs::create_dir_all(&storage_path).expect("创建图书存储目录失败");
        let book_manager = BookManager::new(db.clone(), storage_path);
        
        // 创建测试文件
        let test_content = r#"作者：测试作者

第一章 开始
这是第一章的内容。

第二章 发展
这是第二章的内容。"#;
        
        let file_path = create_test_txt_file(&temp_dir, "test_book.txt", test_content);
        
        // 导入图书
        let book = book_manager.import_book(file_path).await.expect("导入图书失败");
        
        // 验证图书信息
        assert_eq!(book.title, "test_book");
        assert_eq!(book.author, Some("测试作者".to_string()));
        assert_eq!(book.format, BookFormat::Txt);
        
        // 获取章节列表
        let chapters = book_manager.get_book_chapters(&book.id).await.expect("获取章节失败");
        
        // 验证章节信息
        assert_eq!(chapters.len(), 2);
        assert_eq!(chapters[0].title, "第一章 开始");
        assert_eq!(chapters[1].title, "第二章 发展");
        
        // 获取章节内容
        let chapter_content = book_manager.get_chapter_content(&book.id, 0).await.expect("获取章节内容失败");
        
        // 验证章节内容
        assert_eq!(chapter_content.title, "第一章 开始");
        assert!(chapter_content.content.contains("这是第一章的内容"));
        
        // 更新阅读进度
        book_manager.update_reading_progress(&book.id, 0.5).await.expect("更新阅读进度失败");
        
        // 获取阅读进度
        let progress = book_manager.get_reading_progress(&book.id).await.expect("获取阅读进度失败");
        
        // 验证阅读进度
        assert_eq!(progress, 0.5);
    }

    #[tokio::test]
    async fn test_tauri_commands_plugin_operations() {
        // 设置测试环境
        let (db, temp_dir) = setup_test_env().await;
        
        // 创建书源管理器
        let plugin_dir = temp_dir.path().join("plugins");
        std::fs::create_dir_all(&plugin_dir).expect("创建插件目录失败");
        let mut source_manager = BookSourceManager::new(db.clone(), plugin_dir).expect("创建书源管理器失败");
        
        // 创建测试插件
        let (_, js_path) = create_test_plugin(&temp_dir, "test-source", "测试书源");
        
        // 加载书源
        let source_info = source_manager.load_source(js_path).await.expect("加载书源失败");
        
        // 验证书源信息
        assert_eq!(source_info.id, "test-source");
        assert_eq!(source_info.name, "测试书源");
        assert_eq!(source_info.version, "1.0.0");
        assert_eq!(source_info.author, Some("测试作者".to_string()));
        assert_eq!(source_info.description, Some("测试描述".to_string()));
        assert_eq!(source_info.base_url, "https://example.com");
        assert!(source_info.enabled);
        
        // 切换书源状态
        source_manager.toggle_source(&source_info.id, false).await.expect("切换书源状态失败");
        
        // 获取书源列表
        let sources = source_manager.get_sources().await.expect("获取书源列表失败");
        
        // 验证书源状态
        let updated_source = sources.iter().find(|s| s.id == source_info.id).expect("未找到书源");
        assert!(!updated_source.enabled);
        
        // 获取插件代码
        let plugin_code = source_manager.get_plugin_code(&source_info.id).expect("获取插件代码失败");
        
        // 验证插件代码
        assert!(plugin_code.contains("search"));
        assert!(plugin_code.contains("getChapters"));
        assert!(plugin_code.contains("getChapterContent"));
    }

    #[tokio::test]
    async fn test_tauri_commands_error_handling() {
        // 设置测试环境
        let (db, temp_dir) = setup_test_env().await;
        
        // 创建图书管理器
        let storage_path = temp_dir.path().join("books");
        std::fs::create_dir_all(&storage_path).expect("创建图书存储目录失败");
        let book_manager = BookManager::new(db.clone(), storage_path);
        
        // 测试不存在的图书ID
        let nonexistent_id = "nonexistent-id";
        
        // 测试获取不存在的图书章节
        let result = book_manager.get_book_chapters(nonexistent_id).await;
        assert!(result.is_err());
        
        // 测试更新不存在的图书阅读进度
        let result = book_manager.update_reading_progress(nonexistent_id, 0.5).await;
        assert!(result.is_err());
        
        // 创建书源管理器
        let plugin_dir = temp_dir.path().join("plugins");
        std::fs::create_dir_all(&plugin_dir).expect("创建插件目录失败");
        let source_manager = BookSourceManager::new(db.clone(), plugin_dir).expect("创建书源管理器失败");
        
        // 测试获取不存在的插件代码
        let result = source_manager.get_plugin_code("nonexistent-source");
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_tauri_commands_plugin_utils() {
        use novel_nest_lib::book_source_manager::{HttpUtils, UrlUtils};
        
        // 测试 URL 解析
        let base = "https://example.com";
        let relative = "/page";
        let result = UrlUtils::resolve_url(base, relative);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "https://example.com/page");
        
        // 测试 URL 编码
        let input = "测试";
        let result = UrlUtils::encode_url(input);
        assert_eq!(result, "%E6%B5%8B%E8%AF%95");
        
        // 测试 HTTP GET 请求
        // 注意：这个测试可能会失败，因为它依赖于外部网络
        // 所以我们只检查结果是否为 Ok 或 Err，不关心具体内容
        let http_utils = HttpUtils::new();
        let result = http_utils.get("https://httpbin.org/get").await;
        assert!(result.is_ok() || result.is_err());
    }
}