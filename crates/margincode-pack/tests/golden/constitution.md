## Task
# Improve greeting · improve-greeting · open
Goal: Make hello louder
Workspace: /ws/demo/improve-greeting
Changed (1): app/greeter.py

## Constitution (v2, organisation v1; it outranks conventions and records)
- org:1 Write in English
  Code, records and docs.
- art:1 Never log a provider key
  No key in a log line, ever.
- art:2 Lint passes [check]
  Run the linter before review.

## Map
- app/greeter.py
  L3: class Greeter:
  L4: def hello(self, name):
  L7: def goodbye(self, name):
- app/util.py
  L1: def format_name(n):
- repo map (4 files):
  README.md
  app/ (3 files)
    unrelated.py

## Code
- sym:3 Greeter.hello (app/greeter.py:4-5)
```python
    def hello(self, name):
        return format_name(name) + "!"
```
- sym:2 Greeter (app/greeter.py:3-8)
  class Greeter:
- sym:1 format_name (app/util.py:1-2)
```python
def format_name(n):
    return n.title()
```

## Conventions
- conv:4 (app/**/*.py) use type hints everywhere

---
tokens: N
