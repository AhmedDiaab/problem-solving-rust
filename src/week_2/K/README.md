# Problem K. Even Odds

## Problem Statement  
Being a nonconformist, Volodya dislikes the current order of natural numbers.  
He decides to rearrange them in a special way:

1. He writes **all odd numbers** from **1 to n** (in ascending order).
2. Then, he writes **all even numbers** from **1 to n** (also in ascending order).

Your task is to determine which number appears at position **k** in this sequence.

---

## Constraints  
- **1 ≤ k ≤ n ≤ 10¹²**  
- Input consists of two integers **n** and **k**.  
- Please avoid using `%lld` in C++ to handle 64-bit integers. Use `cin, cout` or `%I64d`.  

---

## Input Format  
A single line containing two integers:  
```
n k
```

## Output Format  
Print the **k-th** number in the rearranged sequence.  

---

## Examples  

### **Input 1**  
```
10 3
```  
### **Output 1**  
```
5
```  
**Explanation:**  
The sequence for **n = 10**:  
`{1, 3, 5, 7, 9, 2, 4, 6, 8, 10}`  
The **3rd** number in this sequence is **5**.  

---

### **Input 2**  
```
7 7
```  
### **Output 2**  
```
6
```  
**Explanation:**  
The sequence for **n = 7**:  
`{1, 3, 5, 7, 2, 4, 6}`  
The **7th** number in this sequence is **6**.  
