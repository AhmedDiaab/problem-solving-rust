# Problem C. Box

**Time limit:** 2000 ms  
**Memory limit:** 1048576 kB  

### Problem Statement
We have removed balls from a box that contained \( N \) balls and then put \( B \) new balls into that box. How many balls does the box contain now?

### Constraints
- All values in input are integers.
- \( 100 <= N  <= 200  \)
- \( 1 <= A , B <= 100  \)

### Input
Input is given from Standard Input in the following format:
```
N A B
```
Where:
- \( N \) is the initial number of balls.
- \( A \) is the number of balls removed.
- \( B \) is the number of balls added.

### Output
Print the final number of balls as an integer.

### Sample 1
**Input:**
```
100 1 2
```
**Output:**
```
101
```
**Explanation:**  
The box contained 100 balls. After removing 1 ball and adding 2 new balls, it now contains 101 balls.

### Sample 2
**Input:**
```
100 2 1
```
**Output:**
```
99
```

### Sample 3
**Input:**
```
100 1 1
```
**Output:**
```
100
```
