## Task
# Improve greeting · improve-greeting · open
Goal: Make hello louder
Workspace: /ws/demo/improve-greeting
Changed (1): app/greeter.py

## Questions
- rec:XXXXXXXX open, told to park: May the cache outlive a restart?
  options: Postgres (recommended: the daemon already runs one) | SQLite
  depends: the cache's schema
- rec:XXXXXXXX answered by alice: Which database should the cache use? → SQLite
  It is not the recommended option (Postgres).
  Comment: keep it on one disk

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

## Decisions & dead ends
- rec:XXXXXXXX note: the goal says it must
  anchors: app/greeter.py:Greeter.hello

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

---
tokens: N
