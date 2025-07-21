#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use tempfile::TempDir;
    use tokio;
    use std::sync
 r;
    use novel_nest_l

    /// 创建测试环境
    async fn setup_test_env() -> (Arc<DatabaseManager>, T
        时目录
        let temp_dir = T
        
        // 创建数据库
        lb");
        let db = Arc::new(DatabaseManager::new(&db_pat
        
        (db, ter)
    }

  件
    fn create_test_p
        // 创建插件目录
        let plugin_dir
        std::fs::create_dir_all(&plugin_dir).expect("创建插件目录失败");
        
        // 创元数据文件
        let metadata
            r#"{{
                ""{}",
                "name": "{}",
                "version": "1.0.0",
                "author": "测试作者",
                "description": "测试描述",
                "baseUrl": "https://example.com"
            }}"#,
            id, name
        );
        let me
        std::fs::write(&metadata_path, metadata_content).expect("创败");
        
        插件代码文件
        let js_code "
            class TestBookSurce {
                constructor() {
                    this.baseUrl om";
                }
                
                // 搜索函数
                async s
                    // 模拟调用 plugin_http_get
                    try {
                        const
                        return [
                            { 
                                tiord, 
                                author: "测试作者", 
                                description: "这是,
                                bookUrl: this.baseUrl + ",
                                coverUrl: this.baseUrl + "/cover/1."
                            }
                        ];
                    } catc) {
                        console.error( error);
                        return [];
                    }
                }
                
                // 获取章节列表
                async getChapters(b) {
                    // 模拟调用 plugin_parse_html
                    try {
                        return
                            { title: ",
                            { title: "第二章", url: bookUrl + "/2", index: 1 }
                        ];
                    } catch) {
                        console.error("获取章r);
                        return [];
                    }
                }
                
                // 获取章节内容
                async getC{
                    // 模拟调用 plugin_resolve_url
                    try {
                        retun {
                            title",
                            content: "这是章节内容，来自URL:pterUrl
                        };
                    } catch (er
                        console.error);
                        return null;
                    }
                }
            }
            
            件实例
            window.book
        "#;
        
        l);
        std::fs::write(&js_path, js_code).expect("创建插件代码文件;
        
        (metadata)
    }

    #t]
    async fn test_plug {
        // 设置测试环境
        let (db, ;
        
        /源管理器
        let plugin_dir ins");
        std::fs::create_dir_all(&plugin_dir).expect("创建插件目录失败");
        l败");
        
        // 创建测试插件
        let (_, js源");
        
        源
        let sourc");
        
        // 验证书源信息
        ;
        assert_eq);
        
        // 获取插件代码
        ");
        
        // 验证插件代码包含必要的方法
        assert!(plugin_code.contains("search"));
        assert!(plugin_code.contains("getChapters"));
        );
        
        // 模拟前端插件执行系统
        ript环境
        
        // 模拟解析URL
        let base_url = "https://examp";
        let relative_url = "/book/1";
        let resolved_url = UrlU
        
        assert_eq!(resolved_url, "https://example.com/book/1");
        
        // 模拟URL编码
        let input = "测试关键词";
        let encoded = UrlUtils::encode_url(input);
        
        assert_eq!(encoded, "%E6%B5%8B%E8%AF%95%E5%85%B3%E9%94%AE%E8%AF%8D");
        
        // 模拟HTML解析
        let html = r#"
            <html>
                <body>
                    <div class="chapter-list">
                        <a href="/chapter/1">第一章</a>
                        <a href="/chapter/2">第二章</a>
                    </div>
                </body>
            </html>
        "#;
        
        let elements = HtmlParser::parse_elements(html, ".chapter-list a").expect("HTML解析失败");
        
        
        assert_eq!(elements[0].text, "一章");
        assert_eq!(elements[0].href, Some("/
        assert_eq!(elements[1].text, "第二章");
        assert_eq!(elements[1].href, Some("/g()));
    }


    async fn test_ndling() {
        // 测试无效的选择器
        let html = >";
        let result = HtmlParser::parse_elements(html, "invaliector");
        
        ));
        
        RL
        let result ;
        
        
        
        件
        // 设置测试环境
        let (db, temp_dir) = setup_test_env().await;
        
        // 创建书源管理器
     
");
        let source败");
        
        let plugin_code");
        
        assert!(plugin_code.is_none());
    }

    #[tokio::test]
    a {
HTTP GET请求
        // 注意：这个测试
        let http_utils = HttpUtils::new();
        let result = wait;
        
        // 我们只检查结果关心具体内容
        assert!(result.is_ok() || ));
    }

    #[tokio::test]
    async fn test_plugin_data_flow() {
        // 模拟完整的插件数据流
        
        // 1. 搜索图书
        let sec![
          
        g(),
                auth,
                description:ng()),
                source_id: "test-source".to_string(),
                book_url: "https://exampl(),
                cover_url: Some("https://example.com/cover/1.jpgng()),
            }
        ];
        
        // 2. 获取章节列表
        let chapters = vec![
            novel_nest_linfo {
             g(),
          ,
        x: 0,
            },
            novel_nest_lib::models::ChapterInfo {
                title: "第二章".tring(),
                url: "https://example),
                index: 1,
            }
        ];
        
        内容
        let chap
            chapter_index: 0,
            title: "第一章".to_string(),
            content: "这是第一章的内容".to_string(),
     ,
 
}
    });"title, "第一章ontent.chapter_cq!(ert_ess   a     len(), 2);
ers.rt_eq!(chapt    asse书籍");
    title, "测试ts[0].arch_resulsert_eq!(se   as  验证数据流
           //     
 };
         e: 1,
  ng_timdieaimated_r  est         