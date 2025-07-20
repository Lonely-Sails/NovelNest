use crate::errors::{AppError, AppResult};
use crate::models::{Book, Bookmark, BookSourceInfo};
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection};
use std::path::Path;
use std::sync::{Arc, Mutex};

/// 数据库管理器
pub struct DatabaseManager {
    connection: Arc<Mutex<Connection>>,
}

impl DatabaseManager {
    /// 创建新的数据库管理器实例
    pub fn new<P: AsRef<Path>>(db_path: P) -> AppResult<Self> {
        let conn = Connection::open(db_path)?;
        
        // 启用外键约束
        conn.execute("PRAGMA foreign_keys = ON", [])?;
        
        let manager = Self {
            connection: Arc::new(Mutex::new(conn)),
        };
        
        // 初始化数据库表
        manager.initialize_tables()?;
        
        Ok(manager)
    }

    /// 初始化数据库表结构
    fn initialize_tables(&self) -> AppResult<()> {
        let conn: std::sync::MutexGuard<'_, Connection> = self.connection.lock().map_err(|e| {
            AppError::Generic(format!("Failed to acquire database lock: {}", e))
        })?;

        // 创建图书表
        conn.execute(
            r#"
            CREATE TABLE IF NOT EXISTS books (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                author TEXT,
                file_path TEXT NOT NULL UNIQUE,
                format TEXT NOT NULL,
                file_size INTEGER NOT NULL,
                created_at TEXT NOT NULL,
                last_read TEXT,
                reading_progress REAL DEFAULT 0.0,
                total_chapters INTEGER DEFAULT 0
            )
            "#,
            [],
        )?;

        // 创建书签表
        conn.execute(
            r#"
            CREATE TABLE IF NOT EXISTS bookmarks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                book_id TEXT NOT NULL,
                position INTEGER NOT NULL,
                chapter_index INTEGER,
                note TEXT,
                created_at TEXT NOT NULL,
                FOREIGN KEY (book_id) REFERENCES books(id) ON DELETE CASCADE
            )
            "#,
            [],
        )?;

        // 创建阅读历史表
        conn.execute(
            r#"
            CREATE TABLE IF NOT EXISTS reading_history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                book_id TEXT NOT NULL,
                read_at TEXT NOT NULL,
                duration INTEGER,
                FOREIGN KEY (book_id) REFERENCES books(id) ON DELETE CASCADE
            )
            "#,
            [],
        )?;

        // 创建书源表
        conn.execute(
            r#"
            CREATE TABLE IF NOT EXISTS book_sources (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                version TEXT NOT NULL,
                author TEXT,
                description TEXT,
                base_url TEXT NOT NULL,
                enabled BOOLEAN DEFAULT TRUE,
                plugin_path TEXT NOT NULL,
                installed_at TEXT NOT NULL
            )
            "#,
            [],
        )?;

        // 创建用户设置表
        conn.execute(
            r#"
            CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )
            "#,
            [],
        )?;

        // 创建索引以提高查询性能
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_books_title ON books(title)",
            [],
        )?;
        
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_books_author ON books(author)",
            [],
        )?;
        
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_bookmarks_book_id ON bookmarks(book_id)",
            [],
        )?;
        
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_reading_history_book_id ON reading_history(book_id)",
            [],
        )?;

        Ok(())
    }

    /// 插入新图书
    pub fn insert_book(&self, book: &Book) -> AppResult<()> {
        let conn = self.connection.lock().map_err(|e| {
            AppError::Generic(format!("Failed to acquire database lock: {}", e))
        })?;

        conn.execute(
            r#"
            INSERT INTO books (
                id, title, author, file_path, format, file_size, 
                created_at, last_read, reading_progress, total_chapters
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
            "#,
            params![
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
        )?;

        Ok(())
    }

    /// 获取所有图书
    pub fn get_all_books(&self) -> AppResult<Vec<Book>> {
        let conn = self.connection.lock().map_err(|e| {
            AppError::Generic(format!("Failed to acquire database lock: {}", e))
        })?;

        let mut stmt = conn.prepare(
            r#"
            SELECT id, title, author, file_path, format, file_size, 
                   created_at, last_read, reading_progress, total_chapters
            FROM books ORDER BY created_at DESC
            "#,
        )?;

        let book_iter = stmt.query_map([], |row| {
            let created_at_str: String = row.get(6)?;
            let last_read_str: Option<String> = row.get(7)?;
            let format_str: String = row.get(4)?;

            Ok(Book {
                id: row.get(0)?,
                title: row.get(1)?,
                author: row.get(2)?,
                file_path: row.get(3)?,
                format: match format_str.as_str() {
                    "txt" => crate::models::BookFormat::Txt,
                    "epub" => crate::models::BookFormat::Epub,
                    "pdf" => crate::models::BookFormat::Pdf,
                    _ => crate::models::BookFormat::Txt,
                },
                file_size: row.get(5)?,
                created_at: DateTime::parse_from_rfc3339(&created_at_str)
                    .map_err(|e| rusqlite::Error::InvalidColumnType(6, "created_at".to_string(), rusqlite::types::Type::Text))?
                    .with_timezone(&Utc),
                last_read: last_read_str
                    .map(|s| DateTime::parse_from_rfc3339(&s)
                        .map(|dt| dt.with_timezone(&Utc)))
                    .transpose()
                    .map_err(|e| rusqlite::Error::InvalidColumnType(7, "last_read".to_string(), rusqlite::types::Type::Text))?,
                reading_progress: row.get(8)?,
                total_chapters: row.get(9)?,
            })
        })?;

        let mut books = Vec::new();
        for book in book_iter {
            books.push(book?);
        }

        Ok(books)
    }

    /// 根据ID获取图书
    pub fn get_book_by_id(&self, book_id: &str) -> AppResult<Option<Book>> {
        let conn = self.connection.lock().map_err(|e| {
            AppError::Generic(format!("Failed to acquire database lock: {}", e))
        })?;

        let mut stmt = conn.prepare(
            r#"
            SELECT id, title, author, file_path, format, file_size, 
                   created_at, last_read, reading_progress, total_chapters
            FROM books WHERE id = ?1
            "#,
        )?;

        let mut book_iter = stmt.query_map([book_id], |row| {
            let created_at_str: String = row.get(6)?;
            let last_read_str: Option<String> = row.get(7)?;
            let format_str: String = row.get(4)?;

            Ok(Book {
                id: row.get(0)?,
                title: row.get(1)?,
                author: row.get(2)?,
                file_path: row.get(3)?,
                format: match format_str.as_str() {
                    "txt" => crate::models::BookFormat::Txt,
                    "epub" => crate::models::BookFormat::Epub,
                    "pdf" => crate::models::BookFormat::Pdf,
                    _ => crate::models::BookFormat::Txt,
                },
                file_size: row.get(5)?,
                created_at: DateTime::parse_from_rfc3339(&created_at_str)
                    .map_err(|e| rusqlite::Error::InvalidColumnType(6, "created_at".to_string(), rusqlite::types::Type::Text))?
                    .with_timezone(&Utc),
                last_read: last_read_str
                    .map(|s| DateTime::parse_from_rfc3339(&s)
                        .map(|dt| dt.with_timezone(&Utc)))
                    .transpose()
                    .map_err(|e| rusqlite::Error::InvalidColumnType(7, "last_read".to_string(), rusqlite::types::Type::Text))?,
                reading_progress: row.get(8)?,
                total_chapters: row.get(9)?,
            })
        })?;

        match book_iter.next() {
            Some(book) => Ok(Some(book?)),
            None => Ok(None),
        }
    }

    /// 搜索图书
    pub fn search_books(&self, query: &str) -> AppResult<Vec<Book>> {
        let conn = self.connection.lock().map_err(|e| {
            AppError::Generic(format!("Failed to acquire database lock: {}", e))
        })?;

        let search_pattern = format!("%{}%", query);
        let mut stmt = conn.prepare(
            r#"
            SELECT id, title, author, file_path, format, file_size, 
                   created_at, last_read, reading_progress, total_chapters
            FROM books 
            WHERE title LIKE ?1 OR author LIKE ?1
            ORDER BY created_at DESC
            "#,
        )?;

        let book_iter = stmt.query_map([&search_pattern], |row| {
            let created_at_str: String = row.get(6)?;
            let last_read_str: Option<String> = row.get(7)?;
            let format_str: String = row.get(4)?;

            Ok(Book {
                id: row.get(0)?,
                title: row.get(1)?,
                author: row.get(2)?,
                file_path: row.get(3)?,
                format: match format_str.as_str() {
                    "txt" => crate::models::BookFormat::Txt,
                    "epub" => crate::models::BookFormat::Epub,
                    "pdf" => crate::models::BookFormat::Pdf,
                    _ => crate::models::BookFormat::Txt,
                },
                file_size: row.get(5)?,
                created_at: DateTime::parse_from_rfc3339(&created_at_str)
                    .map_err(|e| rusqlite::Error::InvalidColumnType(6, "created_at".to_string(), rusqlite::types::Type::Text))?
                    .with_timezone(&Utc),
                last_read: last_read_str
                    .map(|s| DateTime::parse_from_rfc3339(&s)
                        .map(|dt| dt.with_timezone(&Utc)))
                    .transpose()
                    .map_err(|e| rusqlite::Error::InvalidColumnType(7, "last_read".to_string(), rusqlite::types::Type::Text))?,
                reading_progress: row.get(8)?,
                total_chapters: row.get(9)?,
            })
        })?;

        let mut books = Vec::new();
        for book in book_iter {
            books.push(book?);
        }

        Ok(books)
    }

    /// 删除图书
    pub fn delete_book(&self, book_id: &str) -> AppResult<()> {
        let conn = self.connection.lock().map_err(|e| {
            AppError::Generic(format!("Failed to acquire database lock: {}", e))
        })?;

        conn.execute("DELETE FROM books WHERE id = ?1", [book_id])?;
        Ok(())
    }

    /// 更新阅读进度
    pub fn update_reading_progress(&self, book_id: &str, progress: f64) -> AppResult<()> {
        let conn = self.connection.lock().map_err(|e| {
            AppError::Generic(format!("Failed to acquire database lock: {}", e))
        })?;

        conn.execute(
            "UPDATE books SET reading_progress = ?1, last_read = ?2 WHERE id = ?3",
            params![progress, Utc::now().to_rfc3339(), book_id],
        )?;

        Ok(())
    }

    /// 插入书签
    pub fn insert_bookmark(&self, bookmark: &Bookmark) -> AppResult<i64> {
        let conn = self.connection.lock().map_err(|e| {
            AppError::Generic(format!("Failed to acquire database lock: {}", e))
        })?;

        conn.execute(
            r#"
            INSERT INTO bookmarks (book_id, position, chapter_index, note, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5)
            "#,
            params![
                bookmark.book_id,
                bookmark.position,
                bookmark.chapter_index,
                bookmark.note,
                bookmark.created_at.to_rfc3339()
            ],
        )?;

        Ok(conn.last_insert_rowid())
    }

    /// 获取图书的所有书签
    pub fn get_bookmarks(&self, book_id: &str) -> AppResult<Vec<Bookmark>> {
        let conn = self.connection.lock().map_err(|e| {
            AppError::Generic(format!("Failed to acquire database lock: {}", e))
        })?;

        let mut stmt = conn.prepare(
            r#"
            SELECT id, book_id, position, chapter_index, note, created_at
            FROM bookmarks WHERE book_id = ?1 ORDER BY position
            "#,
        )?;

        let bookmark_iter = stmt.query_map([book_id], |row| {
            let created_at_str: String = row.get(5)?;

            Ok(Bookmark {
                id: row.get(0)?,
                book_id: row.get(1)?,
                position: row.get(2)?,
                chapter_index: row.get(3)?,
                note: row.get(4)?,
                created_at: DateTime::parse_from_rfc3339(&created_at_str)
                    .map_err(|e| rusqlite::Error::InvalidColumnType(5, "created_at".to_string(), rusqlite::types::Type::Text))?
                    .with_timezone(&Utc),
            })
        })?;

        let mut bookmarks = Vec::new();
        for bookmark in bookmark_iter {
            bookmarks.push(bookmark?);
        }

        Ok(bookmarks)
    }

    /// 删除书签
    pub fn delete_bookmark(&self, bookmark_id: i64) -> AppResult<()> {
        let conn = self.connection.lock().map_err(|e| {
            AppError::Generic(format!("Failed to acquire database lock: {}", e))
        })?;

        conn.execute("DELETE FROM bookmarks WHERE id = ?1", [bookmark_id])?;
        Ok(())
    }

    /// 插入书源信息
    pub fn insert_book_source(&self, source: &BookSourceInfo) -> AppResult<()> {
        let conn = self.connection.lock().map_err(|e| {
            AppError::Generic(format!("Failed to acquire database lock: {}", e))
        })?;

        conn.execute(
            r#"
            INSERT INTO book_sources (
                id, name, version, author, description, base_url, 
                enabled, plugin_path, installed_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
            "#,
            params![
                source.id,
                source.name,
                source.version,
                source.author,
                source.description,
                source.base_url,
                source.enabled,
                source.plugin_path,
                source.installed_at.to_rfc3339()
            ],
        )?;

        Ok(())
    }

    /// 获取所有书源
    pub fn get_all_book_sources(&self) -> AppResult<Vec<BookSourceInfo>> {
        let conn = self.connection.lock().map_err(|e| {
            AppError::Generic(format!("Failed to acquire database lock: {}", e))
        })?;

        let mut stmt = conn.prepare(
            r#"
            SELECT id, name, version, author, description, base_url, 
                   enabled, plugin_path, installed_at
            FROM book_sources ORDER BY installed_at DESC
            "#,
        )?;

        let source_iter = stmt.query_map([], |row| {
            let installed_at_str: String = row.get(8)?;

            Ok(BookSourceInfo {
                id: row.get(0)?,
                name: row.get(1)?,
                version: row.get(2)?,
                author: row.get(3)?,
                description: row.get(4)?,
                base_url: row.get(5)?,
                enabled: row.get(6)?,
                plugin_path: row.get(7)?,
                installed_at: DateTime::parse_from_rfc3339(&installed_at_str)
                    .map_err(|e| rusqlite::Error::InvalidColumnType(8, "installed_at".to_string(), rusqlite::types::Type::Text))?
                    .with_timezone(&Utc),
            })
        })?;

        let mut sources = Vec::new();
        for source in source_iter {
            sources.push(source?);
        }

        Ok(sources)
    }

    /// 更新书源状态
    pub fn toggle_book_source(&self, source_id: &str, enabled: bool) -> AppResult<()> {
        let conn = self.connection.lock().map_err(|e| {
            AppError::Generic(format!("Failed to acquire database lock: {}", e))
        })?;

        conn.execute(
            "UPDATE book_sources SET enabled = ?1 WHERE id = ?2",
            params![enabled, source_id],
        )?;

        Ok(())
    }

    /// 删除书源
    pub fn delete_book_source(&self, source_id: &str) -> AppResult<()> {
        let conn = self.connection.lock().map_err(|e| {
            AppError::Generic(format!("Failed to acquire database lock: {}", e))
        })?;

        conn.execute("DELETE FROM book_sources WHERE id = ?1", [source_id])?;
        Ok(())
    }

    /// 保存设置
    pub fn save_setting(&self, key: &str, value: &str) -> AppResult<()> {
        let conn = self.connection.lock().map_err(|e| {
            AppError::Generic(format!("Failed to acquire database lock: {}", e))
        })?;

        conn.execute(
            r#"
            INSERT OR REPLACE INTO settings (key, value, updated_at)
            VALUES (?1, ?2, ?3)
            "#,
            params![key, value, Utc::now().to_rfc3339()],
        )?;

        Ok(())
    }

    /// 获取设置
    pub fn get_setting(&self, key: &str) -> AppResult<Option<String>> {
        let conn = self.connection.lock().map_err(|e| {
            AppError::Generic(format!("Failed to acquire database lock: {}", e))
        })?;

        let mut stmt = conn.prepare("SELECT value FROM settings WHERE key = ?1")?;
        let mut rows = stmt.query_map([key], |row| {
            Ok(row.get::<_, String>(0)?)
        })?;

        match rows.next() {
            Some(value) => Ok(Some(value?)),
            None => Ok(None),
        }
    }

    /// 获取数据库连接（用于其他模块）
    pub fn get_connection(&self) -> AppResult<std::sync::MutexGuard<Connection>> {
        self.connection.lock().map_err(|e| {
            AppError::Generic(format!("Failed to acquire database lock: {}", e))
        })
    }
}