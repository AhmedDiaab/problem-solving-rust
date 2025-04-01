## Problem D. Bit++

### Problem Statement
The classic programming language of **Bitland** is **Bit++**. This language is peculiar because it has exactly one variable, called **x**, and two operations:
- `++` increases the value of variable **x** by 1.
- `--` decreases the value of variable **x** by 1.

A statement in **Bit++** is a sequence consisting of exactly one operation and one variable **x**. The statement is written without spaces, so it can only contain the characters `+`, `-`, and `X`.

When executing a statement, the operation is applied to **x**. The initial value of **x** is 0. The program consists of several statements, and executing the program means applying all the operations in sequence.

### Input
- The first line contains a single integer **n** (`1 ≤ n ≤ 150`) — the number of statements in the program.
- The next **n** lines each contain a statement. Each statement contains exactly one operation (`++` or `--`) and exactly one variable **x** (denoted as the letter `X`), with no spaces.

### Output
- Print a single integer — the final value of **x** after executing all the statements.

### Example 1
#### Input:
```
1
++X
```
#### Output:
```
1
```

### Example 2
#### Input:
```
2
X++
--X
```
#### Output:
```
0
```

### Explanation
- In the first example, we apply the `++X` operation, so the value of **x** becomes 1.
- In the second example, first `X++` increments **x** by 1, and then `--X` decrements **x** by 1, leaving the value of **x** at 0.

### Solution Approach
1. Initialize **x** to 0.
2. For each statement, check if it contains `++` or `--` and adjust the value of **x** accordingly.
3. Print the final value of **x**.