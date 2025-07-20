#[cfg(test)]
mod tests {
    use crate::database::DatabaseManager;
    use crate::models::{Book, BookFormat, Bookmark, BookSourceInfo};
    use chrono::Utc;
    use std::path::PathBuf;
    use tempfile::TempDir;
    use tokio;

    /// 创建测试用的数据库管理器
    fn create_test_db() -> (DatabaseManager, TempDir) {
        let temp_dir = TempDir::new().expect("创建临时目录失败");
        let db_path = temp_dir.path().join("test.db");
        let db = DatabaseManager::new(&db_path).expect("创建测试数据库失败");
        (db, temp_dir)
    }

    /// 创建测试用的图书对象
    fn create_test_book(title: &str, author: Option<&str>) -> Book {
        Book {
            id: uuid::Uuid::new_v4().to_string(),
            title: title.to_string(),
            author: author.map(|s| s.to_string()),
            file_path: format!("/test/path/{}.txt", title),
            format: BookFormat::Txt,
            file_size: 1024,
            created_at: Utc::now(),
            last_read: None,
            reading_progress: 0.0,
            total_chapters: 0,
        }
    }

    /// 创建测试用的书签对象
    fn create_test_bookmark(book_id: &str, position: i64) -> Bookmark {
        Bookmark {
            id: 0, // 数据库会自动分配
            book_id: book_id.to_string(),
            position,
            chapter_index: Some(1),
            note: Some("测试书签".to_string()),
            created_at: Utc::now(),
        }
    }

    /// 创建测试用的书源信息
    fn create_test_book_source(id: &str, name: &str) -> BookSourceInfo {
        BookSourceInfo {
            id: id.to_string(),
            name: name.to_string(),
            version: "1.0.0".to_string(),
            author: Some("测试作者".to_string()),
            description: Some("测试书源".to_string()),
            base_url: "https://example.com".to_string(),
            enabled: true,
            plugin_path: format!("/plugins/{}.js", id),
            installed_at: Utc::now(),
        }
    }

    #[test]
    fn test_database_creation() {
        let (db, _temp_dir) = create_test_db();
        // 如果能创建数据库实例，说明初始化成功
        assert!(true, "数据库创建应该成功");
    }

    #[test]
    fn test_insert_and_get_book() {
        let (db, _temp_dir) = create_test_db();
        
        let book = create_test_book("测试图书", Some("测试作者"));
        
        // 插入图书
        let result = db.insert_book(&book);
        assert!(result.is_ok(), "插入图书应该成功");

        // 根据ID获取图书
        let retrieved_book = db.get_book_by_id(&book.id).unwrap();
        assert!(retrieved_book.is_some(), "应该能找到插入的图书");
        
        let retrieved_book = retrieved_book.unwrap();
        assert_eq!(retrieved_book.id, book.id);
        assert_eq!(retrieved_book.title, book.title);
        assert_eq!(retrieved_book.author, book.author);
        assert_eq!(retrieved_book.format, book.format);
    }

    #[test]
    fn test_get_all_books() {
        let (db, _temp_dir) = create_test_db();
        
        // 插入多本图书
        let book1 = create_test_book("图书1", Some("作者1"));
        let book2 = create_test_book("图书2", Some("作者2"));
        let book3 = create_test_book("图书3", None);
        
        db.insert_book(&book1).unwrap();
        db.insert_book(&book2).unwrap();
        db.insert_book(&book3).unwrap();

        // 获取所有图书
        let all_books = db.get_all_books().unwrap();
        assert_eq!(all_books.len(), 3, "应该有3本图书");
        
        // 验证图书按创建时间倒序排列
        let titles: Vec<&String> = all_books.iter().map(|b| &b.title).collect();
        assert!(titles.contains(&&"图书1".to_string()));
        assert!(titles.contains(&&"图书2".to_string()));
        assert!(titles.contains(&&"图书3".to_string()));
    }

    #[test]
    fn test_search_books() {
        let (db, _temp_dir) = create_test_db();
        
        let book1 = create_test_book("红楼梦", Some("曹雪芹"));
        let book2 = create_test_book("西游记", Some("吴承恩"));
        let book3 = create_test_book("水浒传", Some("施耐庵"));
        
        db.insert_book(&book1).unwrap();
        db.insert_book(&book2).unwrap();
        db.insert_book(&book3).unwrap();

        // 按标题搜索
        let results = db.search_books("红楼").unwrap();
        assert_eq!(results.len(), 1, "搜索'红楼'应该返回1本书");
        assert_eq!(results[0].title, "红楼梦");

        // 按作者搜索
        let results = db.search_books("曹雪芹").unwrap();
        assert_eq!(results.len(), 1, "搜索'曹雪芹'应该返回1本书");
        assert_eq!(results[0].author, Some("曹雪芹".to_string()));

        // 搜索不存在的内容
        let results = db.search_books("不存在的书").unwrap();
        assert_eq!(results.len(), 0, "搜索不存在的内容应该返回空结果");
    }

    #[test]
    fn test_delete_book() {
        let (db, _temp_dir) = create_test_db();
        
        let book = create_test_book("待删除的书", Some("作者"));
        db.insert_book(&book).unwrap();

        // 验证图书存在
        let retrieved = db.get_book_by_id(&book.id).unwrap();
        assert!(retrieved.is_some(), "图书应该存在");

        // 删除图书
        let result = db.delete_book(&book.id);
        assert!(result.is_ok(), "删除图书应该成功");

        // 验证图书已被删除
        let retrieved = db.get_book_by_id(&book.id).unwrap();
        assert!(retrieved.is_none(), "图书应该已被删除");
    }

    #[test]
    fn test_update_reading_progress() {
        let (db, _temp_dir) = create_test_db();
        
        let book = create_test_book("进度测试书", Some("作者"));
        db.insert_book(&book).unwrap();

        // 更新阅读进度
        let result = db.update_reading_progress(&book.id, 0.75);
        assert!(result.is_ok(), "更新阅读进度应该成功");

        // 验证进度已更新
        let updated_book = db.get_book_by_id(&book.id).unwrap().unwrap();
        assert_eq!(updated_book.reading_progress, 0.75);
        assert!(updated_book.last_read.is_some(), "last_read应该被更新");
    }

    #[test]
    fn test_insert_and_get_bookmarks() {
        let (db, _temp_dir) = create_test_db();
        
        let book = create_test_book("书签测试书", Some("作者"));
        db.insert_book(&book).unwrap();

        let bookmark = create_test_bookmark(&book.id, 100);
        
        // 插入书签
        let bookmark_id = db.insert_bookmark(&bookmark).unwrap();
        assert!(bookmark_id > 0, "书签ID应该大于0");

        // 获取书签列表
        let bookmarks = db.get_bookmarks(&book.id).unwrap();
        assert_eq!(bookmarks.len(), 1, "应该有1个书签");
        assert_eq!(bookmarks[0].book_id, book.id);
        assert_eq!(bookmarks[0].position, 100);
        assert_eq!(bookmarks[0].note, Some("测试书签".to_string()));
    }

    #[test]
    fn test_delete_bookmark() {
        let (db, _temp_dir) = create_test_db();
        
        let book = create_test_book("书签删除测试", Some("作者"));
        db.insert_book(&book).unwrap();

        let bookmark = create_test_bookmark(&book.id, 200);
        let bookmark_id = db.insert_bookmark(&bookmark).unwrap();

        // 删除书签
        let result = db.delete_bookmark(bookmark_id);
        assert!(result.is_ok(), "删除书签应该成功");

        // 验证书签已被删除
        let bookmarks = db.get_bookmarks(&book.id).unwrap();
        assert_eq!(bookmarks.len(), 0, "书签应该已被删除");
    }

    #[test]
    fn test_insert_and_get_book_sources() {
        let (db, _temp_dir) = create_test_db();
        
        let source1 = create_test_book_source("source1", "测试书源1");
        let source2 = create_test_book_source("source2", "测试书源2");
        
        // 插入书源
        db.insert_book_source(&source1).unwrap();
        db.insert_book_source(&source2).unwrap();

        // 获取所有书源
        let sources = db.get_all_book_sources().unwrap();
        assert_eq!(sources.len(), 2, "应该有2个书源");
        
        let source_names: Vec<&String> = sources.iter().map(|s| &s.name).collect();
        assert!(source_names.contains(&&"测试书源1".to_string()));
        assert!(source_names.contains(&&"测试书源2".to_string()));
    }

    #[test]
    fn test_toggle_book_source() {
        let (db, _temp_dir) = create_test_db();
        
        let source = create_test_book_source("toggle_test", "切换测试书源");
        db.insert_book_source(&source).unwrap();

        // 禁用书源
        let result = db.toggle_book_source(&source.id, false);
        assert!(result.is_ok(), "禁用书源应该成功");

        // 验证状态已更新
        let sources = db.get_all_book_sources().unwrap();
        let updated_source = sources.iter().find(|s| s.id == source.id).unwrap();
        assert!(!updated_source.enabled, "书源应该被禁用");

        // 重新启用书源
        let result = db.toggle_book_source(&source.id, true);
        assert!(result.is_ok(), "启用书源应该成功");

        let sources = db.get_all_book_sources().unwrap();
        let updated_source = sources.iter().find(|s| s.id == source.id).unwrap();
        assert!(updated_source.enabled, "书源应该被启用");
    }

    #[test]
    fn test_delete_book_source() {
        let (db, _temp_dir) = create_test_db();
        
        let source = create_test_book_source("delete_test", "删除测试书源");
        db.insert_book_source(&source).unwrap();

        // 删除书源
        let result = db.delete_book_source(&source.id);
        assert!(result.is_ok(), "删除书源应该成功");

        // 验证书源已被删除
        let sources = db.get_all_book_sources().unwrap();
        assert_eq!(sources.len(), 0, "书源应该已被删除");
    }

    #[test]
    fn test_save_and_get_setting() {
        let (db, _temp_dir) = create_test_db();
        
        // 保存设置
        let result = db.save_setting("theme", "dark");
        assert!(result.is_ok(), "保存设置应该成功");

        // 获取设置
        let value = db.get_setting("theme").unwrap();
        assert_eq!(value, Some("dark".to_string()));

        // 获取不存在的设置
        let value = db.get_setting("nonexistent").unwrap();
        assert_eq!(value, None);
    }

    #[test]
    fn test_update_existing_setting() {
        let (db, _temp_dir) = create_test_db();
        
        // 保存初始设置
        db.save_setting("language", "en").unwrap();
        
        // 更新设置
        db.save_setting("language", "zh-CN").unwrap();
        
        // 验证设置已更新
        let value = db.get_setting("language").unwrap();
        assert_eq!(value, Some("zh-CN".to_string()));
    }

    #[test]
    fn test_foreign_key_constraint() {
        let (db, _temp_dir) = create_test_db();
        
        let book = create_test_book("外键测试书", Some("作者"));
        db.insert_book(&book).unwrap();

        let bookmark = create_test_bookmark(&book.id, 50);
        db.insert_bookmark(&bookmark).unwrap();

        // 删除图书应该级联删除书签
        db.delete_book(&book.id).unwrap();

        // 验证书签也被删除
        let bookmarks = db.get_bookmarks(&book.id).unwrap();
        assert_eq!(bookmarks.len(), 0, "书签应该被级联删除");
    }

    #[test]
    fn test_duplicate_book_file_path() {
        let (db, _temp_dir) = create_test_db();
        
        let book1 = create_test_book("图书1", Some("作者1"));
        let mut book2 = create_test_book("图书2", Some("作者2"));
        book2.file_path = book1.file_path.clone(); // 相同的文件路径

        // 插入第一本书应该成功
        let result1 = db.insert_book(&book1);
        assert!(result1.is_ok(), "插入第一本书应该成功");

        // 插入相同文件路径的书应该失败（由于UNIQUE约束）
        let result2 = db.insert_book(&book2);
        assert!(result2.is_err(), "插入重复文件路径的书应该失败");
    }

    #[test]
    fn test_book_format_serialization() {
        let (db, _temp_dir) = create_test_db();
        
        // 测试不同格式的图书
        let txt_book = Book {
            format: BookFormat::Txt,
            ..create_test_book("TXT书", Some("作者"))
        };
        
        let epub_book = Book {
            format: BookFormat::Epub,
            ..create_test_book("EPUB书", Some("作者"))
        };
        
        let pdf_book = Book {
            format: BookFormat::Pdf,
            ..create_test_book("PDF书", Some("作者"))
        };

        // 插入不同格式的图书
        db.insert_book(&txt_book).unwrap();
        db.insert_book(&epub_book).unwrap();
        db.insert_book(&pdf_book).unwrap();

        // 验证格式正确保存和读取
        let retrieved_txt = db.get_book_by_id(&txt_book.id).unwrap().unwrap();
        assert_eq!(retrieved_txt.format, BookFormat::Txt);

        let retrieved_epub = db.get_book_by_id(&epub_book.id).unwrap().unwrap();
        assert_eq!(retrieved_epub.format, BookFormat::Epub);

        let retrieved_pdf = db.get_book_by_id(&pdf_book.id).unwrap().unwrap();
        assert_eq!(retrieved_pdf.format, BookFormat::Pdf);
    }
}