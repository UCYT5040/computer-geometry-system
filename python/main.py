from textual.app import App
from textual.containers import Vertical, HorizontalGroup
from textual.widgets import Button, RichLog, TextArea, TabbedContent

from methods import load_all_methods, properties_registry
from parse import parse_given

load_all_methods("../methods")

class RunTab(Vertical):
    def compose(self):
        yield HorizontalGroup(
            Button("Run", id="run", flat=True), Button("Clear", id="clear", flat=True)
        )
        yield RichLog()

    def on_button_pressed(self, event: Button.Pressed) -> None:
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
        elif event.button.id == "run":
            text_log = self.query_one(RichLog)
            
            # Get the first TextArea (the "Given" tab)
            given_textarea = self.app.query(TextArea).first()
            given = given_textarea.text
            
            parsed = parse_given(given, properties_registry)
            text_log.write(parsed)


if __name__ == "__main__":
    app = ComputerGeometrySystemApp()
    app.run()
