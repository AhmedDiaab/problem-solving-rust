# Problem A. Children and Candies (ABC Edit)

## Problem Statement  
There are **N** children in **AtCoder Kindergarten**.  

Mr. Evi will arrange them in a line and distribute candies as follows:  
- The **1st** child gets **1** candy.  
- The **2nd** child gets **2** candies.  
- The **N-th** child gets **N** candies.  

Find the **total number of candies** needed.

---

## Constraints  
- **1 ≤ N ≤ 100**  

---

## Input Format  
A single integer **N**:  

```
N
```

---

## Output Format  
Print the total number of candies needed.

---

## Examples  

### **Input 1**  
```
3
```  
### **Output 1**  
```
6
```  
**Explanation:**  
- **Candies given:** `1 + 2 + 3 = 6`  
- The answer is **6**.

---

### **Input 2**  
```
10
```  
### **Output 2**  
```
55
```  
**Explanation:**  
- **Candies given:** `1 + 2 + 3 + ... + 10 = 55`  
- The answer is **55**.

---

### **Input 3**  
```
1
```  
### **Output 3**  
```
1
```  
**Explanation:**  
- Only **one child**, receiving **1** candy.  
- The answer is **1**.

---

## Formula  
The total number of candies follows the sum of the first **N** natural numbers:  
$$
\text{Total Candies} = \frac{N \times (N + 1)}{2}
$$