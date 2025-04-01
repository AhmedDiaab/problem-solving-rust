# Problem B. qwerty

## Problem Statement  
You are given a **permutation** of integers **P** consisting of numbers from **1** through **26**.  
It is guaranteed that **all elements are distinct**.  

Print a **string of length 26** that satisfies the following condition:  
- For every **i** (1 ≤ i ≤ 26), the **i-th character** of the output string is the **Pᵢ-th** lowercase English letter.

---

## Constraints  
- **1 ≤ Pᵢ ≤ 26**  
- **P** is a permutation of `{1, 2, ..., 26}`  

---

## Input Format  
A single line with **26 integers** (a permutation of `{1, 2, ..., 26}`):

```
P₁ P₂ ... P₂₆
```

---

## Output Format  
A single string **S** of length **26**.

---

## Examples  

### **Input 1**  
```
1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26
```  
### **Output 1**  
```
abcdefghijklmnopqrstuvwxyz
```  
**Explanation:**  
Each number **Pᵢ** corresponds to the **i-th** letter in alphabetical order.

---

### **Input 2**  
```
2 1 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26
```  
### **Output 2**  
```
bacdefghijklmnopqrstuvwxyz
```  
**Explanation:**  
- **P₁ = 2 → 'b'**  
- **P₂ = 1 → 'a'**  
- The rest are in order.

---

### **Input 3**  
```
5 11 12 16 25 17 18 1 7 10 4 23 20 3 2 24 26 19 14 9 6 22 8 13 15 21
```  
### **Output 3**  
```
eklpyqragjdwtcbxzsnifvhmou
```  
**Explanation:**  
Each **Pᵢ** maps to the corresponding letter in order.

---

## Solution Approach  
The output string **S** is constructed such that:  
- **S[i] = the P[i]-th lowercase letter ('a' to 'z')**  
