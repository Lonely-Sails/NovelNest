import asyncio
from textual.screen import Screen
from textual.widgets import Label

from Scripts.Widgets import Header, Footer, Input
from Scripts.Screens.Result import ResultScreen
from Scripts.Novel import search_book


class WelcomeScreen(Screen):
    search_task = None

    def __init__(self):
        Screen.__init__(self, id='welcome')

    def compose(self):
        yield Header()
        yield Label('欢迎使用小说搜索器！👏')
        yield Input(placeholder='请输入你要搜索的内容')
        yield Footer()

    def on_input_submitted(self, event: Input.Submitted):
        input_box = self.query_one(Input)
        input_box.disabled = True
        footer = self.query_one(Footer)
        footer.update('正在搜索小说，请耐心等待……')
        self.search_task = asyncio.create_task(self.search_book(event.value))

    async def search_book(self, keyword: str):
        if result := await search_book(keyword):
            result_screen = ResultScreen(result)
            await self.app.push_screen(result_screen)
            return self.search_task.cancel()
        input_box = self.query_one(Input)
        input_box.disabled = False
        footer = self.query_one(Footer)
        footer.update('网络遇到问题，查询失败！')
        self.search_task.cancel()
        return self.search_task.cancel()
