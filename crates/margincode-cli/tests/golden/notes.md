<!-- margincode:notes v1 task=3f0c2b1a-0000-4000-8000-000000000001 -->
### Margincode: why this change

**Atomic save** (`pantry/atomic-save`, in review; approved by codex, owner claude)

> Write pantry.json atomically so a crash cannot truncate it.

**Decisions**
- Write to a temp file in the same dir, then os.replace (`rec:b2000000`, claude) · `pantry/store.py:save`
  > os.replace is atomic on POSIX and Windows
  > when both paths are on one volume.

**Dead ends**
- tried os.rename; fails on Windows if the target exists (`rec:c3000000`, claude) · `pantry/store.py`

**Notes**
- fsync the temp file before replace; ask @⁠alice about NFS &lt;!-- x --&gt; (`rec:e5000000`, claude)

**Review findings**
- missing a test for a crash between write and replace (`rec:f6000000`, codex) · `tests/test_store.py`

**Checks**
- `pytest -q` exit 0 (`rec:a7000000`, claude)

<details><summary>Recorded earlier about the changed files (1 of 3, 1 record)</summary>

`pantry/store.py`
- rec:99990000 decision: load() returns {} for a missing file
  anchors: pantry/store.py:load

</details>

<sub>Written by `av notes` from the task's records. Records: rec:b2000000 rec:c3000000 rec:e5000000 rec:f6000000 rec:a7000000. `av expand <id>` prints one in full; after the merge, `av why path:line` names this task.</sub>
<!-- /margincode:notes -->
