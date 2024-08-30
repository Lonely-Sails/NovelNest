import asyncio

from textual.events import MouseUp
from textual.screen import Screen
from textual.widgets import Label, ListView, ListItem
from textual.containers import Container

from Scripts.Novel import get_chapter_list
from Scripts.Globals import book_info_mapping
from Scripts.Widgets import BackHeader, Footer, Input


class InformationScreen(Screen):
    task = None
    book_info: dict = None

    def __init__(self, result: dict):
        Screen.__init__(self, id='information')
        self.book_info = result

    def compose(self):
        yield BackHeader()
        footer = Footer()
        footer.update('加载小说详细信息成功！')
        with Container():
            for key, name in book_info_mapping:
                yield Label(F'  {name}： {self.book_info[key]}')
        yield ListView()
        yield footer

    def on_mount(self):
        self.task = asyncio.create_task(self.get_chapter_list())

    async def get_chapter_list(self):
        footer = self.query_one(Footer)
        footer.update('正在获取章节列表……')
        if chapter_list := await get_chapter_list(self.book_info):
            footer.update('获取章节列表成功！')
            chapter_list_view = self.query_one(ListView)
            for chapter in chapter_list['chapter_list']:
                await chapter_list_view.append(ListItem(chapter['name']))
            self.refresh()
            return self.task.cancel()
        footer.update('获取章节列表失败！')
        return self.task.cancel()
