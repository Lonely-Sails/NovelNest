pub mod book_manager;
pub mod book_source_manager;
pub mod database;
pub mod errors;
pub mod models;
use book_manager::BookManager;
use book_source_manager::BookSourceManager;
use database::DatabaseManager;
use errors::AppResult;
use models::{Book, BookFormat, LibraryStats};
use std::path::PathBuf;
use std::sync::Arc;
use tauri::{Manager, State};

// 应用状态
pub struct AppState {
    pub db: Arc<DatabaseManager>,
    pub book_manager: Arc<BookManager>,
    pub book_source_manager: Arc<tokio::sync::Mutex<BookSourceManager>>,
    pub settings: Arc<tokio::sync::RwLock<models::AppSettings>>,
}

// 图书管理相关命令

/// 导入单个图书文件
#[tauri::command]
async fn import_book(file_path: String, state: State<'_, AppState>) -> Result<Book, String> {
    let path = PathBuf::from(file_path);
    state
        .book_manager
        .import_book(path)
        .await
        .map_err(|e| e.to_string())
}

/// 批量导入文件夹中的图书
#[tauri::command]
async fn import_folder(
    folder_path: String,
    state: State<'_, AppState>,
) -> Result<Vec<Book>, String> {
    let path = PathBuf::from(folder_path);
    state
        .book_manager
        .import_folder(path)
        .await
        .map_err(|e| e.to_string())
}

/// 获取所有图书
#[tauri::command]
async fn get_books(state: State<'_, AppState>) -> Result<Vec<Book>, String> {
    state
        .book_manager
        .get_all_books()
        .await
        .map_err(|e| e.to_string())
}

/// 搜索图书
#[tauri::command]
async fn search_books(query: String, state: State<'_, AppState>) -> Result<Vec<Book>, String> {
    state
        .book_manager
        .search_books(&query)
        .await
        .map_err(|e| e.to_string())
}

/// 删除图书
#[tauri::command]
async fn delete_book(book_id: String, state: State<'_, AppState>) -> Result<(), String> {
    state
        .book_manager
        .delete_book(&book_id)
        .await
        .map_err(|e| e.to_string())
}

/// 按格式过滤图书
#[tauri::command]
async fn filter_books_by_format(
    format: String,
    state: State<'_, AppState>,
) -> Result<Vec<Book>, String> {
    let book_format = match format.to_lowercase().as_str() {
        "txt" => BookFormat::Txt,
        "pdf" => BookFormat::Pdf,
        _ => return Err("不支持的文件格式".to_string()),
    };

    state
        .book_manager
        .filter_books_by_format(&book_format)
        .await
        .map_err(|e| e.to_string())
}

/// 按作者过滤图书
#[tauri::command]
async fn filter_books_by_author(
    author: String,
    state: State<'_, AppState>,
) -> Result<Vec<Book>, String> {
    state
        .book_manager
        .filter_books_by_author(&author)
        .await
        .map_err(|e| e.to_string())
}

/// 获取最近添加的图书
#[tauri::command]
async fn get_recent_books(limit: usize, state: State<'_, AppState>) -> Result<Vec<Book>, String> {
    state
        .book_manager
        .get_recent_books(limit)
        .await
        .map_err(|e| e.to_string())
}

/// 获取最近阅读的图书
#[tauri::command]
async fn get_recently_read_books(
    limit: usize,
    state: State<'_, AppState>,
) -> Result<Vec<Book>, String> {
    state
        .book_manager
        .get_recently_read_books(limit)
        .await
        .map_err(|e| e.to_string())
}

/// 获取图书库统计信息
#[tauri::command]
async fn get_library_stats(state: State<'_, AppState>) -> Result<LibraryStats, String> {
    state
        .book_manager
        .get_library_stats()
        .await
        .map_err(|e| e.to_string())
}

/// 保存阅读进度（章节和行索引）
#[tauri::command]
async fn save_reading_progress(
    book_id: String,
    current_chapter: i32,
    current_line_index: i32,
    state: State<'_, AppState>,
) -> Result<(), String> {
    state
        .book_manager
        .save_reading_progress(&book_id, current_chapter, current_line_index)
        .await
        .map_err(|e| e.to_string())
}

/// 获取阅读进度（章节和行索引）
#[tauri::command]
async fn get_reading_progress(book_id: String, state: State<'_, AppState>) -> Result<(i32, i32), String> {
    state
        .book_manager
        .get_reading_progress(&book_id)
        .await
        .map_err(|e| e.to_string())
}

/// 添加书签
#[tauri::command]
async fn add_bookmark(
    book_id: String,
    position: i64,
    chapter_index: Option<i32>,
    note: Option<String>,
    state: State<'_, AppState>,
) -> Result<i64, String> {
    println!(
        "开始添加书签: book_id={}, position={}, chapter_index={:?}, note={:?}",
        book_id, position, chapter_index, note
    );

    // 输入验证
    if book_id.is_empty() {
        eprintln!("book_id 为空");
        return Err("book_id 不能为空".to_string());
    }

    match state
        .book_manager
        .add_bookmark(&book_id, position, chapter_index, note)
        .await
    {
        Ok(bookmark_id) => {
            println!("成功添加书签: bookmark_id={}", bookmark_id);
            Ok(bookmark_id)
        }
        Err(e) => {
            eprintln!("添加书签失败: {}", e);
            Err(format!("添加书签失败: {}", e))
        }
    }
}

/// 获取图书的所有书签
#[tauri::command]
async fn get_bookmarks(
    book_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<models::Bookmark>, String> {
    println!("开始获取书签: book_id={}", book_id);

    // 输入验证
    if book_id.is_empty() {
        eprintln!("book_id 为空");
        return Err("book_id 不能为空".to_string());
    }

    // 添加超时机制，避免长时间阻塞
    let timeout_duration = std::time::Duration::from_secs(10);

    match tokio::time::timeout(timeout_duration, state.book_manager.get_bookmarks(&book_id)).await {
        Ok(result) => match result {
            Ok(bookmarks) => {
                println!("成功获取 {} 个书签", bookmarks.len());
                Ok(bookmarks)
            }
            Err(e) => {
                eprintln!("获取书签失败: {}", e);
                Err(format!("获取书签失败: {}", e))
            }
        },
        Err(_) => {
            eprintln!("获取书签超时: book_id={}", book_id);
            Err("获取书签超时，请重试".to_string())
        }
    }
}

/// 删除书签
#[tauri::command]
async fn delete_bookmark(bookmark_id: i64, state: State<'_, AppState>) -> Result<(), String> {
    state
        .book_manager
        .delete_bookmark(bookmark_id)
        .await
        .map_err(|e| e.to_string())
}

/// 获取图书内容
#[tauri::command]
async fn get_book_content(book_id: String, state: State<'_, AppState>) -> Result<String, String> {
    state
        .book_manager
        .get_book_content(&book_id)
        .await
        .map_err(|e| e.to_string())
}

/// 获取图书章节列表
#[tauri::command]
async fn get_book_chapters(
    book_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<models::BookChapter>, String> {
    // 添加超时机制，避免长时间阻塞
    let timeout_duration = std::time::Duration::from_secs(30);

    match tokio::time::timeout(
        timeout_duration,
        state.book_manager.get_book_chapters(&book_id),
    )
    .await
    {
        Ok(result) => result.map_err(|e| e.to_string()),
        Err(_) => Err("获取章节列表超时，请检查文件是否过大或损坏".to_string()),
    }
}

/// 获取指定章节内容
#[tauri::command]
async fn get_chapter_content(
    book_id: String,
    chapter_index: usize,
    state: State<'_, AppState>,
) -> Result<models::ChapterContent, String> {
    state
        .book_manager
        .get_chapter_content(&book_id, chapter_index)
        .await
        .map_err(|e| e.to_string())
}

/// 获取分页内容
#[tauri::command]
async fn get_paginated_content(
    book_id: String,
    page_size: usize,
    page_number: usize,
    state: State<'_, AppState>,
) -> Result<models::PaginatedContent, String> {
    state
        .book_manager
        .get_paginated_content(&book_id, page_size, page_number)
        .await
        .map_err(|e| e.to_string())
}

/// 扫描文件夹中的电子书文件
#[tauri::command]
async fn scan_folder_for_books(folder_path: String) -> Result<Vec<String>, String> {
    use std::fs;
    use std::path::Path;

    let folder = Path::new(&folder_path);
    if !folder.exists() || !folder.is_dir() {
        return Err("文件夹不存在或不是有效目录".to_string());
    }

    let supported_extensions = ["txt", "epub", "pdf", "mobi", "azw3"];
    let mut book_files = Vec::new();

    fn scan_directory(
        dir: &Path,
        files: &mut Vec<String>,
        extensions: &[&str],
    ) -> Result<(), String> {
        let entries = fs::read_dir(dir).map_err(|e| format!("读取目录失败: {}", e))?;

        for entry in entries {
            let entry = entry.map_err(|e| format!("读取目录项失败: {}", e))?;
            let path = entry.path();

            if path.is_dir() {
                // 递归扫描子目录
                scan_directory(&path, files, extensions)?;
            } else if path.is_file() {
                if let Some(extension) = path.extension() {
                    if let Some(ext_str) = extension.to_str() {
                        if extensions.contains(&ext_str.to_lowercase().as_str()) {
                            files.push(path.to_string_lossy().to_string());
                        }
                    }
                }
            }
        }

        Ok(())
    }

    scan_directory(folder, &mut book_files, &supported_extensions)
        .map_err(|e| format!("扫描文件夹失败: {}", e))?;

    Ok(book_files)
}

/// 获取文件信息
#[tauri::command]
async fn get_file_info(file_path: String) -> Result<models::FileInfo, String> {
    use std::fs;
    use std::path::Path;

    let path = Path::new(&file_path);
    if !path.exists() {
        return Err("文件不存在".to_string());
    }

    let metadata = fs::metadata(path).map_err(|e| format!("获取文件信息失败: {}", e))?;

    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("未知文件")
        .to_string();

    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_string();

    Ok(models::FileInfo {
        name,
        size: metadata.len(),
        extension,
        path: file_path,
        is_file: metadata.is_file(),
        is_dir: metadata.is_dir(),
        modified: metadata
            .modified()
            .map(|time| {
                time.duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
            })
            .unwrap_or(0),
    })
}

/// 检查图书是否已存在
#[tauri::command]
async fn check_book_exists(file_path: String, state: State<'_, AppState>) -> Result<bool, String> {
    let books = state
        .book_manager
        .get_all_books()
        .await
        .map_err(|e| e.to_string())?;

    let exists = books.iter().any(|book| book.file_path == file_path);
    Ok(exists)
}

/// 批量导入图书（带进度回调）
#[tauri::command]
async fn batch_import_books(
    file_paths: Vec<String>,
    skip_existing: bool,
    state: State<'_, AppState>,
) -> Result<models::BatchImportResult, String> {
    let mut results = Vec::new();
    let mut success_count = 0;
    let mut error_count = 0;
    let mut skipped_count = 0;

    for file_path in file_paths {
        let path = std::path::PathBuf::from(&file_path);

        // 检查是否已存在
        if skip_existing {
            let exists = check_book_exists(file_path.clone(), state.clone()).await?;
            if exists {
                results.push(models::ImportResult {
                    file_path: file_path.clone(),
                    status: "skipped".to_string(),
                    error: Some("文件已存在".to_string()),
                    book: None,
                });
                skipped_count += 1;
                continue;
            }
        }

        // 尝试导入
        match state.book_manager.import_book(path).await {
            Ok(book) => {
                results.push(models::ImportResult {
                    file_path: file_path.clone(),
                    status: "success".to_string(),
                    error: None,
                    book: Some(book),
                });
                success_count += 1;
            }
            Err(e) => {
                results.push(models::ImportResult {
                    file_path: file_path.clone(),
                    status: "error".to_string(),
                    error: Some(e.to_string()),
                    book: None,
                });
                error_count += 1;
            }
        }
    }

    Ok(models::BatchImportResult {
        total: results.len(),
        success: success_count,
        error: error_count,
        skipped: skipped_count,
        results,
    })
}

// 插件工具函数命令

/// 插件HTTP GET请求
#[tauri::command]
async fn plugin_http_get(url: String) -> Result<String, String> {
    use book_source_manager::HttpUtils;

    let http_utils = HttpUtils::new();
    http_utils.get(&url).await.map_err(|e| e.to_string())
}

/// 插件HTTP POST请求
#[tauri::command]
async fn plugin_http_post(url: String, data: String) -> Result<String, String> {
    use book_source_manager::HttpUtils;

    let http_utils = HttpUtils::new();
    http_utils
        .post(&url, &data)
        .await
        .map_err(|e| e.to_string())
}


/// 插件URL编码
#[tauri::command]
async fn plugin_encode_url(url: String) -> Result<String, String> {
    use book_source_manager::UrlUtils;

    Ok(UrlUtils::encode_url(&url))
}

// 书源管理命令

/// 加载书源插件
#[tauri::command]
async fn load_book_source(
    plugin_path: String,
    state: State<'_, AppState>,
) -> Result<models::BookSourceInfo, String> {
    let path = PathBuf::from(plugin_path);
    let mut manager = state.book_source_manager.lock().await;
    manager.load_source(path).await.map_err(|e| e.to_string())
}

/// 获取所有书源
#[tauri::command]
async fn get_book_sources(
    state: State<'_, AppState>,
) -> Result<Vec<models::BookSourceInfo>, String> {
    let manager = state.book_source_manager.lock().await;
    manager.get_sources().await.map_err(|e| e.to_string())
}

/// 切换书源启用状态
#[tauri::command]
async fn toggle_book_source(
    source_id: String,
    enabled: bool,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut manager = state.book_source_manager.lock().await;
    manager
        .toggle_source(&source_id, enabled)
        .await
        .map_err(|e| e.to_string())
}

/// 删除书源
#[tauri::command]
async fn remove_book_source(source_id: String, state: State<'_, AppState>) -> Result<(), String> {
    let mut manager = state.book_source_manager.lock().await;
    manager
        .unload_source(&source_id)
        .await
        .map_err(|e| e.to_string())
}

/// 获取插件代码
#[tauri::command]
async fn get_plugin_code(source_id: String, state: State<'_, AppState>) -> Result<String, String> {
    let manager = state.book_source_manager.lock().await;
    manager
        .get_plugin_code(&source_id)
        .ok_or_else(|| "插件代码不存在".to_string())
        .map(|code| code.clone())
}

/// 获取应用设置
#[tauri::command]
async fn get_settings(state: State<'_, AppState>) -> Result<models::AppSettings, String> {
    let settings = state.settings.read().await;
    Ok(settings.clone())
}

/// 保存应用设置
#[tauri::command]
async fn save_settings(
    settings: models::AppSettings,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut current_settings = state.settings.write().await;
    *current_settings = settings;
    Ok(())
}

/// 打开设置窗口
#[tauri::command]
async fn open_window(router: String, title: String, app: tauri::AppHandle) -> Result<(), String> {
    // 从路由中提取窗口标签，去掉开头的斜杠
    let segments: Vec<&str> = router.split('/').collect();
    let window_label = segments.get(0).copied().unwrap_or(&router);

    // 构建新的URL
    let url_path = format!("/?router={}", router);
    let webview_url = tauri::WebviewUrl::App(url_path.clone().into());

    // 检查窗口是否已经存在
    if let Some(window) = app.get_webview_window(window_label) {
        // 如果窗口存在，更新URL并显示聚焦
        // 注意：navigate方法需要的是Url类型，我们需要使用不同的方法
        // 由于Tauri的限制，我们可能需要重新加载页面或使用其他方式
        window
            .eval(&format!("window.location.href = '{}'", url_path))
            .map_err(|e| format!("更新窗口URL失败: {}", e))?;
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
    } else {
        let _ = tauri::WebviewWindowBuilder::new(&app, window_label, webview_url)
            .title(title + " - NovelNest")
            .inner_size(900.0, 700.0)
            .min_inner_size(800.0, 600.0)
            .resizable(true)
            .build()
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

// 在线搜索和下载功能命令

/// 在线搜索图书
#[tauri::command]
async fn search_online_books(
    query: String,
    state: State<'_, AppState>,
) -> Result<Vec<models::SearchResult>, String> {
    let manager = state.book_source_manager.lock().await;

    // 获取所有启用的书源
    let sources = manager.get_sources().await.map_err(|e| e.to_string())?;
    let enabled_sources: Vec<_> = sources.into_iter().filter(|s| s.enabled).collect();

    if enabled_sources.is_empty() {
        return Ok(Vec::new());
    }

    // 并行搜索所有启用的书源
    let mut search_tasks = Vec::new();

    for source in enabled_sources {
        let source_id = source.id.clone();
        let query_clone = query.clone();

        // 获取插件代码
        if let Some(plugin_code) = manager.get_plugin_code(&source_id) {
            let plugin_code = plugin_code.clone();

            // 创建异步任务进行搜索
            let task = tokio::spawn(async move {
                search_with_plugin(source_id, query_clone, plugin_code).await
            });

            search_tasks.push(task);
        }
    }

    // 等待所有搜索任务完成并聚合结果
    let mut all_results = Vec::new();

    for task in search_tasks {
        match task.await {
            Ok(Ok(mut results)) => {
                all_results.append(&mut results);
            }
            Ok(Err(e)) => {
                eprintln!("书源搜索失败: {}", e);
                // 继续处理其他书源的结果，不因单个书源失败而中断
            }
            Err(e) => {
                eprintln!("搜索任务执行失败: {}", e);
            }
        }
    }

    // 去重和排序
    let deduplicated_results = deduplicate_search_results(all_results);

    Ok(deduplicated_results)
}

/// 获取在线图书章节列表
#[tauri::command]
async fn get_online_chapters(
    source_id: String,
    book_url: String,
    state: State<'_, AppState>,
) -> Result<Vec<models::ChapterInfo>, String> {
    let manager = state.book_source_manager.lock().await;

    // 获取插件代码
    let plugin_code = manager
        .get_plugin_code(&source_id)
        .ok_or_else(|| "插件代码不存在".to_string())?
        .clone();

    // 使用插件获取章节列表
    get_chapters_with_plugin(source_id, book_url, plugin_code)
        .await
        .map_err(|e| e.to_string())
}

/// 下载在线图书
#[tauri::command]
async fn download_book(
    source_id: String,
    book_url: String,
    chapters: Vec<usize>,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let manager = state.book_source_manager.lock().await;

    // 获取插件代码
    let plugin_code = manager
        .get_plugin_code(&source_id)
        .ok_or_else(|| "插件代码不存在".to_string())?
        .clone();

    // 首先获取章节列表
    let all_chapters =
        get_chapters_with_plugin(source_id.clone(), book_url.clone(), plugin_code.clone())
            .await
            .map_err(|e| e.to_string())?;

    // 过滤要下载的章节
    let chapters_to_download: Vec<_> = chapters
        .into_iter()
        .filter_map(|index| all_chapters.get(index).cloned())
        .collect();

    if chapters_to_download.is_empty() {
        return Err("没有找到要下载的章节".to_string());
    }

    // 批量下载章节内容
    let book_content = download_chapters_with_plugin(source_id, chapters_to_download, plugin_code)
        .await
        .map_err(|e| e.to_string())?;

    // 保存为本地文件并添加到图书库
    let book_id = save_downloaded_book(&book_content, &state.book_manager)
        .await
        .map_err(|e| e.to_string())?;

    Ok(book_id)
}

// 在线搜索和下载辅助函数

/// 使用插件搜索图书
async fn search_with_plugin(
    source_id: String,
    query: String,
    _plugin_code: String,
) -> Result<Vec<models::SearchResult>, String> {
    // 模拟搜索结果（实际应该通过插件执行）
    let mock_results = vec![models::SearchResult {
        title: format!("搜索结果: {}", query),
        author: Some("测试作者".to_string()),
        description: Some("这是一个测试搜索结果".to_string()),
        source_id,
        book_url: format!("https://example.com/book/{}", query),
        cover_url: None,
    }];

    Ok(mock_results)
}

/// 使用插件获取章节列表
async fn get_chapters_with_plugin(
    _source_id: String,
    book_url: String,
    _plugin_code: String,
) -> Result<Vec<models::ChapterInfo>, String> {
    // 模拟章节获取（实际应该通过插件执行）
    let mock_chapters = vec![
        models::ChapterInfo {
            title: "第一章".to_string(),
            url: format!("{}/chapter/1", book_url),
            index: 0,
        },
        models::ChapterInfo {
            title: "第二章".to_string(),
            url: format!("{}/chapter/2", book_url),
            index: 1,
        },
    ];

    Ok(mock_chapters)
}

/// 使用插件下载章节内容
async fn download_chapters_with_plugin(
    _source_id: String,
    chapters: Vec<models::ChapterInfo>,
    _plugin_code: String,
) -> Result<DownloadedBook, String> {
    let mut book_content = DownloadedBook {
        title: "下载的图书".to_string(),
        author: Some("未知作者".to_string()),
        chapters: Vec::new(),
    };

    // 模拟下载章节内容
    for chapter in chapters {
        let content = format!("这是{}的内容...", chapter.title);
        book_content.chapters.push(ChapterContent {
            title: chapter.title,
            content,
        });
    }

    Ok(book_content)
}

/// 保存下载的图书到本地
async fn save_downloaded_book(
    book_content: &DownloadedBook,
    book_manager: &BookManager,
) -> Result<String, String> {
    use std::io::Write;

    // 创建下载目录
    let downloads_dir = std::env::current_dir()
        .map_err(|e| format!("获取当前目录失败: {}", e))?
        .join("data")
        .join("downloads");

    if !downloads_dir.exists() {
        std::fs::create_dir_all(&downloads_dir).map_err(|e| format!("创建下载目录失败: {}", e))?;
    }

    // 生成文件名
    let safe_title = book_content
        .title
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == ' ' || *c == '-' || *c == '_')
        .collect::<String>()
        .replace(' ', "_");

    let file_path = downloads_dir.join(format!("{}.txt", safe_title));

    // 写入文件内容
    let mut file = std::fs::File::create(&file_path).map_err(|e| format!("创建文件失败: {}", e))?;

    // 写入标题和作者信息
    writeln!(file, "书名: {}", book_content.title).map_err(|e| format!("写入文件失败: {}", e))?;

    if let Some(author) = &book_content.author {
        writeln!(file, "作者: {}", author).map_err(|e| format!("写入文件失败: {}", e))?;
    }

    writeln!(file, "\n{}", "=".repeat(50)).map_err(|e| format!("写入文件失败: {}", e))?;

    // 写入章节内容
    for chapter in &book_content.chapters {
        writeln!(file, "\n{}\n", chapter.title).map_err(|e| format!("写入文件失败: {}", e))?;
        writeln!(file, "{}", chapter.content).map_err(|e| format!("写入文件失败: {}", e))?;
        writeln!(file, "\n{}", "-".repeat(30)).map_err(|e| format!("写入文件失败: {}", e))?;
    }

    // 导入到图书库
    let book = book_manager
        .import_book(file_path)
        .await
        .map_err(|e| format!("导入图书失败: {}", e))?;

    Ok(book.id)
}

/// 去重搜索结果
fn deduplicate_search_results(results: Vec<models::SearchResult>) -> Vec<models::SearchResult> {
    use std::collections::HashSet;

    let mut seen = HashSet::new();
    let mut deduplicated = Vec::new();

    for result in results {
        // 使用书名和作者作为去重键
        let key = format!(
            "{}_{}",
            result.title.to_lowercase().trim(),
            result.author.as_deref().unwrap_or("").to_lowercase().trim()
        );

        if seen.insert(key) {
            deduplicated.push(result);
        }
    }

    // 按书名排序
    deduplicated.sort_by(|a, b| a.title.cmp(&b.title));

    deduplicated
}

/// 下载的图书数据结构
#[derive(Debug, Clone)]
struct DownloadedBook {
    title: String,
    author: Option<String>,
    chapters: Vec<ChapterContent>,
}

/// 章节内容数据结构
#[derive(Debug, Clone)]
struct ChapterContent {
    title: String,
    content: String,
}

// 初始化应用状态
fn initialize_app_state() -> AppResult<AppState> {
    // 创建数据目录
    let data_dir = std::env::current_dir()?.join("data");
    if !data_dir.exists() {
        std::fs::create_dir_all(&data_dir)?;
    }

    // 创建插件目录
    let plugin_dir = data_dir.join("plugins");
    if !plugin_dir.exists() {
        std::fs::create_dir_all(&plugin_dir)?;
    }

    // 初始化数据库
    let db_path = data_dir.join("NovelNest.db");
    let db = Arc::new(DatabaseManager::new(db_path)?);

    // 初始化图书管理器
    let book_manager = Arc::new(BookManager::new(db.clone(), data_dir.clone()));

    // 初始化书源管理器
    let book_source_manager = BookSourceManager::new(db.clone(), plugin_dir)
        .map_err(|e| crate::errors::AppError::BookSource(e))?;
    let book_source_manager = Arc::new(tokio::sync::Mutex::new(book_source_manager));

    // 初始化应用设置
    let settings = Arc::new(tokio::sync::RwLock::new(models::AppSettings::default()));

    Ok(AppState {
        db,
        book_manager,
        book_source_manager,
        settings,
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 设置 panic hook 来捕获崩溃信息
    std::panic::set_hook(Box::new(|panic_info| {
        eprintln!("应用崩溃！");
        eprintln!("崩溃位置: {:?}", panic_info.location());
        eprintln!(
            "崩溃信息: {:?}",
            panic_info.payload().downcast_ref::<&str>()
        );

        // 尝试获取更多信息
        if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            eprintln!("崩溃详情: {}", s);
        }

        // 打印调用栈
        eprintln!("调用栈:");
        let backtrace = std::backtrace::Backtrace::capture();
        eprintln!("{}", backtrace);
    }));

    // 创建 Tokio 运行时
    let runtime = tokio::runtime::Runtime::new().expect("无法创建 Tokio 运行时");
    let runtime_handle = runtime.handle().clone();

    // 将运行时放入 Arc 中，确保其生命周期足够长
    let runtime = std::sync::Arc::new(runtime);

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(move |app| {
            // 使用 move 关键字获取所有权
            // 初始化应用状态
            let app_state = initialize_app_state().map_err(|e| {
                eprintln!("Failed to initialize app state: {}", e);
                e
            })?;

            // 初始化书源管理器
            let book_source_manager = app_state.book_source_manager.clone();

            // 在 Tokio 运行时中执行异步任务
            runtime_handle.spawn(async move {
                if let Err(e) = book_source_manager.lock().await.initialize().await {
                    eprintln!("Failed to initialize book source manager: {}", e);
                }
            });

            // 将状态添加到应用中
            app.manage(app_state);

            // 将运行时添加到应用中，确保其生命周期与应用相同
            app.manage(runtime);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            import_book,
            import_folder,
            get_books,
            search_books,
            delete_book,
            filter_books_by_format,
            filter_books_by_author,
            get_recent_books,
            get_recently_read_books,
            get_library_stats,
            save_reading_progress,
            get_reading_progress,
            add_bookmark,
            get_bookmarks,
            delete_bookmark,
            get_book_content,
            get_book_chapters,
            get_chapter_content,
            get_paginated_content,
            scan_folder_for_books,
            get_file_info,
            check_book_exists,
            batch_import_books,
            plugin_http_get,
            plugin_http_post,
            plugin_encode_url,
            load_book_source,
            get_book_sources,
            toggle_book_source,
            remove_book_source,
            get_plugin_code,
            search_online_books,
            get_online_chapters,
            download_book,
            get_settings,
            save_settings,
            open_window
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_deduplicate_search_results() {
        let results = vec![
            models::SearchResult {
                title: "测试小说".to_string(),
                author: Some("作者A".to_string()),
                description: Some("描述1".to_string()),
                source_id: "source1".to_string(),
                book_url: "url1".to_string(),
                cover_url: None,
            },
            models::SearchResult {
                title: "测试小说".to_string(),
                author: Some("作者A".to_string()),
                description: Some("描述2".to_string()),
                source_id: "source2".to_string(),
                book_url: "url2".to_string(),
                cover_url: None,
            },
            models::SearchResult {
                title: "另一本小说".to_string(),
                author: Some("作者B".to_string()),
                description: Some("描述3".to_string()),
                source_id: "source1".to_string(),
                book_url: "url3".to_string(),
                cover_url: None,
            },
        ];

        let deduplicated = deduplicate_search_results(results);

        // 应该去重，只保留2个结果
        assert_eq!(deduplicated.len(), 2);

        // 验证结果按标题排序
        assert_eq!(deduplicated[0].title, "另一本小说");
        assert_eq!(deduplicated[1].title, "测试小说");
    }

    #[tokio::test]
    async fn test_search_with_plugin() {
        let result = search_with_plugin(
            "test_source".to_string(),
            "测试查询".to_string(),
            "mock_plugin_code".to_string(),
        )
        .await;

        assert!(result.is_ok());
        let search_results = result.unwrap();
        assert!(!search_results.is_empty());
        assert_eq!(search_results[0].source_id, "test_source");
    }
}
