# Problem N. Rainy Season

## Problem Statement  
AtCoder Town has weather records for **three consecutive days**.  

The weather is represented as a **string (S) of length 3**, where:  
- `'S'` means **sunny**.  
- `'R'` means **rainy**.  

Find the **maximum number of consecutive rainy days** in this period.

---

## Constraints  
- The string **S** has exactly **3** characters (`|S| = 3`).  
- Each character of **S** is either `'S'` or `'R'`.

---

## Input Format  
A single line containing a string of length **3**:  

```
S
```

---

## Output Format  
Print the **maximum** number of consecutive rainy days.

---

## Examples  

### **Input 1**  
```
RRS
```  
### **Output 1**  
```
2
```  
**Explanation:**  
- **Days:** `R R S`  
- The first **two days** were rainy (`RR`), forming a **streak of 2**.  
- The third day was sunny.  
- The longest streak of rainy days is **2**.

---

### **Input 2**  
```
SSS
```  
### **Output 2**  
```
0
```  
**Explanation:**  
- **Days:** `S S S`  
- There were **no rainy days**, so the answer is **0**.

---

### **Input 3**  
```
RSR
```  
### **Output 3**  
```
1
```  
**Explanation:**  
- **Days:** `R S R`  
- There were **two separate rainy days**, but no consecutive streak longer than **1**.  
- The longest streak of rainy days is **1**.  