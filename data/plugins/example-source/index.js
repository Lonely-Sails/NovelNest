// 示例书源插件
class ExampleBookSource {
    constructor() {
        this.baseUrl = "https://api.example.com";
    }

    // 搜索图书 - 必须实现
    async search(keyword) {
        try {
            const url = `${this.baseUrl}/search?q=${encodeURIComponent(keyword)}`;
            
            // 调用Tauri提供的HTTP接口
            const response = await window.__TAURI__.invoke('plugin_http_get', { url });
            const data = JSON.parse(response);
            
            return data.results.map(item => ({
                title: item.title,
                author: item.author,
                description: item.summary || '',
                bookUrl: item.detail_url,
                coverUrl: item.cover_image || null
            }));
        } catch (error) {
            console.error(`搜索失败: ${error.message}`);
            return [];
        }
    }

    // 获取章节列表 - 必须实现
    async getChapters(bookUrl) {
        try {
            // 调用Tauri提供的HTTP接口
            const html = await window.__TAURI__.invoke('plugin_http_get', { url: bookUrl });
            
            // 调用Tauri提供的HTML解析接口
            const chapterElements = await window.__TAURI__.invoke('plugin_parse_html', {
                html: html,
                selector: '.chapter-list a'
            });
            
            return chapterElements.map((element, index) => ({
                title: element.text,
                url: await this.resolveUrl(bookUrl, element.href),
                index: index
            }));
        } catch (error) {
            console.error(`获取章节列表失败: ${error.message}`);
            return [];
        }
    }

    // 获取章节内容 - 必须实现
    async getChapterContent(chapterUrl) {
        try {
            const html = await window.__TAURI__.invoke('plugin_http_get', { url: chapterUrl });
            
            const contentElements = await window.__TAURI__.invoke('plugin_parse_html', {
                html: html,
                selector: '.chapter-content'
            });
            
            const titleElements = await window.__TAURI__.invoke('plugin_parse_html', {
                html: html,
                selector: '.chapter-title'
            });
            
            return {
                content: contentElements[0]?.text || '',
                title: titleElements[0]?.text || ''
            };
        } catch (error) {
            console.error(`获取章节内容失败: ${error.message}`);
            return null;
        }
    }

    // 工具函数
    async resolveUrl(base, relative) {
        return await window.__TAURI__.invoke('plugin_resolve_url', { base, relative });
    }
}

// 导出插件实例
window.bookSourcePlugin = new ExampleBookSource();