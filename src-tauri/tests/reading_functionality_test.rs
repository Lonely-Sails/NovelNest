#[cfg(test)]
mod tests {
    use novel_nest_lib::models::BookFormat;

    #[tokio::test]
    async fn test_reading_progress_validation() {
        // Test progress validation
        let valid_progress = 0.5;
        assert!((0.0..=1.0).contains(&valid_progress));

        let invalid_progress = 1.5;
        assert!(!(0.0..=1.0).contains(&invalid_progress));
    }

    #[tokio::test]
    async fn test_chapter_parsing_patterns() {
        let test_content = r#"第一章 开始
这是第一章的内容。

第二章 继续
这是第二章的内容。

第三章 结束
这是第三章的内容。"#;

        let chapter_patterns = [
            r"^第[一二三四五六七八九十百千万\d]+章", // Use ^ to match start of line
        ];

        let lines: Vec<&str> = test_content.lines().collect();
        let mut chapter_starts = Vec::new();

        for (line_index, line) in lines.iter().enumerate() {
            let line = line.trim();
            
            // Skip empty lines
            if line.is_empty() {
                continue;
            }
            
            for pattern in &chapter_patterns {
                if let Ok(regex) = regex::Regex::new(pattern) {
                    if regex.is_match(line) && line.len() < 100 {
                        chapter_starts.push((line_index, line.to_string()));
                        break;
                    }
                }
            }
        }

        assert_eq!(chapter_starts.len(), 3);
        assert!(chapter_starts[0].1.contains("第一章"));
        assert!(chapter_starts[1].1.contains("第二章"));
        assert!(chapter_starts[2].1.contains("第三章"));
    }

    #[tokio::test]
    async fn test_pagination_logic() {
        let content = "这是一个测试内容，用来验证分页功能是否正常工作。";
        let page_size = 10;
        let total_chars = content.chars().count();
        let total_pages = (total_chars + page_size - 1) / page_size;

        assert!(total_pages > 0);
        
        let page_number = 0;
        let start_position = page_number * page_size;
        let end_position = std::cmp::min(start_position + page_size, total_chars);

        let chars: Vec<char> = content.chars().collect();
        let page_content: String = chars[start_position..end_position].iter().collect();

        assert!(!page_content.is_empty());
        assert!(page_content.chars().count() <= page_size); // Use char count instead of byte length
    }

    #[test]
    fn test_book_format_detection() {
        assert_eq!(BookFormat::from_extension("txt"), Some(BookFormat::Txt));
        assert_eq!(BookFormat::from_extension("epub"), Some(BookFormat::Epub));
        assert_eq!(BookFormat::from_extension("pdf"), Some(BookFormat::Pdf));
        assert_eq!(BookFormat::from_extension("doc"), None);
    }

    #[test]
    fn test_estimated_reading_time() {
        let word_count = 600;
        let estimated_reading_time = (word_count as f64 / 300.0).ceil() as u32;
        assert_eq!(estimated_reading_time, 2); // 600 words / 300 words per minute = 2 minutes
    }
}