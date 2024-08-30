import asyncio

from textual.screen import Screen
from textual.widgets import DataTable
from textual.events import MouseUp

from Scripts.Widgets import Header, Footer, Input
from Scripts.Novel import search_book


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
        if row := table.hover_row:
            book_info = self.search_result[row]
            raise Exception(book_info)

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
            timestamp = str(book['time_update'])
            last_chapter, name, author = book['last_chapter_name'], book['book_name'], book['author']
            table.add_row(name, author, F'{timestamp[:4]}-{timestamp[4:6]}-{timestamp[6:8]}', last_chapter)

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
