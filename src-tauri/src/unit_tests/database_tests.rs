#[cfg(test)]
mod tests {
    use crate::database::DatabaseManager;
    use crate::models::{Book, BookFormat, Bookmark, BookSourceInfo};
    use chrono::Utc;
    use tempfile::TempDir;

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
            author: author.map(|a| a.to_string()),
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
    fn create_test_bookmark(book_id: &str, position: i64, note: Option<&str>) -> Bookmark {
        Bookmark {
            id: 0, // 数据库会自动分配ID
            book_id: book_id.to_string(),
            position,
            chapter_index: Some(1),
            note: note.map(|n| n.to_string()),
            created_at: Utc::now(),
        }
    }

    /// 创建测试用的书源信息对象
    fn create_test_book_source(id: &str, name: &str) -> BookSourceInfo {
        BookSourceInfo {
            id: id.to_string(),
            name: name.to_string(),
            version: "1.0.0".to_string(),
            author: Some("测试作者".to_string()),
            description: Some("测试描述".to_string()),
            base_url: "https://example.com".to_string(),
            enabled: true,
            plugin_path: format!("/test/plugins/{}/index.js", id),
            installed_at: Utc::now(),
        }
    }

    #[test]
    fn test_insert_and_get_book() {
        let (db, _temp_dir) = create_test_db();
        
        // 创建测试图书
        let book = create_test_book("测试图书", Some("测试作者"));
        
        // 插入图书
        let result = db.insert_book(&book);
        assert!(result.is_ok(), "插入图书应该成功");
        
        // 获取所有图书
        let books = db.get_all_books().unwrap();
        assert_eq!(books.len(), 1, "应该有1本图书");
        assert_eq!(books[0].title, "测试图书");
        assert_eq!(books[0].author, Some("测试作者".to_string()));
        
        // 根据ID获取图书
        let book_by_id = db.get_book_by_id(&book.id).unwrap().unwrap();
        assert_eq!(book_by_id.id, book.id);
        assert_eq!(book_by_id.title, book.title);
    }

    #[test]
    fn test_search_books() {
        let (db, _temp_dir) = create_test_db();
        
        // 插入多本图书
        let book1 = create_test_book("小说一", Some("张三"));
        let book2 = create_test_book("小说二", Some("李四"));
        let book3 = create_test_book("故事集", Some("张三"));
        
        db.insert_book(&book1).unwrap();
        db.insert_book(&book2).unwrap();
        db.insert_book(&book3).unwrap();
        
        // 按标题搜索
        let results = db.search_books("小说").unwrap();
        assert_eq!(results.len(), 2, "搜索'小说'应该返回2本书");
        
        // 按作者搜索
        let results = db.search_books("张三").unwrap();
        assert_eq!(results.len(), 2, "搜索'张三'应该返回2本书");
        
        // 精确搜索
        let results = db.search_books("故事集").unwrap();
        assert_eq!(results.len(), 1, "搜索'故事集'应该返回1本书");
        assert_eq!(results[0].title, "故事集");
    }

    #[test]
    fn test_delete_book() {
        let (db, _temp_dir) = create_test_db();
        
        // 插入图书
        let book = create_test_book("待删除图书", None);
        db.insert_book(&book).unwrap();
        
        // 验证图书已插入
        let books_before = db.get_all_books().unwrap();
        assert_eq!(books_before.len(), 1);
        
        // 删除图书
        let result = db.delete_book(&book.id);
        assert!(result.is_ok(), "删除图书应该成功");
        
        // 验证图书已删除
        let books_after = db.get_all_books().unwrap();
        assert_eq!(books_after.len(), 0, "图书应该已被删除");
    }

    #[test]
    fn test_update_reading_progress() {
        let (db, _temp_dir) = create_test_db();
        
        // 插入图书
        let book = create_test_book("阅读进度测试", None);
        db.insert_book(&book).unwrap();
        
        // 更新阅读进度
        let result = db.update_reading_progress(&book.id, 0.5);
        assert!(result.is_ok(), "更新阅读进度应该成功");
        
        // 验证进度已更新
        let updated_book = db.get_book_by_id(&book.id).unwrap().unwrap();
        assert_eq!(updated_book.reading_progress, 0.5);
        assert!(updated_book.last_read.is_some(), "最后阅读时间应该已更新");
    }

    #[test]
    fn test_bookmarks() {
        let (db, _temp_dir) = create_test_db();
        
        // 插入图书
        let book = create_test_book("书签测试", None);
        db.insert_book(&book).unwrap();
        
        // 创建书签
        let bookmark = create_test_bookmark(&book.id, 100, Some("测试笔记"));
        
        // 插入书签
        let bookmark_id = db.insert_bookmark(&bookmark).unwrap();
        assert!(bookmark_id > 0, "书签ID应该大于0");
        
        // 获取书签
        let bookmarks = db.get_bookmarks(&book.id).unwrap();
        assert_eq!(bookmarks.len(), 1, "应该有1个书签");
        assert_eq!(bookmarks[0].position, 100);
        assert_eq!(bookmarks[0].note, Some("测试笔记".to_string()));
        
        // 删除书签
        let result = db.delete_bookmark(bookmark_id);
        assert!(result.is_ok(), "删除书签应该成功");
        
        // 验证书签已删除
        let bookmarks_after = db.get_bookmarks(&book.id).unwrap();
        assert_eq!(bookmarks_after.len(), 0, "书签应该已被删除");
    }

    #[test]
    fn test_book_sources() {
        let (db, _temp_dir) = create_test_db();
        
        // 创建书源
        let source = create_test_book_source("test-source", "测试书源");
        
        // 插入书源
        let result = db.insert_book_source(&source);
        assert!(result.is_ok(), "插入书源应该成功");
        
        // 获取所有书源
        let sources = db.get_all_book_sources().unwrap();
        assert_eq!(sources.len(), 1, "应该有1个书源");
        assert_eq!(sources[0].id, "test-source");
        assert_eq!(sources[0].name, "测试书源");
        
        // 切换书源状态
        let result = db.toggle_book_source("test-source", false);
        assert!(result.is_ok(), "切换书源状态应该成功");
        
        // 验证状态已更新
        let sources_after = db.get_all_book_sources().unwrap();
        assert!(!sources_after[0].enabled, "书源应该被禁用");
        
        // 删除书源
        let result = db.delete_book_source("test-source");
        assert!(result.is_ok(), "删除书源应该成功");
        
        // 验证书源已删除
        let sources_final = db.get_all_book_sources().unwrap();
        assert_eq!(sources_final.len(), 0, "书源应该已被删除");
    }

    #[test]
    fn test_settings() {
        let (db, _temp_dir) = create_test_db();
        
        // 保存设置
        let result = db.save_setting("theme", "dark");
        assert!(result.is_ok(), "保存设置应该成功");
        
        // 获取设置
        let theme = db.get_setting("theme").unwrap();
        assert_eq!(theme, Some("dark".to_string()));
        
        // 更新设置
        let result = db.save_setting("theme", "light");
        assert!(result.is_ok(), "更新设置应该成功");
        
        // 验证设置已更新
        let updated_theme = db.get_setting("theme").unwrap();
        assert_eq!(updated_theme, Some("light".to_string()));
        
        // 获取不存在的设置
        let nonexistent = db.get_setting("nonexistent").unwrap();
        assert_eq!(nonexistent, None);
    }

    #[test]
    fn test_foreign_key_constraints() {
        let (db, _temp_dir) = create_test_db();
        
        // 插入图书
        let book = create_test_book("外键测试", None);
        db.insert_book(&book).unwrap();
        
        // 插入书签
        let bookmark = create_test_bookmark(&book.id, 100, None);
        let bookmark_id = db.insert_bookmark(&bookmark).unwrap();
        
        // 删除图书（应该级联删除书签）
        db.delete_book(&book.id).unwrap();
        
        // 尝试获取书签（应该已被级联删除）
        let mut conn = db.get_connection().unwrap();
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM bookmarks WHERE id = ?").unwrap();
        let count: i64 = stmt.query_row([bookmark_id], |row| row.get(0)).unwrap();
        
        assert_eq!(count, 0, "书签应该已被级联删除");
    }

    #[test]
    fn test_transaction_safety() {
        let (db, _temp_dir) = create_test_db();
        
        // 获取数据库连接
        let mut conn = db.get_connection().unwrap();
        
        // 开始事务
        let tx = conn.transaction().unwrap();
        
        // 在事务中执行操作
        let book = create_test_book("事务测试", None);
        tx.execute(
            r#"
            INSERT INTO books (
                id, title, author, file_path, format, file_size, 
                created_at, last_read, reading_progress, total_chapters
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
            "#,
            rusqlite::params![
                book.id,
                book.title,
                book.author,
                book.file_path,
                book.format.to_string(),
                book.file_size,
                book.created_at.to_rfc3339(),
                book.last_read.map(|dt| dt.to_rfc3339()),
                book.reading_progress,
                book.total_chapters
            ],
        ).unwrap();
        
        // 回滚事务
        tx.rollback().unwrap();
        
        // 验证数据未被保存
        let books = db.get_all_books().unwrap();
        assert_eq!(books.len(), 0, "事务回滚后不应该有图书");
    }
}