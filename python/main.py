from textual.app import App
from textual.containers import Vertical, HorizontalGroup
from textual.widgets import Button, RichLog, TextArea, TabbedContent


class RunTab(Vertical):
    def compose(self):
        yield HorizontalGroup(
            Button("Run", id="run", flat=True), Button("Clear", id="clear", flat=True)
        )
        yield RichLog()

    def on_button_pressed(self, event: Button.Pressed) -> None:
        if event.button.id == "run":
            text_log = self.query_one(RichLog)
            text_log.write("Not implemented")
        if event.button.id == "clear":
            text_log = self.query_one(RichLog)
            text_log.clear()


class ComputerGeometrySystemApp(App):
    def compose(self):
        yield Button.error("Exit", id="exit", flat=True)
        with TabbedContent("Given", "Find", "Run"):
            yield TextArea()
            yield TextArea()
            yield RunTab()

    def on_button_pressed(self, event: Button.Pressed) -> None:
        if event.button.id == "exit":
            self.exit()


if __name__ == "__main__":
    app = ComputerGeometrySystemApp()
    app.run()
