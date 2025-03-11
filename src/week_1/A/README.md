# Problem A. Brick

**Time limit:** 2000 ms  
**Memory limit:** 1048576 kB  

### Problem Statement
We have a truck, which can carry at most **N** kilograms.  
We will load bricks onto this truck, each of which weighs **W** kilograms.  
At most how many bricks can be loaded?

### Constraints
- **1 ≤ N, W ≤ 1000**

### Input
Input is given from Standard Input in the following format:
```
N W
```

### Output
Print an integer representing the maximum number of bricks that can be loaded onto the truck.

### Sample 1
#### Input:
```
10 3
```
#### Output:
```
3
```
**Explanation:** Each brick weighs 3 kilograms, so 3 bricks weigh 9 kilograms, and 4 bricks weigh 12 kilograms. Thus, we can load at most 3 bricks onto the truck that can carry at most 10 kilograms.

---

### Sample 2
#### Input:
```
1000 1
```
#### Output:
```
1000
