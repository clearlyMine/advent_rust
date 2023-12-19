# Performance
| Day | Time (Part 1) | Rank (Part 1) | Time (Part 2) | Rank (Part 2) | Remarks* |
|----:|--------------:|--------------:|--------------:|--------------:|---------|
| [19](https://github.com/clearlyMine/advent_rust/blob/1265b42a14473bc9ee03fb797f7c4ddbcb236675/year_2023/src/bin/day19.rs)  | 01:18:23      | 4,797         | 02:53:23      | 3,409         | Parsing the input took almost all of the time in Part 1. Once parsed it was fairly straight forward in Part 1. Part 2: This was tricky and I'm quite proud of what I was able to program |
| [18](https://github.com/clearlyMine/advent_rust/blob/42328d6f62fa6394963369f269b3da41a8b7c4eb/year_2023/src/bin/day18.rs)  | 02:07:09      | 5,527         | 04:24:51      | 4,822         | This was a tough one, I brute-forced Part 1 and for Part 2 tried multiple different approaches but in the end I had to go back to day 10 and figure out the implementation of Shoelace for this one. |
| [16](https://github.com/clearlyMine/advent_rust/blob/2e6b6bd1950318973ba0ee9996fa9175868f42ec/year_2023/src/bin/day16.rs)  | 01:21:15      | 4,642         | 01:37:55      | 4,455         | The question wasn't too hard, figured it would be a recursively done program from the start, it took me long to get all the cases correct and correctly determine the cache. |
| [15](https://github.com/clearlyMine/advent_rust/blob/8fbbf97f16e5b81af380b0959d15dbb016ae9e9f/year_2023/src/bin/day15.rs)  | 00:14:20      | 4,729         | 00:50:05      | 4,305         | Part 1: Kinda disappointed by how long it took me. Only thing that tripped me was not using .trim() on the input as it contained a return(\n) at the end. Part 2: That was one verbose explanation for a fairly simple problem. Only thing that I misread was the HASH algorithm was to be run only on the label and not the whole string. |
| [14](https://github.com/clearlyMine/advent_rust/blob/19b4ff0d6444bc1c9e1945ce5956df607ab386d6/year_2023/src/bin/day14.rs)  | 00:19:56      | 3,202         | 01:42:53      | 4,265         | Part 1 was fairly simple and I just used math instead of modifying the grid. Part 2: I figured out that there has to be a cycle but it took me a substantial amount of time to figure out how to manage the cache. |
| [13](https://github.com/clearlyMine/advent_rust/blob/99c976fd5b4cb197c9c7f6f1e25ef50cee3fcf2d/year_2023/src/bin/day13.rs)  | 02:16:01      | 7,732         | 02:46:52      | 6,380         | Naively assumed that only one end will have to be ignored (which works for the sample) and wasted a long time in trying to get the correct solution for the input. |
| [12](https://github.com/clearlyMine/advent_rust/blob/60a0b39af7236859c2a4b27cf43c540a2873089d/year_2023/src/bin/day12.rs)  | 01:04:34      | 4,471         | 07:24:32      | 6,309         | WTF even is dynamic programming 🙃🙃 |
| [11](https://github.com/clearlyMine/advent_rust/blob/4e598fb4045fdc95df38f68caa3b6a904c2294fd/year_2023/src/bin/day11.rs)  | 01:07:32      | 7,555         | 01:28:59      | 7,108         | Copied the corrected sample and wasted a very long time trying to get the correct answer. For part 2 at first I tried actually increasing the grid size instead of using math, then came up with the math based solution. |
| [10](https://github.com/clearlyMine/advent_rust/blob/59d98f603b654a93e5e0dd49f22b6e0f1c83d2e7/year_2023/src/bin/day10.rs)  | 01:10:58      | 5,040         | 06:04:34      | 7,878         | Part 2 was rough, even after figuring out that shoelace algorithm could be used I wasted way too long in coming up with another kind of solution. |
| [9](https://github.com/clearlyMine/advent_rust/blob/e6fcefd4dc49d5f9a9d8c1c80f544540dc1ca8a1/year_2023/src/bin/day09.rs)   | 00:54:36      | 8,440         | 01:27:23      | 9,634         ||
| [8](https://github.com/clearlyMine/advent_rust/blob/d320b7e8e1c8b95ac2a7e02aaa1c32a1ad08ee5b/year_2023/src/bin/day08.rs)   | 00:20:05      | 5,804         | 01:00:24      | 4,773         | Wasted time in finding a brute-force solution rather that an LCM for Part 2. |
| [7](https://github.com/clearlyMine/advent_rust/blob/3eb0cd67f4e75198834163ceefe93ffb552f31c9/year_2023/src/bin/day07.rs)   | 01:20:25      | 8,839         | 02:12:35      | 9,055         ||
| [6](https://github.com/clearlyMine/advent_rust/blob/3cddbc10281915532b0b935211a27d40742f9193/year_2023/src/bin/day06.rs)   | 00:20:45      | 6,955         | 00:31:07      | 7,393         ||
| [5](https://github.com/clearlyMine/advent_rust/blob/60fd277f4ae1530de22a0bdc5169544df652d9e6/year_2023/src/bin/day05.rs)   | 01:06:58      | 8,648         | 01:49:37      | 3,890         ||
| [4](https://github.com/clearlyMine/advent_rust/blob/340e3a9853da915b0f3902ee076dd294e412f5d0/year_2023/src/bin/day04.rs)   | 00:20:32      | 7,633         | 00:45:59      | 7,273         ||
| [3](https://github.com/clearlyMine/advent_rust/blob/399c3a4bc965f36075a38419734b0ea6c523c88f/year_2023/src/bin/day03.rs)   | >24h          | 80,383        | >24h          | 69,101        | Was busy on the day, started after Day 04. |
| [2](https://github.com/clearlyMine/advent_rust/blob/2f66d15b67a159e3632b2d2f190af383af1034c1/year_2023/src/bin/day02.rs)   | 01:52:59      | 16,082        | 02:01:05      | 15,263        ||
| [1](https://github.com/clearlyMine/advent_rust/blob/2abe1cfd81ae6360a0bc7d32c863df593f774e86/year_2023/src/bin/day01.rs)   | 04:29:55      | 38,947        | 05:46:55      | 28,917        | Was pretty straight-forward but started pretty late. |

*Remarks are for the very first solution through which I got the answer that I submitted, the latest code will be different as I keep optimizing the solutions.

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
