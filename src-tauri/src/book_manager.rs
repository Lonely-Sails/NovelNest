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
            .ok_or_else(|| {
                BookError::UnsupportedFormat("无法确定文件格式".to_string())
            })?;

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
            BookFormat::Epub => self.extract_epub_metadata(file_path).await,
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
                return Some(line.replace("作者：", "").replace("作者:", "").trim().to_string());
            }
            
            if line.starts_with("Author:") || line.starts_with("author:") {
                return Some(line.replace("Author:", "").replace("author:", "").trim().to_string());
            }
            
            // 检查是否包含"著"字
            if line.contains("著") && line.len() < 50 {
                // 简单的启发式规则：如果行很短且包含"著"，可能是作者信息
                return Some(line.to_string());
            }
        }

        None
    }

    /// 提取EPUB文件元数据
    async fn extract_epub_metadata(&self, file_path: &Path) -> BookResult<(String, Option<String>)> {
        let doc = epub::doc::EpubDoc::new(file_path).map_err(|e| {
            BookError::ParseError(format!("EPUB文件解析失败: {}", e))
        })?;

        let title = doc
            .mdata("title")
            .map(|s| s.to_string())
            .unwrap_or_else(|| {
                file_path
                    .file_stem()
                    .and_then(|name| name.to_str())
                    .unwrap_or("未知标题")
                    .to_string()
            });

        let author = doc.mdata("creator").map(|s| s.to_string());

        Ok((title, author))
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
            b.last_read.unwrap_or_default().cmp(&a.last_read.unwrap_or_default())
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
        let mut total_progress = 0.0;

        for book in &all_books {
            *format_counts.entry(book.format.clone()).or_insert(0) += 1;
            total_size += book.file_size;
            if book.reading_progress > 0.0 {
                books_with_progress += 1;
                total_progress += book.reading_progress;
            }
        }

        let average_progress = if books_with_progress > 0 {
            total_progress / books_with_progress as f64
        } else {
            0.0
        };

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

    /// 更新阅读进度
    pub async fn update_reading_progress(&self, book_id: &str, progress: f64) -> BookResult<()> {
        self.db
            .update_reading_progress(book_id, progress)
            .map_err(|e| BookError::MetadataExtractionFailed(format!("更新阅读进度失败: {}", e)))
    }

    /// 保存阅读进度（带自动保存逻辑）
    pub async fn save_reading_progress(&self, book_id: &str, progress: f64) -> BookResult<()> {
        // 验证进度值范围
        if !(0.0..=1.0).contains(&progress) {
            return Err(BookError::InvalidProgress(format!(
                "阅读进度必须在0.0到1.0之间，当前值: {}", progress
            )));
        }

        // 验证图书是否存在
        let book = self
            .db
            .get_book_by_id(book_id)
            .map_err(|e| BookError::MetadataExtractionFailed(format!("获取图书信息失败: {}", e)))?
            .ok_or_else(|| BookError::NotFound(format!("图书不存在: {}", book_id)))?;

        // 只有当进度有显著变化时才保存（避免频繁写入）
        let progress_diff = (progress - book.reading_progress).abs();
        if progress_diff >= 0.01 || progress == 1.0 || progress == 0.0 {
            self.db
                .update_reading_progress(book_id, progress)
                .map_err(|e| BookError::MetadataExtractionFailed(format!("保存阅读进度失败: {}", e)))?;
        }

        Ok(())
    }

    /// 获取阅读进度
    pub async fn get_reading_progress(&self, book_id: &str) -> BookResult<f64> {
        let book = self
            .db
            .get_book_by_id(book_id)
            .map_err(|e| BookError::MetadataExtractionFailed(format!("获取图书信息失败: {}", e)))?
            .ok_or_else(|| BookError::NotFound(format!("图书不存在: {}", book_id)))?;

        Ok(book.reading_progress)
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
                "书签位置不能为负数: {}", position
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
        let bookmark_id = self.db
            .insert_bookmark(&bookmark)
            .map_err(|e| BookError::MetadataExtractionFailed(format!("添加书签失败: {}", e)))?;

        Ok(bookmark_id)
    }

    /// 获取图书的所有书签
    pub async fn get_bookmarks(&self, book_id: &str) -> BookResult<Vec<crate::models::Bookmark>> {
        // 验证图书是否存在
        let _book = self
            .db
            .get_book_by_id(book_id)
            .map_err(|e| BookError::MetadataExtractionFailed(format!("获取图书信息失败: {}", e)))?
            .ok_or_else(|| BookError::NotFound(format!("图书不存在: {}", book_id)))?;

        // 获取书签列表
        let bookmarks = self.db
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
    pub async fn get_bookmark_by_id(&self, bookmark_id: i64) -> BookResult<Option<crate::models::Bookmark>> {
        // 获取所有书签，然后找到指定ID的书签
        // 这里可以优化为直接从数据库查询单个书签
        let all_bookmarks = self.db
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

        // 根据格式读取内容
        match book.format {
            BookFormat::Txt => self.read_txt_content(&file_path).await,
            BookFormat::Epub => self.read_epub_content(&file_path).await,
            BookFormat::Pdf => self.read_pdf_content(&file_path).await,
        }
    }

    /// 读取TXT文件内容
    async fn read_txt_content(&self, file_path: &Path) -> BookResult<String> {
        fs::read_to_string(file_path).map_err(|e| {
            BookError::CorruptedFile(format!("读取TXT文件失败: {}", e))
        })
    }

    /// 读取EPUB文件内容
    async fn read_epub_content(&self, file_path: &Path) -> BookResult<String> {
        let mut doc = epub::doc::EpubDoc::new(file_path).map_err(|e| {
            BookError::ParseError(format!("EPUB文件解析失败: {}", e))
        })?;

        let mut content = String::new();

        // 获取所有章节内容
        for i in 0..doc.get_num_pages() {
            if !doc.set_current_page(i) {
                return Err(BookError::ChapterParseError(format!("设置EPUB页面失败: {}", i)));
            }

            match doc.get_current_str() {
                Some((chapter_content, _)) => {
                    content.push_str(&chapter_content);
                    content.push('\n');
                }
                None => {
                    return Err(BookError::ChapterParseError(format!("获取EPUB章节内容失败: {}", i)));
                }
            }
        }

        Ok(content)
    }

    /// 读取PDF文件内容
    async fn read_pdf_content(&self, file_path: &Path) -> BookResult<String> {
        let bytes = fs::read(file_path).map_err(|e| {
            BookError::CorruptedFile(format!("读取PDF文件失败: {}", e))
        })?;

        let content = pdf_extract::extract_text_from_mem(&bytes).map_err(|e| {
            BookError::ParseError(format!("PDF文本提取失败: {}", e))
        })?;

        Ok(content)
    }

    /// 获取图书章节列表
    pub async fn get_book_chapters(&self, book_id: &str) -> BookResult<Vec<crate::models::BookChapter>> {
        // 获取图书信息
        let book = self
            .db
            .get_book_by_id(book_id)
            .map_err(|e| BookError::MetadataExtractionFailed(format!("获取图书信息失败: {}", e)))?
            .ok_or_else(|| BookError::NotFound(format!("图书不存在: {}", book_id)))?;

        let file_path = PathBuf::from(&book.file_path);

        // 根据格式解析章节
        match book.format {
            BookFormat::Txt => self.parse_txt_chapters(&file_path).await,
            BookFormat::Epub => self.parse_epub_chapters(&file_path).await,
            BookFormat::Pdf => self.parse_pdf_chapters(&file_path).await,
        }
    }

    /// 解析TXT文件章节
    async fn parse_txt_chapters(&self, file_path: &Path) -> BookResult<Vec<crate::models::BookChapter>> {
        let content = self.read_txt_content(file_path).await?;
        let mut chapters = Vec::new();

        // 常见的章节标题模式
        let chapter_patterns = [
            r"第[一二三四五六七八九十百千万\d]+章",
            r"第[一二三四五六七八九十百千万\d]+节",
            r"Chapter\s+\d+",
            r"CHAPTER\s+\d+",
            r"第\d+章",
            r"第\d+节",
            r"^\d+\.",
            r"^\d+、",
        ];

        let lines: Vec<&str> = content.lines().collect();
        let mut chapter_starts = Vec::new();

        // 查找章节开始位置
        for (line_index, line) in lines.iter().enumerate() {
            let line = line.trim();
            
            // 检查是否匹配章节模式
            for pattern in &chapter_patterns {
                if let Ok(regex) = regex::Regex::new(pattern) {
                    if regex.is_match(line) && line.len() < 100 {
                        chapter_starts.push((line_index, line.to_string()));
                        break;
                    }
                }
            }
        }

        // 如果没有找到章节，创建一个默认章节
        if chapter_starts.is_empty() {
            chapters.push(crate::models::BookChapter {
                index: 0,
                title: "全文".to_string(),
                start_position: 0,
                end_position: content.len(),
                word_count: content.chars().count(),
            });
            return Ok(chapters);
        }

        // 创建章节对象
        for (i, (line_index, title)) in chapter_starts.iter().enumerate() {
            let start_line = *line_index;
            let end_line = if i + 1 < chapter_starts.len() {
                chapter_starts[i + 1].0
            } else {
                lines.len()
            };

            let chapter_lines = &lines[start_line..end_line];
            let chapter_content = chapter_lines.join("\n");
            
            let start_pos = if i == 0 {
                0
            } else {
                lines[..start_line].join("\n").len() + 1
            };
            
            let end_pos = start_pos + chapter_content.len();

            chapters.push(crate::models::BookChapter {
                index: i,
                title: title.clone(),
                start_position: start_pos,
                end_position: end_pos,
                word_count: chapter_content.chars().count(),
            });
        }

        Ok(chapters)
    }

    /// 解析EPUB文件章节
    async fn parse_epub_chapters(&self, file_path: &Path) -> BookResult<Vec<crate::models::BookChapter>> {
        let mut doc = epub::doc::EpubDoc::new(file_path).map_err(|e| {
            BookError::ParseError(format!("EPUB文件解析失败: {}", e))
        })?;

        let mut chapters = Vec::new();
        let mut current_position = 0;

        // 获取所有章节
        for i in 0..doc.get_num_pages() {
            if !doc.set_current_page(i) {
                continue;
            }

            if let Some((chapter_content, _)) = doc.get_current_str() {
                let title = doc.get_current_id()
                    .map(|_id| format!("第{}章", i + 1))
                    .unwrap_or_else(|| format!("第{}章", i + 1));

                let word_count = chapter_content.chars().count();
                let end_position = current_position + chapter_content.len();

                chapters.push(crate::models::BookChapter {
                    index: i,
                    title,
                    start_position: current_position,
                    end_position,
                    word_count,
                });

                current_position = end_position + 1; // +1 for newline
            }
        }

        Ok(chapters)
    }

    /// 解析PDF文件章节
    async fn parse_pdf_chapters(&self, file_path: &Path) -> BookResult<Vec<crate::models::BookChapter>> {
        let content = self.read_pdf_content(file_path).await?;
        
        // PDF章节解析比较复杂，这里简化处理
        // 可以根据页面分割或者文本模式识别
        let chapters = vec![crate::models::BookChapter {
            index: 0,
            title: "全文".to_string(),
            start_position: 0,
            end_position: content.len(),
            word_count: content.chars().count(),
        }];

        Ok(chapters)
    }

    /// 获取指定章节内容
    pub async fn get_chapter_content(
        &self,
        book_id: &str,
        chapter_index: usize,
    ) -> BookResult<crate::models::ChapterContent> {
        let chapters = self.get_book_chapters(book_id).await?;
        
        let chapter = chapters.get(chapter_index)
            .ok_or_else(|| BookError::ChapterParseError(format!("章节不存在: {}", chapter_index)))?;

        let full_content = self.get_book_content(book_id).await?;
        
        // 使用字符索引而不是字节索引来避免UTF-8边界问题
        let chars: Vec<char> = full_content.chars().collect();
        let total_chars = chars.len();
        
        let chapter_content = if chapter.end_position <= total_chars {
            chars[chapter.start_position..chapter.end_position].iter().collect()
        } else if chapter.start_position < total_chars {
            chars[chapter.start_position..].iter().collect()
        } else {
            String::new()
        };

        // 计算预估阅读时间（假设每分钟阅读300字）
        let estimated_reading_time = (chapter.word_count as f64 / 300.0).ceil() as u32;

        Ok(crate::models::ChapterContent {
            chapter_index,
            title: chapter.title.clone(),
            content: chapter_content,
            word_count: chapter.word_count,
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
                "页面不存在: {}, 总页数: {}", page_number, total_pages
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

    // 以下是为集成测试添加的测试方法

    /// 测试用：导入图书
    #[cfg(test)]
    pub async fn test_import_book(file_path: PathBuf) -> BookResult<Book> {
        use crate::database::DatabaseManager;
        use std::sync::Arc;
        use tempfile::TempDir;

        // 创建临时目录和数据库
        let temp_dir = TempDir::new().expect("创建临时目录失败");
        let db_path = temp_dir.path().join("test.db");
        let db = Arc::new(DatabaseManager::new(&db_path).expect("创建测试数据库失败"));
        
        // 创建图书管理器
        let storage_path = temp_dir.path().join("books");
        std::fs::create_dir_all(&storage_path).expect("创建图书存储目录失败");
        let book_manager = BookManager::new(db, storage_path);
        
        // 导入图书
        book_manager.import_book(file_path).await
    }

    /// 测试用：获取图书章节
    #[cfg(test)]
    pub async fn test_get_book_chapters(book_id: &str) -> BookResult<Vec<crate::models::BookChapter>> {
        use crate::database::DatabaseManager;
        use std::sync::Arc;
        use tempfile::TempDir;

        // 创建临时目录和数据库
        let temp_dir = TempDir::new().expect("创建临时目录失败");
        let db_path = temp_dir.path().join("test.db");
        let db = Arc::new(DatabaseManager::new(&db_path).expect("创建测试数据库失败"));
        
        // 创建图书管理器
        let storage_path = temp_dir.path().join("books");
        std::fs::create_dir_all(&storage_path).expect("创建图书存储目录失败");
        let book_manager = BookManager::new(db, storage_path);
        
        // 获取章节
        book_manager.get_book_chapters(book_id).await
    }

    /// 测试用：获取章节内容
    #[cfg(test)]
    pub async fn test_get_chapter_content(book_id: &str, chapter_index: usize) -> BookResult<crate::models::ChapterContent> {
        use crate::database::DatabaseManager;
        use std::sync::Arc;
        use tempfile::TempDir;

        // 创建临时目录和数据库
        let temp_dir = TempDir::new().expect("创建临时目录失败");
        let db_path = temp_dir.path().join("test.db");
        let db = Arc::new(DatabaseManager::new(&db_path).expect("创建测试数据库失败"));
        
        // 创建图书管理器
        let storage_path = temp_dir.path().join("books");
        std::fs::create_dir_all(&storage_path).expect("创建图书存储目录失败");
        let book_manager = BookManager::new(db, storage_path);
        
        // 获取章节内容
        book_manager.get_chapter_content(book_id, chapter_index).await
    }

    /// 测试用：更新阅读进度
    #[cfg(test)]
    pub async fn test_update_reading_progress(book_id: &str, progress: f64) -> BookResult<()> {
        use crate::database::DatabaseManager;
        use std::sync::Arc;
        use tempfile::TempDir;

        // 创建临时目录和数据库
        let temp_dir = TempDir::new().expect("创建临时目录失败");
        let db_path = temp_dir.path().join("test.db");
        let db = Arc::new(DatabaseManager::new(&db_path).expect("创建测试数据库失败"));
        
        // 创建图书管理器
        let storage_path = temp_dir.path().join("books");
        std::fs::create_dir_all(&storage_path).expect("创建图书存储目录失败");
        let book_manager = BookManager::new(db, storage_path);
        
        // 更新阅读进度
        book_manager.update_reading_progress(book_id, progress).await
    }

    /// 测试用：获取阅读进度
    #[cfg(test)]
    pub async fn test_get_reading_progress(book_id: &str) -> BookResult<f64> {
        use crate::database::DatabaseManager;
        use std::sync::Arc;
        use tempfile::TempDir;

        // 创建临时目录和数据库
        let temp_dir = TempDir::new().expect("创建临时目录失败");
        let db_path = temp_dir.path().join("test.db");
        let db = Arc::new(DatabaseManager::new(&db_path).expect("创建测试数据库失败"));
        
        // 创建图书管理器
        let storage_path = temp_dir.path().join("books");
        std::fs::create_dir_all(&storage_path).expect("创建图书存储目录失败");
        let book_manager = BookManager::new(db, storage_path);
        
        // 获取阅读进度
        book_manager.get_reading_progress(book_id).await
    }
}