# Performance

|                                                                                                                      Day | Rank (Part 1) | Rank (Part 2) | Remarks\*                                                                                                                                                                                                                       |
| -----------------------------------------------------------------------------------------------------------------------: | ------------: | ------------: | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [3](https://github.com/clearlyMine/advent_rust/blob/81cc5519a5b330102844d388a3756ae2f146191a/year_2021/src/bin/day03.rs) |        80,383 |        69,101 | It was straight-forward, so tried to find the fastest method to complete it but because the input isn't very large even the benchmarks don't run long enough to acually tell if there's a real difference caused by my changes. |
| [2](https://github.com/clearlyMine/advent_rust/blob/cbd813869cf8931162614c776d07aff290e6696d/year_2021/src/bin/day02.rs) |        16,082 |        15,263 |                                                                                                                                                                                                                                 |
| [1](https://github.com/clearlyMine/advent_rust/blob/adf262c2eca99e068a79798ef8a37d166f8ea33c/year_2021/src/bin/day01.rs) |        38,947 |        28,917 |                                                                                                                                                                                                                                 |

\*Remarks are for the very first solution through which I got the answer that I submitted, the latest code will be different as I keep optimizing the solutions.

### To run a particular day

```bash
mkdir inputs
cd inputs
```

a separate file needs to be provided for each day in the format dayxx.txt

```bash
touch dayxx.txt
```

Open the file in your favourite editor and paste your puzzle input inside the file

```bash
cargo run --release --bin dayxx
```

(make sure to change the day number)
<br>

### To run all the days at once

make sure that all the puzzle inputs are present

```bash
cargo run --release
```
