# Problem B. +-x  

## Time limit  
2000 ms  

## Memory limit  
1048576 kB  

## Problem Statement  
We have two integers: **A** and **B**.  
Print the largest number among **A + B, A - B,** and **A × B**.  

## Constraints  
- **-100 ≤ A, B ≤ 100**  
- All values in input are integers.  

## Input  
The input is given from Standard Input in the following format:  
```
A B
```  

## Output  
Print the largest number among **A + B, A - B,** and **A × B**.  

## Sample 1  
**Input**  
```
-13 3
```  
**Output**  
```
-10
```  
### Explanation  
A + B = -10  
A - B = -16  
A × B = -39  
The largest number is **-10**.  

## Sample 2  
**Input**  
```
1 -33
```  
**Output**  
```
34
```  
### Explanation  
A + B = -32  
A - B = 34  
A × B = -33  
The largest number is **34**.  

## Sample 3  
**Input**  
```
13 3
```  
**Output**  
```
39
```  
### Explanation  
A + B = 16  
A - B = 10  
A × B = 39  
The largest number is **39**.  