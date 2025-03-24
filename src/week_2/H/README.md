# Problem H. 500 Yen Coins

## Time limit  
2000 ms  

## Memory limit  
1048576 kB  

## Problem Statement  
Takahashi has **K** `500-yen` coins. If the total value of these coins is **X yen** or more, print **"Yes"**; otherwise, print **"No"**.  

## Constraints  
- **1 ≤ K ≤ 100** (Number of 500-yen coins)  
- **1 ≤ X ≤ 10⁵** (Target amount in yen)  

## Input  
A single line containing two integers:  
```
K X
```  
where:  
- **K** is the number of **500-yen** coins.  
- **X** is the required amount in yen.  

## Output  
Print **"Yes"** if the total value of the coins is at least **X**, otherwise print **"No"**.  

## Example  

### **Input 1**  
```
2 900
```  
### **Output 1**  
```
Yes
```  
**Explanation:**  
Two **500-yen** coins have a total value of **1000 yen**, which is **≥ 900 yen**.  

### **Input 2**  
```
1 501
```  
### **Output 2**  
```
No
```  
**Explanation:**  
One **500-yen** coin is worth **500 yen**, which is **< 501 yen**.  

### **Input 3**  
```
4 2000
```  
### **Output 3**  
```
Yes
```  
**Explanation:**  
Four **500-yen** coins have a total value of **2000 yen**, which is **≥ 2000 yen**.  
