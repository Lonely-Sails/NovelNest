import json
import asyncio
from random import randint
from httpx import ConnectTimeout, AsyncClient
from base64 import b64decode
from urllib.parse import unquote
from fake_useragent import UserAgent

client = AsyncClient()
user_agent = UserAgent()


# https://kt.bqg123a.top
async def request(url: str, params: dict = None):
    headers = {'User-Agent': user_agent.random, 'Referer': 'https://bqg123.net/'}
    params.setdefault('ws', randint(1000, 2160))
    try:
        response = await client.get('https://la2.bqg123a.top' + url, params=params, headers=headers, timeout=10)
        if response.status_code == 200:
            value = response.text.split('="', maxsplit=1)[1]
            value = b64decode(value.split('";')[0])
            return json.loads(unquote(value.decode('Utf-8')))
    except ConnectTimeout:
        return None
    print(response.status_code)


async def search_book(keyword: str):
    params = {'keyword': keyword, 'pf': 'null'}
    if response := await request('/v3/search1', params):
        return response.get('book_list')
    return None


async def get_chapter(chapter_info: dict):
    chapter_key = chapter_info.get('url_kv')
    params = {'t': chapter_info.get('len'), 'tk': '0404'}
    if chapter := await request(F'/load_chapter/{chapter_key}.js', params):
        chapter_content = chapter['chapter_kv']['content']
        chapter['chapter_kv']['content'] = [line.strip('</p>\n') for line in chapter_content.split('<p>')]
        return chapter


async def get_book_info(book_info: dict):
    params = {'tk': '0404'}
    book_id = book_info.get('book_id')
    book_uni = book_info.get('book_uni_id')
    return await request(F'/v3/load_book_info/{book_uni}/{book_id}.js', params)


async def get_chapter_list(book_info: dict):
    key = book_info.get('url_chapter_list_kv')
    params = {'t': book_info.get('time_chapter_list_kv'), 'tk': '0404'}
    return await request(F'/load_chapter_list/{key}.js', params)


async def main():
    books_info = await search_book(input('请输入关键字: '))
    for book_info_ in books_info:
        print(book_info_)
    index = int(input('请输入书籍序号: '))
    book_more_info = await get_book_info(books_info[index - 1])
    print(book_more_info)
    chapter_list = (await get_chapter_list(book_more_info))['chapter_list']
    for chapter_info in chapter_list:
        print(chapter_info)
    index_chapter = int(input('请输入章节序号: '))
    chapter_info = await get_chapter(chapter_list[index_chapter - 1])
    print('\n'.join(chapter_info['chapter_kv']['content']))


if __name__ == '__main__':
    asyncio.run(main())
