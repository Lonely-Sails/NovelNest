from textual.app import App

from .Screens import WelcomeScreen


class Application(App):
    CSS_PATH = 'Application.css'

    def on_mount(self):
        welcome_screen = WelcomeScreen()
        self.push_screen(welcome_screen)
