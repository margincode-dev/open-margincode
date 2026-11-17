# F4: a checkpoint while another task starts (2026-11-17)

Throwaway daemons on Docker Desktop, workspaces on a Windows bind mount (`AV_EXEC_BIT=ignore`),
this repository at `35fa87e` with its `.av/sparse`. Task A is open; task B starts in the
background, and 1.5 s later task A checkpoints one edited `README.md`, timed from the client.
Then B is abandoned. One daemon per image, the same data volume and workspace directory.

| daemon | checkpoint alone (ms) | checkpoint during B's start (ms) | B's start (ms) |
|---|---|---|---|
| `main` (`35fa87e`) | 455, 344, 345 | 4,771, 4,711, 9,717 | 6,805, 6,745, 11,780 |
| this change, first run | 351, 347, 343 | 377, 378, 2,236 | 6,803, 7,171, 8,428 |
| this change, second run | | 374, 370, 375, 374, 377 | 6,826, 6,737, 6,763, 6,779, 6,756 |

- On `main` a start writes its workspace (about 6 s of its 6.8 s) under the repo's lock, so the
  checkpoint waited for the rest of the copy. The dogfood daemon (`35fa87e`) showed the same before
  this change: 4,751 to 5,305 ms.
- With the files written after the lock is released, a checkpoint during a start costs what it
  costs alone. One of eight runs took 2.2 s (the third of the first run, which was also the slowest
  run on `main`); not explained here.
- A start itself takes as long as before.
