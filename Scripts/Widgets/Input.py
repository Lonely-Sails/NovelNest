from textual import events
from textual.widgets import Input as TextualInput


class Input(TextualInput):
    DEFAULT_CSS = '''
    Input {
        border: round !important;
    }
    '''

    def on_input_changed(self, event: TextualInput.Changed):
        self.app.refresh()
