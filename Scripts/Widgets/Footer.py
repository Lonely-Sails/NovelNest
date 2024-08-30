from textual.widgets import Label
from textual.widgets import Footer as TextualFooter


class Footer(TextualFooter):
    def __init__(self):
        TextualFooter.__init__(self)
        self.label = Label()
        self.mount(self.label)

    def compose(self):
        yield self.label

    def update(self, text):
        self.label.update(text)
