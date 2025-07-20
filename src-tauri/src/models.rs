use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// 图书格式枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum BookFormat {
    Txt,
    Epub,
    Pdf,
}

impl BookFormat {
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "txt" => Some(BookFormat::Txt),
            "epub" => Some(BookFormat::Epub),
            "pdf" => Some(BookFormat::Pdf),
            _ => None,
        }
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            BookFormat::Txt => "txt",
            BookFormat::Epub => "epub",
            BookFormat::Pdf => "pdf",
        }
    }
}

/// 图书数据模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Book {
    pub id: String,
    pub title: String,
    pub author: Option<String>,
    pub file_path: String,
    pub format: BookFormat,
    pub file_size: i64,
    pub created_at: DateTime<Utc>,
    pub last_read: Option<DateTime<Utc>>,
    pub reading_progress: f64,
    pub total_chapters: i32,
}

impl Book {
    pub fn new(
        title: String,
        author: Option<String>,
        file_path: PathBuf,
        format: BookFormat,
        file_size: i64,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            author,
            file_path: file_path.to_string_lossy().to_string(),
            format,
            file_size,
            created_at: Utc::now(),
            last_read: None,
            reading_progress: 0.0,
            total_chapters: 0,
        }
    }
}

/// 书签数据模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bookmark {
    pub id: i64,
    pub book_id: String,
    pub position: i64,
    pub chapter_index: Option<i32>,
    pub note: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// 阅读历史数据模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadingHistory {
    pub id: i64,
    pub book_id: String,
    pub read_at: DateTime<Utc>,
    pub duration: Option<i32>, // 阅读时长(秒)
}

/// 书源信息数据模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookSourceInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: Option<String>,
    pub description: Option<String>,
    pub base_url: String,
    pub enabled: bool,
    pub plugin_path: String,
    pub installed_at: DateTime<Utc>,
}

/// 搜索结果数据模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub title: String,
    pub author: Option<String>,
    pub description: Option<String>,
    pub source_id: String,
    pub book_url: String,
    pub cover_url: Option<String>,
}

/// 章节信息数据模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChapterInfo {
    pub title: String,
    pub url: String,
    pub index: usize,
}

/// HTML元素数据模型（用于插件HTML解析）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HtmlElement {
    pub text: String,
    pub href: Option<String>,
    pub src: Option<String>,
    pub class: Option<String>,
    pub id: Option<String>,
}

/// 应用设置数据模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub theme: String,
    pub language: String,
    pub auto_save: bool,
    pub data_path: String,
    pub reader: ReaderSettings,
    pub book_sources: BookSourceSettings,
    pub download: DownloadSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReaderSettings {
    pub font_size: i32,
    pub font_family: String,
    pub line_height: f64,
    pub page_margin: i32,
    pub background_color: String,
    pub text_color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookSourceSettings {
    pub enabled: bool,
    pub auto_update: bool,
    pub enabled_sources: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadSettings {
    pub concurrent: i32,
    pub timeout: i32,
    pub retry_count: i32,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: "light".to_string(),
            language: "zh-CN".to_string(),
            auto_save: true,
            data_path: "./data".to_string(),
            reader: ReaderSettings {
                font_size: 16,
                font_family: "system".to_string(),
                line_height: 1.6,
                page_margin: 20,
                background_color: "#ffffff".to_string(),
                text_color: "#333333".to_string(),
            },
            book_sources: BookSourceSettings {
                enabled: true,
                auto_update: false,
                enabled_sources: Vec::new(),
            },
            download: DownloadSettings {
                concurrent: 3,
                timeout: 30000,
                retry_count: 3,
            },
        }
    }
}

/// 图书库统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryStats {
    pub total_books: usize,
    pub total_size: i64,
    pub format_counts: HashMap<BookFormat, usize>,
    pub books_with_progress: usize,
    pub average_progress: f64,
}

/// 图书章节信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookChapter {
    pub index: usize,
    pub title: String,
    pub start_position: usize,
    pub end_position: usize,
    pub word_count: usize,
}

/// 章节内容
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChapterContent {
    pub chapter_index: usize,
    pub title: String,
    pub content: String,
    pub word_count: usize,
    pub estimated_reading_time: u32, // 预估阅读时间（分钟）
}

/// 分页内容
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedContent {
    pub page_number: usize,
    pub page_size: usize,
    pub total_pages: usize,
    pub content: String,
    pub start_position: usize,
    pub end_position: usize,
    pub has_next_page: bool,
    pub has_previous_page: bool,
}
