# Problem C. Alloy  

## Time limit  
2000 ms  

## Memory limit  
1048576 kB  

## Problem Statement  
Takahashi melted and mixed **A** grams of gold and **B** grams of silver to produce a new metal.  
What metal did he produce: pure gold, pure silver, or an alloy?  

Formally, the product is classified as follows:  
- **Pure gold** if **A > 0** and **B = 0**.  
- **Pure silver** if **A = 0** and **B > 0**.  
- **Alloy** if **A > 0** and **B > 0**.  

## Constraints  
- **0 ≤ A, B ≤ 100**  
- **1 ≤ A + B**  
- All values in input are integers.  

## Input  
The input is given from Standard Input in the following format:  
```
A B
```  

## Output  
- If the product is **pure gold**, print `"Gold"`.  
- If it is **pure silver**, print `"Silver"`.  
- If it is an **alloy**, print `"Alloy"`.  

## Sample 1  
**Input**  
```
50 50
```  
**Output**  
```
Alloy
```  
### Explanation  
Since **A > 0** and **B > 0**, the product is an **alloy**.  

## Sample 2  
**Input**  
```
100 0
```  
**Output**  
```
Gold
```  
### Explanation  
Since **A > 0** and **B = 0**, the product is **pure gold**.  

## Sample 3  
**Input**  
```
0 100
```  
**Output**  
```
Silver
```  
### Explanation  
Since **A = 0** and **B > 0**, the product is **pure silver**.  

## Sample 4  
**Input**  
```
100 2
```  
**Output**  
```
Alloy
```  
### Explanation  
Since **A > 0** and **B > 0**, the product is an **alloy**.  
