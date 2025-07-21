#[cfg(test)]
mod tests {
    use crate::book_manager::BookManager;
    use crate::database::DatabaseManager;
    use crate::models::{Book, BookFormat};
    use std::path::PathBuf;
    use std::sync::Arc;
    use tempfile::TempDir;

    /// 创建测试用的数据库管理器
    async fn create_test_db() -> (Arc<DatabaseManager>, TempDir) {
        let temp_dir = TempDir::new().expect("创建临时目录失败");
        let db_path = temp_dir.path().join("test.db");
        let db = Arc::new(DatabaseManager::new(&db_path).expect("创建测试数据库失败"));
        (db, temp_dir)
    }

    /// 创建测试用的图书管理器
    async fn create_test_book_manager() -> (BookManager, TempDir) {
        let (db, temp_dir) = create_test_db().await;
        let storage_path = temp_dir.path().to_path_buf();
        let book_manager = BookManager::new(db, storage_path);
        (book_manager, temp_dir)
    }

    /// 创建测试用的TXT文件
    fn create_test_txt_file(temp_dir: &TempDir, filename: &str, content: &str) -> PathBuf {
        let file_path = temp_dir.path().join(filename);
        std::fs::write(&file_path, content).expect("创建测试文件失败");
        file_path
    }

    #[tokio::test]
    async fn test_import_txt_book_success() {
        let (book_manager, temp_dir) = create_test_book_manager().await;
        
        // 创建测试TXT文件
        let test_content = "作者：测试作者\n\n第一章 开始\n\n这是测试内容...";
        let file_path = create_test_txt_file(&temp_dir, "test_book.txt", test_content);

        // 导入图书
        let result = book_manager.import_book(file_path.clone()).await;
        assert!(result.is_ok(), "导入图书应该成功");

        let book = result.unwrap();
        assert_eq!(book.title, "test_book");
        assert_eq!(book.author, Some("测试作者".to_string()));
        assert_eq!(book.format, BookFormat::Txt);
        assert!(book.file_size > 0);
    }

    #[tokio::test]
    async fn test_import_nonexistent_file() {
        let (book_manager, _temp_dir) = create_test_book_manager().await;
        
        let nonexistent_path = PathBuf::from("/nonexistent/file.txt");
        let result = book_manager.import_book(nonexistent_path).await;
        
        assert!(result.is_err(), "导入不存在的文件应该失败");
        assert!(result.unwrap_err().to_string().contains("文件不存在"));
    }

    #[tokio::test]
    async fn test_import_unsupported_format() {
        let (book_manager, temp_dir) = create_test_book_manager().await;
        
        // 创建不支持的文件格式
        let file_path = temp_dir.path().join("test.xyz");
        std::fs::write(&file_path, "test content").expect("创建测试文件失败");

        let result = book_manager.import_book(file_path).await;
        assert!(result.is_err(), "导入不支持格式的文件应该失败");
        assert!(result.unwrap_err().to_string().contains("不支持的文件格式"));
    }

    #[tokio::test]
    async fn test_import_duplicate_book() {
        let (book_manager, temp_dir) = create_test_book_manager().await;
        
        let test_content = "测试内容";
        let file_path = create_test_txt_file(&temp_dir, "duplicate_test.txt", test_content);

        // 第一次导入应该成功
        let result1 = book_manager.import_book(file_path.clone()).await;
        assert!(result1.is_ok(), "第一次导入应该成功");

        // 第二次导入相同文件应该失败
        let result2 = book_manager.import_book(file_path).await;
        assert!(result2.is_err(), "导入重复文件应该失败");
        assert!(result2.unwrap_err().to_string().contains("图书已存在"));
    }

    #[tokio::test]
    async fn test_get_all_books() {
        let (book_manager, temp_dir) = create_test_book_manager().await;
        
        // 导入多本图书
        let file1 = create_test_txt_file(&temp_dir, "book1.txt", "内容1");
        let file2 = create_test_txt_file(&temp_dir, "book2.txt", "内容2");
        
        let _ = book_manager.import_book(file1).await.unwrap();
        let _ = book_manager.import_book(file2).await.unwrap();

        // 获取所有图书
        let books = book_manager.get_all_books().await.unwrap();
        assert_eq!(books.len(), 2, "应该有2本图书");
    }

    #[tokio::test]
    async fn test_search_books() {
        let (book_manager, temp_dir) = create_test_book_manager().await;
        
        // 创建测试图书
        let file1 = create_test_txt_file(&temp_dir, "小说1.txt", "作者：张三\n内容...");
        let file2 = create_test_txt_file(&temp_dir, "故事2.txt", "作者：李四\n内容...");
        
        let _ = book_manager.import_book(file1).await.unwrap();
        let _ = book_manager.import_book(file2).await.unwrap();

        // 按标题搜索
        let results = book_manager.search_books("小说").await.unwrap();
        assert_eq!(results.len(), 1, "搜索'小说'应该返回1本书");
        assert!(results[0].title.contains("小说"));

        // 按作者搜索
        let results = book_manager.search_books("张三").await.unwrap();
        assert_eq!(results.len(), 1, "搜索'张三'应该返回1本书");
        assert_eq!(results[0].author, Some("张三".to_string()));
    }

    #[tokio::test]
    async fn test_filter_books_by_format() {
        let (book_manager, temp_dir) = create_test_book_manager().await;
        
        // 导入TXT文件
        let txt_file = create_test_txt_file(&temp_dir, "test.txt", "内容");
        let _ = book_manager.import_book(txt_file).await.unwrap();

        // 按格式过滤
        let txt_books = book_manager.filter_books_by_format(&BookFormat::Txt).await.unwrap();
        assert_eq!(txt_books.len(), 1, "应该有1本TXT格式的书");

        let epub_books = book_manager.filter_books_by_format(&BookFormat::Epub).await.unwrap();
        assert_eq!(epub_books.len(), 0, "应该没有EPUB格式的书");
    }

    #[tokio::test]
    async fn test_delete_book() {
        let (book_manager, temp_dir) = create_test_book_manager().await;
        
        let file_path = create_test_txt_file(&temp_dir, "to_delete.txt", "内容");
        let book = book_manager.import_book(file_path).await.unwrap();

        // 删除图书
        let result = book_manager.delete_book(&book.id).await;
        assert!(result.is_ok(), "删除图书应该成功");

        // 验证图书已被删除
        let books = book_manager.get_all_books().await.unwrap();
        assert_eq!(books.len(), 0, "图书应该已被删除");
    }

    #[tokio::test]
    async fn test_reading_progress() {
        let (book_manager, temp_dir) = create_test_book_manager().await;
        
        let file_path = create_test_txt_file(&temp_dir, "progress_test.txt", "内容");
        let book = book_manager.import_book(file_path).await.unwrap();

        // 保存阅读进度
        let result = book_manager.save_reading_progress(&book.id, 0.5).await;
        assert!(result.is_ok(), "保存阅读进度应该成功");

        // 获取阅读进度
        let progress = book_manager.get_reading_progress(&book.id).await.unwrap();
        assert_eq!(progress, 0.5, "阅读进度应该是0.5");
    }

    #[tokio::test]
    async fn test_invalid_reading_progress() {
        let (book_manager, temp_dir) = create_test_book_manager().await;
        
        let file_path = create_test_txt_file(&temp_dir, "invalid_progress.txt", "内容");
        let book = book_manager.import_book(file_path).await.unwrap();

        // 测试无效的进度值
        let result = book_manager.save_reading_progress(&book.id, 1.5).await;
        assert!(result.is_err(), "无效的阅读进度应该失败");
        assert!(result.unwrap_err().to_string().contains("阅读进度必须在0.0到1.0之间"));

        let result = book_manager.save_reading_progress(&book.id, -0.1).await;
        assert!(result.is_err(), "负数阅读进度应该失败");
    }

    #[tokio::test]
    async fn test_bookmarks() {
        let (book_manager, temp_dir) = create_test_book_manager().await;
        
        let file_path = create_test_txt_file(&temp_dir, "bookmark_test.txt", "内容");
        let book = book_manager.import_book(file_path).await.unwrap();

        // 添加书签
        let bookmark_id = book_manager
            .add_bookmark(&book.id, 100, Some(1), Some("测试书签".to_string()))
            .await
            .unwrap();
        assert!(bookmark_id > 0, "书签ID应该大于0");

        // 获取书签列表
        let bookmarks = book_manager.get_bookmarks(&book.id).await.unwrap();
        assert_eq!(bookmarks.len(), 1, "应该有1个书签");
        assert_eq!(bookmarks[0].position, 100);
        assert_eq!(bookmarks[0].note, Some("测试书签".to_string()));

        // 删除书签
        let result = book_manager.delete_bookmark(bookmark_id).await;
        assert!(result.is_ok(), "删除书签应该成功");

        // 验证书签已被删除
        let bookmarks = book_manager.get_bookmarks(&book.id).await.unwrap();
        assert_eq!(bookmarks.len(), 0, "书签应该已被删除");
    }

    #[tokio::test]
    async fn test_get_library_stats() {
        let (book_manager, temp_dir) = create_test_book_manager().await;
        
        // 导入多本不同格式的图书
        let txt_file = create_test_txt_file(&temp_dir, "test.txt", "内容");
        let book = book_manager.import_book(txt_file).await.unwrap();
        
        // 设置阅读进度
        let _ = book_manager.save_reading_progress(&book.id, 0.3).await;

        // 获取统计信息
        let stats = book_manager.get_library_stats().await.unwrap();
        assert_eq!(stats.total_books, 1);
        assert_eq!(stats.books_with_progress, 1);
        assert_eq!(stats.average_progress, 0.3);
        assert!(stats.format_counts.contains_key(&BookFormat::Txt));
    }

    #[tokio::test]
    async fn test_import_folder() {
        let (book_manager, temp_dir) = create_test_book_manager().await;
        
        // 创建子目录和多个文件
        let sub_dir = temp_dir.path().join("books");
        std::fs::create_dir(&sub_dir).expect("创建子目录失败");
        
        let _ = create_test_txt_file(&temp_dir, "book1.txt", "内容1");
        let _ = std::fs::write(sub_dir.join("book2.txt"), "内容2");
        let _ = std::fs::write(sub_dir.join("readme.md"), "不支持的格式"); // 不支持的格式

        // 批量导入文件夹
        let result = book_manager.import_folder(temp_dir.path().to_path_buf()).await;
        assert!(result.is_ok(), "批量导入应该成功");

        let imported_books = result.unwrap();
        assert_eq!(imported_books.len(), 2, "应该导入2本书（跳过不支持的格式）");
    }

    #[tokio::test]
    async fn test_get_book_content() {
        let (book_manager, temp_dir) = create_test_book_manager().await;
        
        let test_content = "这是测试内容\n第二行\n第三行";
        let file_path = create_test_txt_file(&temp_dir, "content_test.txt", test_content);
        let book = book_manager.import_book(file_path).await.unwrap();

        // 获取图书内容
        let content = book_manager.get_book_content(&book.id).await.unwrap();
        assert_eq!(content, test_content);
    }

    #[tokio::test]
    async fn test_parse_txt_chapters() {
        let (book_manager, temp_dir) = create_test_book_manager().await;
        
        let test_content = r#"第一章 开始
这是第一章的内容...

第二章 发展
这是第二章的内容...

第三章 结束
这是第三章的内容..."#;
        
        let file_path = create_test_txt_file(&temp_dir, "chapters_test.txt", test_content);
        let book = book_manager.import_book(file_path).await.unwrap();

        // 获取章节列表
        let chapters = book_manager.get_book_chapters(&book.id).await.unwrap();
        assert_eq!(chapters.len(), 3, "应该解析出3个章节");
        assert_eq!(chapters[0].title, "第一章 开始");
        assert_eq!(chapters[1].title, "第二章 发展");
        assert_eq!(chapters[2].title, "第三章 结束");
    }

    #[tokio::test]
    async fn test_get_chapter_content() {
        let (book_manager, temp_dir) = create_test_book_manager().await;
        
        let test_content = r#"第一章 测试章节
这是第一章的详细内容，用于测试章节内容获取功能。
内容包含多行文本。

第二章 另一章节
这是第二章的内容。"#;
        
        let file_path = create_test_txt_file(&temp_dir, "chapter_content_test.txt", test_content);
        let book = book_manager.import_book(file_path).await.unwrap();

        // 获取第一章内容
        let chapter_content = book_manager.get_chapter_content(&book.id, 0).await.unwrap();
        assert_eq!(chapter_content.chapter_index, 0);
        assert_eq!(chapter_content.title, "第一章 测试章节");
        assert!(chapter_content.content.contains("这是第一章的详细内容"));
        assert!(chapter_content.word_count > 0);
        assert!(chapter_content.estimated_reading_time > 0);
    }

    #[tokio::test]
    async fn test_get_paginated_content() {
        let (book_manager, temp_dir) = create_test_book_manager().await;
        
        let test_content = "a".repeat(1000); // 1000个字符
        let file_path = create_test_txt_file(&temp_dir, "pagination_test.txt", &test_content);
        let book = book_manager.import_book(file_path).await.unwrap();

        // 获取分页内容（每页100字符）
        let page_content = book_manager
            .get_paginated_content(&book.id, 100, 0)
            .await
            .unwrap();
        
        assert_eq!(page_content.page_number, 0);
        assert_eq!(page_content.page_size, 100);
        assert_eq!(page_content.total_pages, 10); // 1000/100 = 10页
        assert_eq!(page_content.content.len(), 100);
        assert!(page_content.has_next_page);
        assert!(!page_content.has_previous_page);

        // 获取最后一页
        let last_page = book_manager
            .get_paginated_content(&book.id, 100, 9)
            .await
            .unwrap();
        
        assert!(!last_page.has_next_page);
        assert!(last_page.has_previous_page);
    }

    #[tokio::test]
    async fn test_extract_author_from_txt_content() {
        let (book_manager, temp_dir) = create_test_book_manager().await;
        
        // 测试不同的作者标识格式
        let test_cases = vec![
            ("作者：张三\n内容...", Some("张三".to_string())),
            ("作者:李四\n内容...", Some("李四".to_string())),
            ("Author: John Doe\n内容...", Some("John Doe".to_string())),
            ("王五著\n内容...", Some("王五著".to_string())),
            ("没有作者信息\n内容...", None),
        ];

        for (i, (content, expected_author)) in test_cases.iter().enumerate() {
            let filename = format!("author_test_{}.txt", i);
            let file_path = create_test_txt_file(&temp_dir, &filename, content);
            let book = book_manager.import_book(file_path).await.unwrap();
            
            assert_eq!(book.author, *expected_author, "测试用例 {} 失败", i);
        }
    }
}