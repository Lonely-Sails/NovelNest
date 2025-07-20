use thiserror::Error;

/// 应用主错误类型
#[derive(Debug, Error)]
pub enum AppError {
    #[error("图书错误: {0}")]
    Book(#[from] BookError),
    
    #[error("书源错误: {0}")]
    BookSource(#[from] BookSourceError),
    
    #[error("文件系统错误: {0}")]
    FileSystem(#[from] std::io::Error),
    
    #[error("数据库错误: {0}")]
    Database(#[from] rusqlite::Error),
    
    #[error("网络错误: {0}")]
    Network(#[from] reqwest::Error),
    
    #[error("序列化错误: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("UUID错误: {0}")]
    Uuid(#[from] uuid::Error),
    
    #[error("通用错误: {0}")]
    Generic(String),
}

/// 图书相关错误类型
#[derive(Debug, Error)]
pub enum BookError {
    #[error("不支持的文件格式: {0}")]
    UnsupportedFormat(String),
    
    #[error("文件损坏或无法读取: {0}")]
    CorruptedFile(String),
    
    #[error("图书不存在: {0}")]
    NotFound(String),
    
    #[error("重复的图书: {0}")]
    Duplicate(String),
    
    #[error("文件解析错误: {0}")]
    ParseError(String),
    
    #[error("元数据提取失败: {0}")]
    MetadataExtractionFailed(String),
    
    #[error("章节解析失败: {0}")]
    ChapterParseError(String),
    
    #[error("无效的阅读进度: {0}")]
    InvalidProgress(String),
}

/// 书源相关错误类型
#[derive(Debug, Error)]
pub enum BookSourceError {
    #[error("书源加载失败: {0}")]
    LoadFailed(String),
    
    #[error("网络请求错误: {0}")]
    NetworkError(String),
    
    #[error("解析错误: {0}")]
    ParseError(String),
    
    #[error("书源不存在: {0}")]
    NotFound(String),
    
    #[error("书源格式无效: {0}")]
    InvalidFormat(String),
    
    #[error("插件执行错误: {0}")]
    PluginExecutionError(String),
    
    #[error("JavaScript运行时错误: {0}")]
    JavaScriptError(String),
    
    #[error("HTTP请求失败: {0}")]
    HttpRequestFailed(String),
    
    #[error("HTML解析失败: {0}")]
    HtmlParseError(String),
}

/// 数据库相关错误类型
#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("连接失败: {0}")]
    ConnectionFailed(String),
    
    #[error("查询执行失败: {0}")]
    QueryFailed(String),
    
    #[error("事务失败: {0}")]
    TransactionFailed(String),
    
    #[error("数据迁移失败: {0}")]
    MigrationFailed(String),
    
    #[error("数据完整性错误: {0}")]
    IntegrityError(String),
}

/// 文件系统相关错误类型
#[derive(Debug, Error)]
pub enum FileSystemError {
    #[error("文件不存在: {0}")]
    FileNotFound(String),
    
    #[error("权限不足: {0}")]
    PermissionDenied(String),
    
    #[error("磁盘空间不足")]
    DiskSpaceFull,
    
    #[error("文件已存在: {0}")]
    FileAlreadyExists(String),
    
    #[error("目录创建失败: {0}")]
    DirectoryCreationFailed(String),
}

/// 结果类型别名
pub type AppResult<T> = Result<T, AppError>;
pub type BookResult<T> = Result<T, BookError>;
pub type BookSourceResult<T> = Result<T, BookSourceError>;
pub type DatabaseResult<T> = Result<T, DatabaseError>;
pub type FileSystemResult<T> = Result<T, FileSystemError>;

impl From<AppError> for String {
    fn from(error: AppError) -> Self {
        error.to_string()
    }
}

impl From<BookError> for String {
    fn from(error: BookError) -> Self {
        error.to_string()
    }
}

impl From<BookSourceError> for String {
    fn from(error: BookSourceError) -> Self {
        error.to_string()
    }
}

impl From<DatabaseError> for String {
    fn from(error: DatabaseError) -> Self {
        error.to_string()
    }
}

impl From<FileSystemError> for String {
    fn from(error: FileSystemError) -> Self {
        error.to_string()
    }
}