from textual.widgets._header import HeaderTitle, HeaderClock, HeaderIcon
from textual.widgets._header import Header as TextualHeader
from textual.events import Click
from textual.reactive import Reactive


class Icon(HeaderIcon):
    icon = Reactive('x')

    def on_click(self, event: Click):
        self.app.exit()


class Header(TextualHeader):
    def compose(self):
        yield Icon()
        yield HeaderTitle()
        yield HeaderClock()


class BackIcon(HeaderIcon):
    icon = Reactive('<')

    def on_click(self, event: Click):
        screen = self.app.pop_screen()
        self.app.uninstall_screen(screen)


class BackHeader(TextualHeader):
    def compose(self):
        yield BackIcon()
        yield HeaderTitle()
        yield HeaderClock()
