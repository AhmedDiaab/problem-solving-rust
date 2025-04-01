## Problem E. Team

### Problem Statement

Petya, Vasya, and Tonya are best friends who want to participate in a programming contest. In the contest, they are offered several problems. They will implement a solution to a problem if at least two of them are sure about the solution. Otherwise, they won't implement the solution.

You are given **n** problems, and for each problem, you know whether each of the three friends is sure about the solution. You need to count how many problems they will implement.

### Input

- The first line contains an integer **n** (`1 ≤ n ≤ 1000`) — the number of problems.
- The next **n** lines each contain three integers (either `0` or `1`), which represent whether Petya, Vasya, and Tonya are sure about the solution for each problem. If the integer is `1`, the corresponding friend is sure, and if the integer is `0`, they are not sure.

### Output

- Print a single integer — the number of problems the friends will implement.

### Example 1

#### Input:
```
3
1 1 0
1 1 1
1 0 0
```

#### Output:
```
2
```

#### Explanation:
- For the first problem, Petya and Vasya are sure, so they will implement it.
- For the second problem, all three friends are sure, so they will implement it.
- For the third problem, only Petya is sure, and that is not enough, so they will not implement it.

### Example 2

#### Input:
```
2
1 0 0
0 1 1
```

#### Output:
```
1
```

#### Explanation:
- For the first problem, only Petya is sure, so they will not implement it.
- For the second problem, Vasya and Tonya are sure, so they will implement it.

