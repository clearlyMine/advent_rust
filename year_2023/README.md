# Performance
| Day | Time (Part 1) | Rank (Part 1) | Time (Part 2) | Rank (Part 2) | Remarks |
|----:|--------------:|--------------:|--------------:|--------------:|---------|
| 8   | 00:20:05      | 5,804         | 01:00:24      | 4,773         | Wasted time in finding a brute-force solution rather that an LCM for Part 2|
| 7   | 01:20:25      | 8,839         | 02:12:35      | 9,055         ||
| 6   | 00:20:45      | 6,955         | 00:31:07      | 7,393         ||
| 5   | 01:06:58      | 8,648         | 01:49:37      | 3,890         ||
| 4   | 00:20:32      | 7,633         | 00:45:59      | 7,273         ||
| 3   | >24h          | 80,383        | >24h          | 69,101        | Was busy on the day, started after Day 04 |
| 2   | 01:52:59      | 16,082        | 02:01:05      | 15,263        ||
| 1   | 04:29:55      | 38,947        | 05:46:55      | 28,917        | Was pretty straight-forward but started pretty late|

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
