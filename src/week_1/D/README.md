# Problem D. Three Dice

**Time limit:** 2000 ms  
**Memory limit:** 1048576 kB  

### Problem Statement
Takahashi has rolled three dice. They are showing numbers \( a \), \( b \), and \( c \) on the top faces.  
Find the sum of the numbers on the bottom faces.

Each die is a standard cubic die, where the sum of the numbers on opposite faces is **7**.

### Constraints
- All values in input are integers.
- \( 1 <= a, b, c <= 6 \)

### Input
Input is given from Standard Input in the following format:
```
a b c
```
Where:
- \( a, b, c \) are the numbers on the top faces of the three dice.

### Output
Print the sum of the numbers on the bottom faces.

### Sample 1
**Input:**
```
1 4 3
```
**Output:**
```
13
```
**Explanation:**  
The numbers on the bottom faces are \( 7 - 1 = 6 \), \( 7 - 4 = 3 \), and \( 7 - 3 = 4 \).  
We have \( 6 + 3 + 4 = 13 \), which should be printed.

### Sample 2
**Input:**
```
5 6 4
```
**Output:**
```
6
```
**Explanation:**  
The numbers on the bottom faces are \( 7 - 5 = 2 \), \( 7 - 6 = 1 \), and \( 7 - 4 = 3 \).  
We have \( 2 + 1 + 3 = 6 \).
