#[cfg(test)]
mod tests {
    use crate::book_source_manager::{BookSourceManager, HttpUtils, HtmlParser, UrlUtils};
    use crate::database::DatabaseManager;
    use crate::models::{BookSourceInfo, HtmlElement};
    use std::path::PathBuf;
    use std::sync::Arc;
    use tempfile::TempDir;
    use tokio;

    /// 创建测试用的数据库管理器
    async fn create_test_db() -> (DatabaseManager, TempDir) {
        let temp_dir = TempDir::new().expect("创建临时目录失败");
        let db_path = temp_dir.path().join("test.db");
        let db = DatabaseManager::new(&db_path).expect("创建测试数据库失败");
        (db, temp_dir)
    }

    /// 创建测试用的书源管理器
    async fn create_test_source_manager() -> (BookSourceManager, TempDir) {
        let (db, temp_dir) = create_test_db().await;
        let plugin_dir = temp_dir.path().join("plugins");
        let source_manager = BookSourceManager::new(Arc::new(db), plugin_dir)
            .expect("创建书源管理器失败");
        (source_manager, temp_dir)
    }

    /// 创建测试插件文件
    fn create_test_plugin(temp_dir: &TempDir, plugin_name: &str) -> PathBuf {
        let plugin_dir = temp_dir.path().join("plugins").join(plugin_name);
        std::fs::create_dir_all(&plugin_dir).expect("创建插件目录失败");

        // 创建 plugin.json 元数据文件
        let metadata = serde_json::json!({
            "id": plugin_name,
            "name": format!("测试插件 {}", plugin_name),
            "version": "1.0.0",
            "author": "测试作者",
            "description": "这是一个测试插件",
            "baseUrl": "https://example.com"
        });

        let metadata_path = plugin_dir.join("plugin.json");
        std::fs::write(&metadata_path, metadata.to_string()).expect("创建元数据文件失败");

        // 创建 JavaScript 插件文件
        let js_content = r#"
// 测试插件代码
function search(keyword) {
    return [];
}

function getChapters(bookUrl) {
    return [];
}

function getChapterContent(chapterUrl) {
    return "";
}
"#;

        let js_path = plugin_dir.join("index.js");
        std::fs::write(&js_path, js_content).expect("创建JS文件失败");

        js_path
    }

    #[tokio::test]
    async fn test_load_source_success() {
        let (mut source_manager, temp_dir) = create_test_source_manager().await;
        
        let plugin_path = create_test_plugin(&temp_dir, "test_plugin");
        
        // 加载插件
        let result = source_manager.load_source(plugin_path).await;
        assert!(result.is_ok(), "加载插件应该成功");

        let plugin_info = result.unwrap();
        assert_eq!(plugin_info.id, "test_plugin");
        assert_eq!(plugin_info.name, "测试插件 test_plugin");
        assert_eq!(plugin_info.version, "1.0.0");
        assert_eq!(plugin_info.author, Some("测试作者".to_string()));
        assert!(plugin_info.enabled);
    }

    #[tokio::test]
    async fn test_load_nonexistent_plugin() {
        let (mut source_manager, _temp_dir) = create_test_source_manager().await;
        
        let nonexistent_path = PathBuf::from("/nonexistent/plugin.js");
        let result = source_manager.load_source(nonexistent_path).await;
        
        assert!(result.is_err(), "加载不存在的插件应该失败");
        assert!(result.unwrap_err().to_string().contains("插件文件不存在"));
    }

    #[tokio::test]
    async fn test_unload_source() {
        let (mut source_manager, temp_dir) = create_test_source_manager().await;
        
        let plugin_path = create_test_plugin(&temp_dir, "unload_test");
        let plugin_info = source_manager.load_source(plugin_path).await.unwrap();

        // 卸载插件
        let result = source_manager.unload_source(&plugin_info.id).await;
        assert!(result.is_ok(), "卸载插件应该成功");

        // 验证插件代码已从内存中移除
        assert!(source_manager.get_plugin_code(&plugin_info.id).is_none());
    }

    #[tokio::test]
    async fn test_get_sources() {
        let (mut source_manager, temp_dir) = create_test_source_manager().await;
        
        // 加载多个插件
        let _plugin1 = source_manager.load_source(create_test_plugin(&temp_dir, "plugin1")).await.unwrap();
        let _plugin2 = source_manager.load_source(create_test_plugin(&temp_dir, "plugin2")).await.unwrap();

        // 获取所有书源
        let sources = source_manager.get_sources().await.unwrap();
        assert_eq!(sources.len(), 2, "应该有2个书源");
    }

    #[tokio::test]
    async fn test_toggle_source() {
        let (mut source_manager, temp_dir) = create_test_source_manager().await;
        
        let plugin_path = create_test_plugin(&temp_dir, "toggle_test");
        let plugin_info = source_manager.load_source(plugin_path).await.unwrap();

        // 禁用书源
        let result = source_manager.toggle_source(&plugin_info.id, false).await;
        assert!(result.is_ok(), "禁用书源应该成功");

        // 重新启用书源
        let result = source_manager.toggle_source(&plugin_info.id, true).await;
        assert!(result.is_ok(), "启用书源应该成功");
    }

    // HTML解析工具测试
    #[test]
    fn test_html_parser_parse_elements() {
        let html = r#"
        <div class="book-list">
            <a href="/book/1" class="book-link">书籍1</a>
            <a href="/book/2" class="book-link">书籍2</a>
            <img src="/cover/1.jpg" alt="封面1" />
        </div>
        "#;

        // 解析链接元素
        let links = HtmlParser::parse_elements(html, "a.book-link").unwrap();
        assert_eq!(links.len(), 2, "应该解析出2个链接");
        assert_eq!(links[0].text, "书籍1");
        assert_eq!(links[0].href, Some("/book/1".to_string()));
        assert_eq!(links[1].text, "书籍2");
        assert_eq!(links[1].href, Some("/book/2".to_string()));

        // 解析图片元素
        let images = HtmlParser::parse_elements(html, "img").unwrap();
        assert_eq!(images.len(), 1, "应该解析出1个图片");
        assert_eq!(images[0].src, Some("/cover/1.jpg".to_string()));
    }

    // URL工具测试
    #[test]
    fn test_url_utils_resolve_url() {
        let base = "https://example.com/books/";
        let relative = "chapter1.html";
        
        let resolved = UrlUtils::resolve_url(base, relative).unwrap();
        assert_eq!(resolved, "https://example.com/books/chapter1.html");
    }

    #[test]
    fn test_url_utils_encode_url() {
        let input = "搜索关键词 with spaces";
        let encoded = UrlUtils::encode_url(input);
        assert!(encoded.contains("%"), "编码后的URL应该包含%符号");
        assert!(!encoded.contains(" "), "编码后的URL不应该包含空格");
    }
}