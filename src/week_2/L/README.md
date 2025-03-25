# Problem L. AtCoder Quiz 2

## Problem Statement  
In the **Kingdom of AtCoder**, there is a standardized test for competitive programming proficiency.  
An examinee receives a **score (X) out of 100** and is assigned a **rank** based on the score:

- **Novice**: **0 ≤ X < 40**
- **Intermediate**: **40 ≤ X < 70**
- **Advanced**: **70 ≤ X < 90**
- **Expert**: **90 ≤ X ≤ 100**

Takahashi took the test and got **X** points.  
Find the **minimum number of extra points** he needs to **reach the next rank**.  
If Takahashi is already **Expert**, print `"expert"`.

---

## Constraints  
- **0 ≤ X ≤ 100** (X is an integer)

---

## Input Format  
A single integer **X**, representing the score.  

```
X
```

## Output Format  
Print the minimum extra points needed to reach the next rank.  
If the score is **90 or more**, print `"expert"`.  

---

## Examples  

### **Input 1**  
```
56
```  
### **Output 1**  
```
14
```  
**Explanation:**  
Takahashi got **56 points** (**Intermediate** rank).  
He needs **14 more points** to reach **70 (Advanced rank)**.  

---

### **Input 2**  
```
32
```  
### **Output 2**  
```
8
```  
**Explanation:**  
Takahashi got **32 points** (**Novice** rank).  
He needs **8 more points** to reach **40 (Intermediate rank)**.  

---

### **Input 3**  
```
0
```  
### **Output 3**  
```
40
```  
**Explanation:**  
Takahashi got **0 points** (**Novice** rank).  
He needs **40 points** to reach **40 (Intermediate rank)**.  

---

### **Input 4**  
```
100
```  
### **Output 4**  
```
expert
```  
**Explanation:**  
Takahashi got **100 points** (**Expert rank**).  
Since there is no higher rank, print `"expert"`.  