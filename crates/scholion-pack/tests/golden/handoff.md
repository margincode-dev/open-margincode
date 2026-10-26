## Goal
# Improve greeting · improve-greeting · open
Goal: Make hello louder
Owner: claude
Workspace: /ws/demo/improve-greeting

## State
Remaining: make goodbye louder too
Currently broken: test_goodbye fails
Done: hello now ends with an exclamation mark

## Records
- rec:XXXXXXXX attempt: tried upper() in format_name; broke other callers
  anchors: app/util.py:format_name
- rec:XXXXXXXX decision: append the mark in hello, not in format_name
  anchors: app/greeter.py:Greeter.hello
  format_name is shared with goodbye
- rec:XXXXXXXX note: goodbye is out of scope for now
- rec:XXXXXXXX note: greeting style lives in util

## Changes
- sym:2 Greeter (app/greeter.py:3-8)
  class Greeter:
- sym:3 Greeter.hello (app/greeter.py:4-5)
```diff
@@ -4,3 +4,3 @@
     def hello(self, name):
-        return format_name(name)
+        return format_name(name) + "!"
```

## Open questions
Question: should goodbye shout in all caps?

---
tokens: N
