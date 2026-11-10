"""Hidden check: the settings form has the display-name field and a Save button."""

from html.parser import HTMLParser
from pathlib import Path


class Page(HTMLParser):
    def __init__(self):
        super().__init__()
        self.form_id = None
        self.labels = []
        self.inputs = set()
        self.buttons = []
        self._label_for = None
        self._label_text = []
        self._button = None
        self._button_text = []

    def handle_starttag(self, tag, attrs):
        a = dict(attrs)
        if tag == "form":
            self.form_id = a.get("id")
        elif tag == "label":
            self._label_for = a.get("for")
            self._label_text = []
        elif tag == "input" and "id" in a:
            self.inputs.add(a["id"])
        elif tag == "button":
            self._button = (a.get("id"), a.get("type"))
            self._button_text = []

    def handle_endtag(self, tag):
        if tag == "label" and self._label_for is not None:
            self.labels.append((self._label_for, "".join(self._label_text).strip()))
            self._label_for = None
        elif tag == "button" and self._button is not None:
            self.buttons.append((*self._button, "".join(self._button_text).strip()))
            self._button = None

    def handle_data(self, data):
        if self._label_for is not None:
            self._label_text.append(data)
        if self._button is not None:
            self._button_text.append(data)


page = Page()
page.feed(Path("page/index.html").read_text(encoding="utf-8"))
assert page.form_id == "profile", page.form_id
assert ("display-name", "Display name") in page.labels, page.labels
assert "display-name" in page.inputs, page.inputs
assert ("save", "submit", "Save") in page.buttons, page.buttons
print("ok")
