#[cfg(test)]
mod tests {
    use crate::book_manager::BookManager;
    use crate::book_source_manager::BookSourceManager;
    use crate::database::DatabaseManager;
    use crate::models::BookFormat;
    use std::path::PathBuf;
    use std::sync::Arc;
    use tempfile::TempDir;
    use tokio;

    /// 创建测试环境
    async fn setup_test_env() -> (BookManager, BookSourceManager, TempDir) {
        // 创建临时目录
        let temp_dir = TempDir::new().expect("创建临时目录失败");
        let db_path = temp_dir.path().join("test.db");
        
        // 创建数据库管理器
        let db = Arc::new(DatabaseManager::new(&db_path).expect("创建测试数据库失败"));
        
        // 创建图书管理器
        let storage_path = temp_dir.path().join("books");
        std::fs::create_dir_all(&storage_path).expect("创建图书存储目录失败");
        let book_manager = BookManager::new(db.clone(), storage_path);
        
        // 创建书源管理器
        let plugin_dir = temp_dir.path().join("plugins");
        std::fs::create_dir_all(&plugin_dir).expect("创建插件目录失败");
        let source_manager = BookSourceManager::new(db.clone(), plugin_dir)
            .expect("创建书源管理器失败");
        
        (book_manager, source_manager, temp_dir)
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
    async fn test_end_to_end_book_import_and_read() {
        // 设置测试环境
        let (book_manager, _, temp_dir) = setup_test_env().await;
        
        // 创建测试文件
        let test_content = r#"作者：测试作者

第一章 开始
这是第一章的内容。

第二章 发展
这是第二章的内容。"#;
        
        let file_path = create_test_txt_file(&temp_dir, "test_book.txt", test_content);
        
        // 导入图书
        let book = book_manager.import_book(file_path).await.unwrap();
        
        // 验证图书信息
        assert_eq!(book.title, "test_book");
        assert_eq!(book.author, Some("测试作者".to_string()));
        
        // 获取章节列表
        let chapters = book_manager.get_book_chapters(&book.id).await.unwrap();
        assert_eq!(chapters.len(), 2);
        assert_eq!(chapters[0].title, "第一章 开始");
        assert_eq!(chapters[1].title, "第二章 发展");
        
        // 获取章节内容
        let chapter_content = book_manager.get_chapter_content(&book.id, 0).await.unwrap();
        assert_eq!(chapter_content.title, "第一章 开始");
        assert!(chapter_content.content.contains("这是第一章的内容"));
        
        // 添加书签
        let bookmark_id = book_manager
            .add_bookmark(&book.id, chapters[0].start_position as i64 + 5, Some(0), Some("测试书签".to_string()))
            .await
            .unwrap();
        
        // 获取书签
        let bookmarks = book_manager.get_bookmarks(&book.id).await.unwrap();
        assert_eq!(bookmarks.len(), 1);
        assert_eq!(bookmarks[0].id, bookmark_id);
        
        // 更新阅读进度
        let _ = book_manager.update_reading_progress(&book.id, 0.5).await.unwrap();
        
        // 获取更新后的图书
        let books = book_manager.get_all_books().await.unwrap();
        let updated_book = books.iter().find(|b| b.id == book.id).unwrap();
        assert_eq!(updated_book.reading_progress, 0.5);
        assert!(updated_book.last_read.is_some());
    }

    #[tokio::test]
    async fn test_end_to_end_book_source_workflow() {
        // 设置测试环境
        let (_, mut source_manager, temp_dir) = setup_test_env().await;
        
        // 创建测试插件
        let (_, js_path) = create_test_plugin(&temp_dir, "test-source", "测试书源");
        
        // 加载书源
        let source_info = source_manager.load_source(js_path).await.unwrap();
        assert_eq!(source_info.name, "测试书源");
        
        // 获取所有书源
        let sources = source_manager.get_sources().await.unwrap();
        assert_eq!(sources.len(), 1);
        
        // 禁用书源
        let _ = source_manager.toggle_source(&source_info.id, false).await.unwrap();
        
        // 验证书源已被禁用
        let updated_sources = source_manager.get_sources().await.unwrap();
        assert!(!updated_sources[0].enabled);
        
        // 重新启用书源
        let _ = source_manager.toggle_source(&source_info.id, true).await.unwrap();
        
        // 验证书源已被启用
        let final_sources = source_manager.get_sources().await.unwrap();
        assert!(final_sources[0].enabled);
    }

    #[tokio::test]
    async fn test_library_statistics() {
        // 设置测试环境
        let (book_manager, _, temp_dir) = setup_test_env().await;
        
        // 创建并导入多本图书
        let file1 = create_test_txt_file(&temp_dir, "book1.txt", "内容1");
        let file2 = create_test_txt_file(&temp_dir, "book2.txt", "内容2");
        
        let book1 = book_manager.import_book(file1).await.unwrap();
        let book2 = book_manager.import_book(file2).await.unwrap();
        
        // 设置不同的阅读进度
        let _ = book_manager.update_reading_progress(&book1.id, 0.3).await;
        let _ = book_manager.update_reading_progress(&book2.id, 0.7).await;
        
        // 获取图书库统计信息
        let stats = book_manager.get_library_stats().await.unwrap();
        
        // 验证统计信息
        assert_eq!(stats.total_books, 2);
        assert_eq!(stats.books_with_progress, 2);
        assert_eq!(stats.average_progress, 0.5); // (0.3 + 0.7) / 2
        assert!(stats.format_counts.contains_key(&BookFormat::Txt));
        assert_eq!(*stats.format_counts.get(&BookFormat::Txt).unwrap(), 2);
    }

    #[tokio::test]
    async fn test_batch_operations() {
        // 设置测试环境
        let (book_manager, _, temp_dir) = setup_test_env().await;
        
        // 创建测试目录结构
        let books_dir = temp_dir.path().join("books_import");
        std::fs::create_dir_all(&books_dir).expect("创建导入目录失败");
        
        // 创建多个测试文件
        let _ = std::fs::write(books_dir.join("book1.txt"), "内容1");
        let _ = std::fs::write(books_dir.join("book2.txt"), "内容2");
        let _ = std::fs::write(books_dir.join("book3.txt"), "内容3");
        let _ = std::fs::write(books_dir.join("readme.md"), "说明文件"); // 不支持的格式
        
        // 批量导入
        let result = book_manager.import_folder(books_dir).await;
        assert!(result.is_ok());
        
        let imported_books = result.unwrap();
        assert_eq!(imported_books.len(), 3); // 应该导入3本TXT图书
        
        // 获取所有图书
        let all_books = book_manager.get_all_books().await.unwrap();
        assert_eq!(all_books.len(), 3);
        
        // 获取最近添加的图书
        let recent_books = book_manager.get_recent_books(2).await.unwrap();
        assert_eq!(recent_books.len(), 2); // 限制为最近2本
    }

    #[tokio::test]
    async fn test_error_handling() {
        // 设置测试环境
        let (book_manager, _, _) = setup_test_env().await;
        
        // 测试不存在的图书ID
        let nonexistent_id = "nonexistent-id";
        
        // 尝试获取不存在的图书的章节
        let result = book_manager.get_book_chapters(nonexistent_id).await;
        assert!(result.is_err());
        
        // 尝试更新不存在的图书的阅读进度
        let result = book_manager.update_reading_progress(nonexistent_id, 0.5).await;
        assert!(result.is_err());
        
        // 尝试添加书签到不存在的图书
        let result = book_manager
            .add_bookmark(nonexistent_id, 100, None, None)
            .await;
        assert!(result.is_err());
    }
}