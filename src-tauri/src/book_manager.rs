use crate::database::DatabaseManager;
use crate::errors::{BookError, BookResult};
use crate::models::{Book, BookFormat, LibraryStats};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// 图书管理器
pub struct BookManager {
    db: Arc<DatabaseManager>,
    storage_path: PathBuf,
}

impl BookManager {
    /// 创建新的图书管理器实例
    pub fn new(db: Arc<DatabaseManager>, storage_path: PathBuf) -> Self {
        Self { db, storage_path }
    }

    /// 导入单个图书文件
    pub async fn import_book(&self, file_path: PathBuf) -> BookResult<Book> {
        // 验证文件是否存在
        if !file_path.exists() {
            return Err(BookError::NotFound(format!(
                "文件不存在: {}",
                file_path.display()
            )));
        }

        // 获取文件扩展名并验证格式
        let extension = file_path
            .extension()
            .and_then(|ext| ext.to_str())
            .ok_or_else(|| BookError::UnsupportedFormat("无法确定文件格式".to_string()))?;

        let format = BookFormat::from_extension(extension).ok_or_else(|| {
            BookError::UnsupportedFormat(format!("不支持的文件格式: {}", extension))
        })?;

        // 检查是否已经导入过该文件
        if let Ok(existing_books) = self.db.get_all_books() {
            for existing_book in existing_books {
                if existing_book.file_path == file_path.to_string_lossy() {
                    return Err(BookError::Duplicate(format!(
                        "图书已存在: {}",
                        existing_book.title
                    )));
                }
            }
        }

        // 获取文件大小
        let file_size = fs::metadata(&file_path)
            .map_err(|e| BookError::CorruptedFile(format!("无法读取文件信息: {}", e)))?
            .len() as i64;

        // 解析图书元数据
        let (title, author) = self.extract_metadata(&file_path, &format).await?;

        // 创建图书对象
        let book = Book::new(title, author, file_path, format, file_size);

        // 保存到数据库
        self.db
            .insert_book(&book)
            .map_err(|e| BookError::MetadataExtractionFailed(format!("数据库保存失败: {}", e)))?;

        Ok(book)
    }

    /// 批量导入文件夹中的图书
    pub async fn import_folder(&self, folder_path: PathBuf) -> BookResult<Vec<Book>> {
        if !folder_path.exists() || !folder_path.is_dir() {
            return Err(BookError::NotFound(format!(
                "文件夹不存在或不是有效目录: {}",
                folder_path.display()
            )));
        }

        let mut imported_books = Vec::new();
        let mut errors = Vec::new();

        // 递归扫描文件夹
        self.scan_folder_recursive(&folder_path, &mut imported_books, &mut errors)
            .await;

        // 如果有错误但也有成功导入的书籍，记录错误但返回成功的结果
        if !errors.is_empty() {
            eprintln!("导入过程中遇到 {} 个错误:", errors.len());
            for error in &errors {
                eprintln!("  - {}", error);
            }
        }

        Ok(imported_books)
    }

    /// 递归扫描文件夹
    async fn scan_folder_recursive(
        &self,
        folder_path: &Path,
        imported_books: &mut Vec<Book>,
        errors: &mut Vec<String>,
    ) {
        let entries = match fs::read_dir(folder_path) {
            Ok(entries) => entries,
            Err(e) => {
                errors.push(format!("无法读取目录 {}: {}", folder_path.display(), e));
                return;
            }
        };

        for entry in entries {
            let entry = match entry {
                Ok(entry) => entry,
                Err(e) => {
                    errors.push(format!("读取目录项失败: {}", e));
                    continue;
                }
            };

            let path = entry.path();

            if path.is_dir() {
                // 递归处理子目录
                Box::pin(self.scan_folder_recursive(&path, imported_books, errors)).await;
            } else if path.is_file() {
                // 尝试导入文件
                match self.import_book(path.clone()).await {
                    Ok(book) => {
                        imported_books.push(book);
                    }
                    Err(BookError::UnsupportedFormat(_)) => {
                        // 跳过不支持的格式，不记录为错误
                    }
                    Err(BookError::Duplicate(_)) => {
                        // 跳过重复文件，不记录为错误
                    }
                    Err(e) => {
                        errors.push(format!("导入文件 {} 失败: {}", path.display(), e));
                    }
                }
            }
        }
    }

    /// 提取图书元数据
    async fn extract_metadata(
        &self,
        file_path: &Path,
        format: &BookFormat,
    ) -> BookResult<(String, Option<String>)> {
        match format {
            BookFormat::Txt => self.extract_txt_metadata(file_path).await,
            BookFormat::Pdf => self.extract_pdf_metadata(file_path).await,
        }
    }

    /// 提取TXT文件元数据
    async fn extract_txt_metadata(&self, file_path: &Path) -> BookResult<(String, Option<String>)> {
        // 对于TXT文件，使用文件名作为标题
        let title = file_path
            .file_stem()
            .and_then(|name| name.to_str())
            .unwrap_or("未知标题")
            .to_string();

        // 尝试从文件内容的前几行提取作者信息
        let author = match fs::read_to_string(file_path) {
            Ok(content) => self.extract_author_from_txt_content(&content),
            Err(_) => None,
        };

        Ok((title, author))
    }

    /// 从TXT内容中提取作者信息
    fn extract_author_from_txt_content(&self, content: &str) -> Option<String> {
        let lines: Vec<&str> = content.lines().take(10).collect(); // 只检查前10行

        for line in lines {
            let line = line.trim();

            // 常见的作者标识模式
            if line.starts_with("作者：") || line.starts_with("作者:") {
                return Some(
                    line.replace("作者：", "")
                        .replace("作者:", "")
                        .trim()
                        .to_string(),
                );
            }

            if line.starts_with("Author:") || line.starts_with("author:") {
                return Some(
                    line.replace("Author:", "")
                        .replace("author:", "")
                        .trim()
                        .to_string(),
                );
            }

            // 检查是否包含"著"字
            if line.contains("著") && line.len() < 50 {
                // 简单的启发式规则：如果行很短且包含"著"，可能是作者信息
                return Some(line.to_string());
            }
        }

        None
    }

    /// 提取PDF文件元数据
    async fn extract_pdf_metadata(&self, file_path: &Path) -> BookResult<(String, Option<String>)> {
        // 使用文件名作为标题（PDF元数据提取比较复杂，这里简化处理）
        let title = file_path
            .file_stem()
            .and_then(|name| name.to_str())
            .unwrap_or("未知标题")
            .to_string();

        // 尝试提取PDF元数据
        let author = match self.extract_pdf_author(file_path) {
            Ok(author) => author,
            Err(_) => None,
        };

        Ok((title, author))
    }

    /// 提取PDF作者信息
    fn extract_pdf_author(&self, file_path: &Path) -> BookResult<Option<String>> {
        // 这里可以使用pdf-extract crate来提取PDF元数据
        // 由于PDF解析比较复杂，这里先返回None，后续可以完善
        let _ = file_path; // 避免未使用变量警告
        Ok(None)
    }

    /// 获取所有图书
    pub async fn get_all_books(&self) -> BookResult<Vec<Book>> {
        self.db
            .get_all_books()
            .map_err(|e| BookError::MetadataExtractionFailed(format!("获取图书列表失败: {}", e)))
    }

    /// 搜索图书
    pub async fn search_books(&self, query: &str) -> BookResult<Vec<Book>> {
        self.db
            .search_books(query)
            .map_err(|e| BookError::MetadataExtractionFailed(format!("搜索图书失败: {}", e)))
    }

    /// 按格式过滤图书
    pub async fn filter_books_by_format(&self, format: &BookFormat) -> BookResult<Vec<Book>> {
        let all_books = self.get_all_books().await?;
        let filtered_books = all_books
            .into_iter()
            .filter(|book| book.format == *format)
            .collect();
        Ok(filtered_books)
    }

    /// 按作者过滤图书
    pub async fn filter_books_by_author(&self, author: &str) -> BookResult<Vec<Book>> {
        let all_books = self.get_all_books().await?;
        let filtered_books = all_books
            .into_iter()
            .filter(|book| {
                book.author
                    .as_ref()
                    .map(|a| a.contains(author))
                    .unwrap_or(false)
            })
            .collect();
        Ok(filtered_books)
    }

    /// 获取最近添加的图书
    pub async fn get_recent_books(&self, limit: usize) -> BookResult<Vec<Book>> {
        let all_books = self.get_all_books().await?;
        let mut recent_books = all_books;
        recent_books.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        recent_books.truncate(limit);
        Ok(recent_books)
    }

    /// 获取最近阅读的图书
    pub async fn get_recently_read_books(&self, limit: usize) -> BookResult<Vec<Book>> {
        let all_books = self.get_all_books().await?;
        let mut recently_read: Vec<Book> = all_books
            .into_iter()
            .filter(|book| book.last_read.is_some())
            .collect();

        recently_read.sort_by(|a, b| {
            b.last_read
                .unwrap_or_default()
                .cmp(&a.last_read.unwrap_or_default())
        });
        recently_read.truncate(limit);
        Ok(recently_read)
    }

    /// 获取图书统计信息
    pub async fn get_library_stats(&self) -> BookResult<LibraryStats> {
        let all_books = self.get_all_books().await?;

        let total_books = all_books.len();
        let mut format_counts = std::collections::HashMap::new();
        let mut total_size = 0i64;
        let mut books_with_progress = 0;

        for book in &all_books {
            *format_counts.entry(book.format.clone()).or_insert(0) += 1;
            total_size += book.file_size;
            if book.current_chapter > 0 || book.current_line_index > 0 {
                books_with_progress += 1;
            }
        }

        // 简化统计，不再计算平均进度
        let average_progress = 0.0;

        Ok(LibraryStats {
            total_books,
            total_size,
            format_counts,
            books_with_progress,
            average_progress,
        })
    }

    /// 删除图书
    pub async fn delete_book(&self, book_id: &str) -> BookResult<()> {
        // 首先获取图书信息
        let _book = self
            .db
            .get_book_by_id(book_id)
            .map_err(|e| BookError::MetadataExtractionFailed(format!("获取图书信息失败: {}", e)))?
            .ok_or_else(|| BookError::NotFound(format!("图书不存在: {}", book_id)))?;

        // 从数据库中删除记录
        self.db
            .delete_book(book_id)
            .map_err(|e| BookError::MetadataExtractionFailed(format!("删除图书失败: {}", e)))?;

        // 注意：这里不删除实际文件，只删除数据库记录
        // 如果需要删除文件，可以添加相应的逻辑

        Ok(())
    }

    /// 保存阅读进度（章节和行索引）
    pub async fn save_reading_progress(
        &self, 
        book_id: &str, 
        current_chapter: i32, 
        current_line_index: i32
    ) -> BookResult<()> {
        // 验证图书是否存在
        let _book = self
            .db
            .get_book_by_id(book_id)
            .map_err(|e| BookError::MetadataExtractionFailed(format!("获取图书信息失败: {}", e)))?
            .ok_or_else(|| BookError::NotFound(format!("图书不存在: {}", book_id)))?;

        // 保存阅读进度信息
        self.db
            .update_reading_progress(book_id, current_chapter, current_line_index)
            .map_err(|e| {
                BookError::MetadataExtractionFailed(format!("保存阅读进度失败: {}", e))
            })?;

        Ok(())
    }

    /// 获取阅读进度
    pub async fn get_reading_progress(&self, book_id: &str) -> BookResult<(i32, i32)> {
        let book = self
            .db
            .get_book_by_id(book_id)
            .map_err(|e| BookError::MetadataExtractionFailed(format!("获取图书信息失败: {}", e)))?
            .ok_or_else(|| BookError::NotFound(format!("图书不存在: {}", book_id)))?;

        Ok((book.current_chapter, book.current_line_index))
    }

    /// 添加书签
    pub async fn add_bookmark(
        &self,
        book_id: &str,
        position: i64,
        chapter_index: Option<i32>,
        note: Option<String>,
    ) -> BookResult<i64> {
        // 验证图书是否存在
        let _book = self
            .db
            .get_book_by_id(book_id)
            .map_err(|e| BookError::MetadataExtractionFailed(format!("获取图书信息失败: {}", e)))?
            .ok_or_else(|| BookError::NotFound(format!("图书不存在: {}", book_id)))?;

        // 验证位置参数
        if position < 0 {
            return Err(BookError::InvalidProgress(format!(
                "书签位置不能为负数: {}",
                position
            )));
        }

        // 创建书签对象
        let bookmark = crate::models::Bookmark {
            id: 0, // 数据库会自动分配ID
            book_id: book_id.to_string(),
            position,
            chapter_index,
            note,
            created_at: chrono::Utc::now(),
        };

        // 保存到数据库
        let bookmark_id = self
            .db
            .insert_bookmark(&bookmark)
            .map_err(|e| BookError::MetadataExtractionFailed(format!("添加书签失败: {}", e)))?;

        Ok(bookmark_id)
    }

    /// 获取图书的所有书签
    pub async fn get_bookmarks(&self, book_id: &str) -> BookResult<Vec<crate::models::Bookmark>> {
        // 直接获取书签列表，不需要先验证图书是否存在
        // 如果图书不存在，书签查询会返回空列表，这是合理的行为
        let bookmarks = self
            .db
            .get_bookmarks(book_id)
            .map_err(|e| BookError::MetadataExtractionFailed(format!("获取书签列表失败: {}", e)))?;

        Ok(bookmarks)
    }

    /// 删除书签
    pub async fn delete_bookmark(&self, bookmark_id: i64) -> BookResult<()> {
        self.db
            .delete_bookmark(bookmark_id)
            .map_err(|e| BookError::MetadataExtractionFailed(format!("删除书签失败: {}", e)))?;

        Ok(())
    }

    /// 快速跳转到书签位置（获取书签详细信息）
    pub async fn get_bookmark_by_id(
        &self,
        bookmark_id: i64,
    ) -> BookResult<Option<crate::models::Bookmark>> {
        // 获取所有书签，然后找到指定ID的书签
        // 这里可以优化为直接从数据库查询单个书签
        let all_bookmarks = self
            .db
            .get_all_books()
            .map_err(|e| BookError::MetadataExtractionFailed(format!("获取图书列表失败: {}", e)))?;

        for book in all_bookmarks {
            let bookmarks = self.get_bookmarks(&book.id).await?;
            for bookmark in bookmarks {
                if bookmark.id == bookmark_id {
                    return Ok(Some(bookmark));
                }
            }
        }

        Ok(None)
    }

    /// 获取图书内容（用于阅读）
    pub async fn get_book_content(&self, book_id: &str) -> BookResult<String> {
        // 获取图书信息
        let book = self
            .db
            .get_book_by_id(book_id)
            .map_err(|e| BookError::MetadataExtractionFailed(format!("获取图书信息失败: {}", e)))?
            .ok_or_else(|| BookError::NotFound(format!("图书不存在: {}", book_id)))?;

        let file_path = PathBuf::from(&book.file_path);

        // 支持TXT和PDF格式
        match book.format {
            BookFormat::Txt => self.read_txt_content(&file_path).await,
            BookFormat::Pdf => self.read_pdf_content(&file_path).await,
        }
    }

    /// 读取TXT文件内容
    async fn read_txt_content(&self, file_path: &Path) -> BookResult<String> {
        // 检查文件大小，避免读取过大的文件
        let metadata = fs::metadata(file_path)
            .map_err(|e| BookError::CorruptedFile(format!("无法获取文件信息: {}", e)))?;

        let file_size = metadata.len();

        // 如果文件超过50MB，返回错误
        if file_size > 50 * 1024 * 1024 {
            return Err(BookError::CorruptedFile(format!(
                "文件过大 ({:.1}MB)，无法处理。请选择较小的文件。",
                file_size as f64 / (1024.0 * 1024.0)
            )));
        }

        // 使用tokio异步读取文件
        tokio::fs::read_to_string(file_path)
            .await
            .map_err(|e| BookError::CorruptedFile(format!("读取TXT文件失败: {}", e)))
    }

    /// 读取PDF文件内容
    async fn read_pdf_content(&self, file_path: &Path) -> BookResult<String> {
        let bytes = fs::read(file_path)
            .map_err(|e| BookError::CorruptedFile(format!("读取PDF文件失败: {}", e)))?;

        let content = pdf_extract::extract_text_from_mem(&bytes)
            .map_err(|e| BookError::ParseError(format!("PDF文本提取失败: {}", e)))?;

        Ok(content)
    }

    /// 获取图书章节列表
    pub async fn get_book_chapters(
        &self,
        book_id: &str,
    ) -> BookResult<Vec<crate::models::BookChapter>> {
        // 获取图书信息
        let book = self
            .db
            .get_book_by_id(book_id)
            .map_err(|e| BookError::MetadataExtractionFailed(format!("获取图书信息失败: {}", e)))?
            .ok_or_else(|| BookError::NotFound(format!("图书不存在: {}", book_id)))?;

        let file_path = PathBuf::from(&book.file_path);

        // 检查文件是否存在
        if !file_path.exists() {
            return Err(BookError::NotFound(format!(
                "图书文件不存在: {}",
                file_path.display()
            )));
        }

        // 支持TXT和PDF格式
        match book.format {
            BookFormat::Txt => self.parse_txt_chapters(&file_path).await,
            BookFormat::Pdf => self.parse_pdf_chapters(&file_path).await,
        }
    }

    /// 解析TXT文件章节
    async fn parse_txt_chapters(
        &self,
        file_path: &Path,
    ) -> BookResult<Vec<crate::models::BookChapter>> {
        let content = self.read_txt_content(file_path).await?;
        let mut chapters = Vec::new();

        // 按行分割内容
        let lines: Vec<&str> = content.lines().collect();
        let total_lines = lines.len();

        // 如果文件太大（超过10MB），直接返回单章节避免性能问题
        if content.len() > 10 * 1024 * 1024 {
            let char_count = content.chars().count();
            chapters.push(crate::models::BookChapter {
                index: 0,
                title: "全文".to_string(),
                start_line: 0,
                end_line: total_lines,
                word_count: char_count,
            });
            return Ok(chapters);
        }

        // 预编译正则表达式模式，避免在循环中重复编译
        let chapter_regexes = vec![
            regex::Regex::new(r"第[一二三四五六七八九十百千万\d]+章(.*)")
                .map_err(|e| BookError::ParseError(format!("正则表达式编译失败: {}", e)))?,
            regex::Regex::new(r"第[一二三四五六七八九十百千万\d]+节(.*)")
                .map_err(|e| BookError::ParseError(format!("正则表达式编译失败: {}", e)))?,
            regex::Regex::new(r"Chapter\s+\d+(.*)")
                .map_err(|e| BookError::ParseError(format!("正则表达式编译失败: {}", e)))?,
            regex::Regex::new(r"CHAPTER\s+\d+(.*)")
                .map_err(|e| BookError::ParseError(format!("正则表达式编译失败: {}", e)))?,
            regex::Regex::new(r"第\d+章(.*)")
                .map_err(|e| BookError::ParseError(format!("正则表达式编译失败: {}", e)))?,
            regex::Regex::new(r"第\d+节(.*)")
                .map_err(|e| BookError::ParseError(format!("正则表达式编译失败: {}", e)))?,
            regex::Regex::new(r"^\d+\.(.*)")
                .map_err(|e| BookError::ParseError(format!("正则表达式编译失败: {}", e)))?,
            regex::Regex::new(r"^\d+、(.*)")
                .map_err(|e| BookError::ParseError(format!("正则表达式编译失败: {}", e)))?,
        ];

        let mut chapter_starts = Vec::new();

        // 限制处理的行数，避免处理超大文件时卡死
        let max_lines = std::cmp::min(lines.len(), 50000);

        // 查找章节开始位置
        for (line_index, line) in lines.iter().enumerate().take(max_lines) {
            let line = line.trim();

            // 跳过空行和过长的行
            if line.is_empty() || line.len() > 200 {
                continue;
            }

            // 检查是否匹配章节模式
            for regex in &chapter_regexes {
                if regex.is_match(line) && line.len() < 100 {
                    chapter_starts.push((line_index, line.to_string()));
                    break;
                }
            }
        }

        // 如果没有找到章节，创建一个默认章节
        if chapter_starts.is_empty() {
            let total_lines = lines.len();
            let char_count = content.chars().count();
            chapters.push(crate::models::BookChapter {
                index: 0,
                title: "全文".to_string(),
                start_line: 0,
                end_line: total_lines,
                word_count: char_count,
            });
            return Ok(chapters);
        }

        // 创建章节对象，使用行索引
        for (index, (line_index, title)) in chapter_starts.iter().enumerate() {
            let start_line = *line_index;
            let end_line = if index + 1 < chapter_starts.len() {
                chapter_starts[index + 1].0
            } else {
                lines.len()
            };

            // 计算章节内容的字符数
            let chapter_lines = &lines[start_line..end_line];
            let chapter_char_count: usize =
                chapter_lines.iter().map(|line| line.chars().count()).sum();

            chapters.push(crate::models::BookChapter {
                index,
                title: title.clone(),
                start_line,
                end_line,
                word_count: chapter_char_count,
            });
        }

        Ok(chapters)
    }

    /// 解析PDF文件章节
    async fn parse_pdf_chapters(
        &self,
        file_path: &Path,
    ) -> BookResult<Vec<crate::models::BookChapter>> {
        let content = self.read_pdf_content(file_path).await?;
        let lines: Vec<&str> = content.lines().collect();
        let total_lines = lines.len();

        // PDF章节解析比较复杂，这里简化处理
        // 可以根据页面分割或者文本模式识别
        let chapters = vec![crate::models::BookChapter {
            index: 0,
            title: "全文".to_string(),
            start_line: 0,
            end_line: total_lines,
            word_count: content.chars().count(),
        }];

        Ok(chapters)
    }

    /// 移除章节内容开头的标题行和空行
    fn remove_chapter_title(&self, content: &str, title: &str) -> String {
        let lines: Vec<&str> = content.lines().collect();
        if lines.is_empty() {
            return content.to_string();
        }

        let mut line_index = 0;
        // 跳过标题行后的所有空行
        while line_index < lines.len() && lines[line_index].trim().is_empty() {
            line_index += 1;
        }

        if lines[line_index].contains(title.trim()) {
            line_index += 1;
        }

        while line_index < lines.len() && lines[line_index].trim().is_empty() {
            line_index += 1;
        }

        // 返回处理后的内容
        if line_index < lines.len() {
            lines[line_index..].join("\n")
        } else {
            String::new()
        }
    }

    /// 获取指定章节内容
    pub async fn get_chapter_content(
        &self,
        book_id: &str,
        chapter_index: usize,
    ) -> BookResult<crate::models::ChapterContent> {
        let chapters = self.get_book_chapters(book_id).await?;

        let chapter = chapters.get(chapter_index).ok_or_else(|| {
            BookError::ChapterParseError(format!("章节不存在: {}", chapter_index))
        })?;

        let full_content = self.get_book_content(book_id).await?;

        // 按行分割内容
        let lines: Vec<&str> = full_content.lines().collect();
        let total_lines = lines.len();

        // 确保行索引在有效范围内
        let start_line = std::cmp::min(chapter.start_line, total_lines);
        let end_line = std::cmp::min(chapter.end_line, total_lines);

        let mut chapter_content = if start_line < end_line && start_line < total_lines {
            lines[start_line..end_line].join("\n")
        } else {
            String::new()
        };

        // 去掉章节标题行（如果存在）
        chapter_content = self.remove_chapter_title(&chapter_content, &chapter.title);

        // 将内容按行分割成Vec<String>，过滤掉空行
        let content_lines: Vec<String> = chapter_content
            .lines()
            .map(|line| line.to_string())
            .filter(|line| !line.trim().is_empty()) // 过滤空行
            .collect();

        // 计算预估阅读时间（假设每分钟阅读300字）
        let content_char_count = content_lines
            .iter()
            .map(|line| line.chars().count())
            .sum::<usize>();
        let estimated_reading_time = (content_char_count as f64 / 300.0).ceil() as u32;

        Ok(crate::models::ChapterContent {
            chapter_index,
            title: chapter.title.clone(),
            content: content_lines,
            estimated_reading_time,
        })
    }

    /// 获取分页内容
    pub async fn get_paginated_content(
        &self,
        book_id: &str,
        page_size: usize,
        page_number: usize,
    ) -> BookResult<crate::models::PaginatedContent> {
        let full_content = self.get_book_content(book_id).await?;
        let total_chars = full_content.chars().count();
        let total_pages = (total_chars + page_size - 1) / page_size; // 向上取整

        if page_number >= total_pages {
            return Err(BookError::ChapterParseError(format!(
                "页面不存在: {}, 总页数: {}",
                page_number, total_pages
            )));
        }

        let start_position = page_number * page_size;
        let end_position = std::cmp::min(start_position + page_size, total_chars);

        // 使用字符索引而不是字节索引来确保正确处理Unicode字符
        let chars: Vec<char> = full_content.chars().collect();
        let page_content: String = chars[start_position..end_position].iter().collect();

        Ok(crate::models::PaginatedContent {
            page_number,
            page_size,
            total_pages,
            content: page_content,
            start_position,
            end_position,
            has_next_page: page_number + 1 < total_pages,
            has_previous_page: page_number > 0,
        })
    }
}
