import asyncio

from textual.screen import Screen
from textual.widgets import DataTable
from textual.events import MouseUp

from .Information import InformationScreen
from Scripts.Utils import deal_timestamp
from Scripts.Widgets import Header, Footer, Input
from Scripts.Novel import get_book_info, search_book


class ResultScreen(Screen):
    search_task = None
    search_result = None

    def __init__(self, result: dict):
        Screen.__init__(self, id='result')
        self.search_result = result

    def compose(self):
        yield Header()
        yield Input(placeholder='请输入你要搜索的内容')
        table = DataTable()
        table.add_column('书名')
        table.add_column('作者')
        table.add_column('更新时间')
        table.add_column('最新章节')
        yield table
        footer = Footer()
        footer.update('搜索小说成功！')
        yield footer

    def on_mount(self):
        self.update_table()

    def on_mouse_up(self, event: MouseUp):
        table = self.query_one(DataTable)
        if event.y <= (4 + table.row_count):
            book_info = self.search_result[table.hover_row]
            self.search_task = asyncio.create_task(self.search_book_info(book_info))

    def on_input_submitted(self, event: Input.Submitted):
        input_box = self.query_one(Input)
        input_box.disabled = True
        footer = self.query_one(Footer)
        footer.update('正在搜索小说，请耐心等待……')
        self.search_task = asyncio.create_task(self.search_book(event.value))

    def update_table(self):
        table = self.query_one(DataTable)
        table.clear()
        for book in self.search_result:
            timestamp = deal_timestamp(book['time_update'])
            last_chapter, name, author = book['last_chapter_name'], book['book_name'], book['author']
            table.add_row(name, author, timestamp, last_chapter)

    async def search_book(self, keyword: str):
        footer = self.query_one(Footer)
        input_box = self.query_one(Input)
        if result := await search_book(keyword):
            input_box.value = ''
            input_box.disabled = False
            footer.update('搜索小说成功！')
            self.search_result = result
            self.update_table()
            return self.search_task.cancel()
        input_box.disabled = False
        footer.update('网络遇到问题，查询失败！')
        return self.search_task.cancel()

    async def search_book_info(self, book_info: dict):
        footer = self.query_one(Footer)
        table = self.query_one(DataTable)
        input_box = self.query_one(Input)
        table.disabled = True
        input_box.disabled = True
        if result := await get_book_info(book_info):
            table.disabled = False
            input_box.disabled = False
            screen = InformationScreen(result)
            await self.app.push_screen(screen)
            return self.search_task.cancel()
        table.disabled = False
        input_box.disabled = False
        footer.update('网络遇到问题，加载书籍信息失败！')
        return self.search_task.cancel()
