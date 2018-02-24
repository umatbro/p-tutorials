'''
Download video links from your `YouTube <http://www.youtube.com>`_ playlist.
Open your web browser on your YouTube playlist, right click and ``Save page as...``
When you have page **HTML** source saved on your PC, call :func:`get_links` generator function and provide it with the path to the file.

This generator function will yield dictionaries with ``title`` and ``href`` keys.
You can print results using :func:`pretty_song` function.
'''

from typing import List, Dict
from bs4 import BeautifulSoup as BS


def get_links(file_src='index.html') -> List[Dict[str, str]]:
    """
    Fetch video links from YouTube page.

    :param file_src: file with HTML code of playlist
    :return: list containing dictionaries with keys: ('title', 'href')
    """
    with open(file_src) as file:
        soup = BS(file.read(), 'html.parser')

    vid_entries = soup.select('a.yt-simple-endpoint.style-scope.ytd-playlist-video-renderer')
    for vid_elem in vid_entries:
        song = vid_elem.select_one('span[title]')
        if song:
            title = song['title']
            href = vid_elem.select_one('a[href]')['href']
            yield {'title': title, 'href': href}


def pretty_song(song: Dict[str, str]) -> str:
    """
    :param song: dictionary with title and link
    :return: prettified song entry
    """
    return '{}\n  http://www.youtube.com{}\n'.format(song['title'], song['href'])


if __name__ == '__main__':
    # example usage
    for song in list(get_links('file.html'))[206:]:
        print(pretty_song(song))
