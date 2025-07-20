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

    /// 创建完整的测试环境
    async fn create_test_environment() -> (BookManager, BookSourceManager, TempDir) {
        let temp_dir = TempDir::new().expect("创建临时目录失败");
        let db_path = temp_dir.path().join("integration_test.db");
        let db = Arc::new(DatabaseManager::new(&db_path).expect("创建测试数据库失败"));
        
        let storage_path = temp_dir.path().join("storage");
        let plugin_dir = temp_dir.path().join("plugins");
        
        let book_manager = BookManager::new(db.clone(), storage_path);
        let book_source_manager = BookSourceManager::new(db.clone(), plugin_dir)
            .expect("创建书源管理器失败");
        
        (book_manager, book_source_manager, temp_dir)
    }

    /// 创建测试文件
    fn create_test_file(temp_dir: &TempDir, filename: &str, content: &str) -> PathBuf {
        let file_path = temp_dir.path().join(filename);
        std::fs::write(&file_path, content).expect("创建测试文件失败");
        file_path
    }

    /// 创建测试插件
    fn create_test_plugin(temp_dir: &TempDir, plugin_name: &str) -> PathBuf {
        let plugin_dir = temp_dir.path().join("plugins").join(plugin_name);
        std::fs::create_dir_all(&plugin_dir).expect("创建插件目录失败");

        // 创建元数据文件
        let metadata = serde_json::json!({
            "id": plugin_name,
            "name": format!("集成测试插件 {}", plugin_name),
            "version": "1.0.0",
            "author": "集成测试",
            "description": "用于集成测试的插件",
            "baseUrl": "https://test.example.com"
        });

        let metadata_path = plugin_dir.join("plugin.json");
        std::fs::write(&metadata_path, metadata.to_string()).expect("创建元数据文件失败");

        // 创建插件代码
        let js_content = r#"
function search(keyword) {
    return [
        {
            title: "测试小说 " + keyword,
            author: "测试作者",
            description: "这是一个测试小说",
            bookUrl: "https://test.example.com/book/" + keyword,
            coverUrl: "https://test.example.com/cover/" + keyword + ".jpg"
        }
    ];
}

function getChapters(bookUrl) {
    return [
        { title: "第一章", url: bookUrl + "/chapter/1", index: 0 },
        { title: "第二章", url: bookUrl + "/chapter/2", index: 1 },
        { title: "第三章", url: bookUrl + "/chapter/3", index: 2 }
    ];
}

function getChapterContent(chapterUrl) {
    return "这是章节内容：" + chapterUrl;
}
"#;

        let js_path = plugin_dir.join("index.js");
        std::fs::write(&js_path, js_content).expect("创建JS文件失败");

        js_path
    }

    #[tokio::test]
    async fn test_complete_book_management_workflow() {
        let (book_manager, _source_manager, temp_dir) = create_test_environment().await;
        
        // 1. 创建测试图书文件
        let book_content = r#"作者：测试作者

第一章 开始的故事
这是第一章的内容，讲述了故事的开始...

第二章 发展
故事继续发展，主人公遇到了挑战...

第三章 高潮
故事达到高潮，所有矛盾集中爆发...

第四章 结局
故事圆满结束，所有问题得到解决..."#;

        let book_file = create_test_file(&temp_dir, "complete_test_book.txt", book_content);

        // 2. 导入图书
        let book = book_manager.import_book(book_file).await.unwrap();
        assert_eq!(book.title, "complete_test_book");
        assert_eq!(book.author, Some("测试作者".to_string()));

        // 3. 验证图书已保存到数据库
        let all_books = book_manager.get_all_books().await.unwrap();
        assert_eq!(all_books.len(), 1);

        // 4. 获取图书内容
        let content = book_manager.get_book_content(&book.id).await.unwrap();
        assert!(content.contains("第一章 开始的故事"));

        // 5. 解析章节
        let chapters = book_manager.get_book_chapters(&book.id).await.unwrap();
        assert_eq!(chapters.len(), 4);
        assert_eq!(chapters[0].title, "第一章 开始的故事");
        assert_eq!(chapters[3].title, "第四章 结局");

        // 6. 获取章节内容
        let chapter_content = book_manager.get_chapter_content(&book.id, 0).await.unwrap();
        assert_eq!(chapter_content.title, "第一章 开始的故事");
        assert!(chapter_content.content.contains("这是第一章的内容"));

        // 7. 添加书签
        let bookmark_id = book_manager
            .add_bookmark(&book.id, 500, Some(1), Some("重要情节".to_string()))
            .await
            .unwrap();
        assert!(bookmark_id > 0);

        // 8. 获取书签列表
        let bookmarks = book_manager.get_bookmarks(&book.id).await.unwrap();
        assert_eq!(bookmarks.len(), 1);
        assert_eq!(bookmarks[0].note, Some("重要情节".to_string()));

        // 9. 更新阅读进度
        book_manager.save_reading_progress(&book.id, 0.6).await.unwrap();
        let progress = book_manager.get_reading_progress(&book.id).await.unwrap();
        assert_eq!(progress, 0.6);

        // 10. 搜索图书
        let search_results = book_manager.search_books("complete").await.unwrap();
        assert_eq!(search_results.len(), 1);
        assert_eq!(search_results[0].id, book.id);

        // 11. 获取分页内容
        let page_content = book_manager
            .get_paginated_content(&book.id, 100, 0)
            .await
            .unwrap();
        assert_eq!(page_content.page_number, 0);
        assert_eq!(page_content.page_size, 100);
        assert!(page_content.content.len() <= 100);

        // 12. 获取统计信息
        let stats = book_manager.get_library_stats().await.unwrap();
        assert_eq!(stats.total_books, 1);
        assert_eq!(stats.books_with_progress, 1);
        assert_eq!(stats.average_progress, 0.6);
    }

    #[tokio::test]
    async fn test_batch_import_workflow() {
        let (book_manager, _source_manager, temp_dir) = create_test_environment().await;
        
        // 创建多个测试文件
        let books_dir = temp_dir.path().join("books");
        std::fs::create_dir(&books_dir).expect("创建图书目录失败");

        // 创建子目录
        let fiction_dir = books_dir.join("fiction");
        std::fs::create_dir(&fiction_dir).expect("创建小说目录失败");

        // 创建不同类型的文件
        std::fs::write(books_dir.join("book1.txt"), "作者：作者1\n内容1").expect("创建文件失败");
        std::fs::write(books_dir.join("book2.txt"), "作者：作者2\n内容2").expect("创建文件失败");
        std::fs::write(fiction_dir.join("novel1.txt"), "作者：小说家1\n小说内容1").expect("创建文件失败");
        std::fs::write(fiction_dir.join("novel2.txt"), "作者：小说家2\n小说内容2").expect("创建文件失败");
        std::fs::write(books_dir.join("readme.md"), "这是说明文件").expect("创建文件失败"); // 不支持的格式

        // 批量导入
        let imported_books = book_manager.import_folder(books_dir).await.unwrap();
        assert_eq!(imported_books.len(), 4, "应该导入4本书（跳过不支持的格式）");

        // 验证所有图书都已导入
        let all_books = book_manager.get_all_books().await.unwrap();
        assert_eq!(all_books.len(), 4);

        // 验证不同作者的图书
        let authors: Vec<Option<String>> = all_books.iter().map(|b| b.author.clone()).collect();
        assert!(authors.contains(&Some("作者1".to_string())));
        assert!(authors.contains(&Some("作者2".to_string())));
        assert!(authors.contains(&Some("小说家1".to_string())));
        assert!(authors.contains(&Some("小说家2".to_string())));
    }

    #[tokio::test]
    async fn test_book_source_integration() {
        let (_book_manager, mut source_manager, temp_dir) = create_test_environment().await;
        
        // 创建测试插件
        let plugin1_path = create_test_plugin(&temp_dir, "source1");
        let plugin2_path = create_test_plugin(&temp_dir, "source2");

        // 加载插件
        let source1 = source_manager.load_source(plugin1_path).await.unwrap();
        let source2 = source_manager.load_source(plugin2_path).await.unwrap();

        // 验证插件已加载
        assert_eq!(source1.id, "source1");
        assert_eq!(source2.id, "source2");

        // 获取所有书源
        let sources = source_manager.get_sources().await.unwrap();
        assert_eq!(sources.len(), 2);

        // 禁用一个书源
        source_manager.toggle_source(&source1.id, false).await.unwrap();

        // 验证状态已更新
        let sources = source_manager.get_sources().await.unwrap();
        let disabled_source = sources.iter().find(|s| s.id == source1.id).unwrap();
        assert!(!disabled_source.enabled);

        // 卸载书源
        source_manager.unload_source(&source2.id).await.unwrap();

        // 验证书源已卸载
        let sources = source_manager.get_sources().await.unwrap();
        assert_eq!(sources.len(), 1); // 只剩下一个书源
    }

    #[tokio::test]
    async fn test_error_handling_integration() {
        let (book_manager, mut source_manager, temp_dir) = create_test_environment().await;
        
        // 测试导入不存在的文件
        let nonexistent_file = PathBuf::from("/nonexistent/file.txt");
        let result = book_manager.import_book(nonexistent_file).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("文件不存在"));

        // 测试导入不支持的格式
        let unsupported_file = create_test_file(&temp_dir, "test.xyz", "内容");
        let result = book_manager.import_book(unsupported_file).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("不支持的文件格式"));

        // 测试重复导入
        let test_file = create_test_file(&temp_dir, "duplicate.txt", "内容");
        let _book1 = book_manager.import_book(test_file.clone()).await.unwrap();
        let result = book_manager.import_book(test_file).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("图书已存在"));

        // 测试无效的阅读进度
        let book_file = create_test_file(&temp_dir, "progress_test.txt", "内容");
        let book = book_manager.import_book(book_file).await.unwrap();
        let result = book_manager.save_reading_progress(&book.id, 1.5).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("阅读进度必须在0.0到1.0之间"));

        // 测试操作不存在的图书
        let fake_id = "nonexistent-book-id";
        let result = book_manager.get_reading_progress(fake_id).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("图书不存在"));

        // 测试加载不存在的插件
        let nonexistent_plugin = PathBuf::from("/nonexistent/plugin.js");
        let result = source_manager.load_source(nonexistent_plugin).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("插件文件不存在"));
    }

    #[tokio::test]
    async fn test_concurrent_operations() {
        let (book_manager, _source_manager, temp_dir) = create_test_environment().await;
        
        // 创建多个测试文件
        let files: Vec<PathBuf> = (0..5)
            .map(|i| create_test_file(&temp_dir, &format!("concurrent_book_{}.txt", i), &format!("内容{}", i)))
            .collect();

        // 并发导入图书（使用Arc共享BookManager）
        let book_manager = Arc::new(book_manager);
        let mut handles = Vec::new();
        for file in files {
            let book_manager_clone = book_manager.clone();
            let handle = tokio::spawn(async move {
                book_manager_clone.import_book(file).await
            });
            handles.push(handle);
        }

        // 等待所有任务完成
        let mut successful_imports = 0;
        for handle in handles {
            if let Ok(Ok(_)) = handle.await {
                successful_imports += 1;
            }
        }

        // 验证所有图书都成功导入
        assert_eq!(successful_imports, 5, "所有5本书都应该成功导入");

        // 验证数据库中的图书数量
        let all_books = book_manager.get_all_books().await.unwrap();
        assert_eq!(all_books.len(), 5);
    }

    #[tokio::test]
    async fn test_data_consistency() {
        let (book_manager, _source_manager, temp_dir) = create_test_environment().await;
        
        // 导入图书
        let book_file = create_test_file(&temp_dir, "consistency_test.txt", "测试内容");
        let book = book_manager.import_book(book_file).await.unwrap();

        // 添加多个书签
        let bookmark1_id = book_manager
            .add_bookmark(&book.id, 100, Some(1), Some("书签1".to_string()))
            .await
            .unwrap();
        
        let bookmark2_id = book_manager
            .add_bookmark(&book.id, 200, Some(2), Some("书签2".to_string()))
            .await
            .unwrap();

        // 更新阅读进度
        book_manager.save_reading_progress(&book.id, 0.5).await.unwrap();

        // 验证数据一致性
        let retrieved_book = book_manager.get_all_books().await.unwrap();
        assert_eq!(retrieved_book.len(), 1);
        assert_eq!(retrieved_book[0].reading_progress, 0.5);

        let bookmarks = book_manager.get_bookmarks(&book.id).await.unwrap();
        assert_eq!(bookmarks.len(), 2);

        // 删除图书应该级联删除书签
        book_manager.delete_book(&book.id).await.unwrap();

        // 验证图书和书签都被删除
        let remaining_books = book_manager.get_all_books().await.unwrap();
        assert_eq!(remaining_books.len(), 0);

        let remaining_bookmarks = book_manager.get_bookmarks(&book.id).await.unwrap();
        assert_eq!(remaining_bookmarks.len(), 0);
    }

    #[tokio::test]
    async fn test_performance_with_large_content() {
        let (book_manager, _source_manager, temp_dir) = create_test_environment().await;
        
        // 创建大文件内容（模拟大型小说）
        let mut large_content = String::new();
        large_content.push_str("作者：性能测试作者\n\n");
        
        for i in 1..=100 {
            large_content.push_str(&format!("第{}章 章节标题{}\n", i, i));
            large_content.push_str(&format!("这是第{}章的内容。", i));
            large_content.push_str(&"内容".repeat(1000)); // 每章约1000个字符
            large_content.push_str("\n\n");
        }

        let large_file = create_test_file(&temp_dir, "large_book.txt", &large_content);

        // 测试导入大文件的性能
        let start_time = std::time::Instant::now();
        let book = book_manager.import_book(large_file).await.unwrap();
        let import_duration = start_time.elapsed();
        
        println!("导入大文件耗时: {:?}", import_duration);
        assert!(import_duration.as_secs() < 10, "导入大文件应该在10秒内完成");

        // 测试章节解析性能
        let start_time = std::time::Instant::now();
        let chapters = book_manager.get_book_chapters(&book.id).await.unwrap();
        let parse_duration = start_time.elapsed();
        
        println!("解析章节耗时: {:?}", parse_duration);
        assert_eq!(chapters.len(), 100, "应该解析出100个章节");
        assert!(parse_duration.as_secs() < 5, "章节解析应该在5秒内完成");

        // 测试分页内容获取性能
        let start_time = std::time::Instant::now();
        let page_content = book_manager
            .get_paginated_content(&book.id, 1000, 50)
            .await
            .unwrap();
        let pagination_duration = start_time.elapsed();
        
        println!("分页内容获取耗时: {:?}", pagination_duration);
        assert!(pagination_duration.as_millis() < 100, "分页内容获取应该在100ms内完成");
        assert_eq!(page_content.content.len(), 1000);
    }
}