# Problem I. Domino Piling  

**Time limit:** 2000 ms  
**Memory limit:** 262144 kB  

### Problem Statement  
You are given a rectangular board of **M × N** squares.  
You also have an unlimited number of standard domino pieces of **2 × 1** squares.  
You are allowed to rotate the pieces.  

You must place as many dominoes as possible on the board while satisfying the following conditions:  

1. Each domino completely covers **two squares**.  
2. No two dominoes overlap.  
3. Each domino lies entirely **inside the board** (it can touch the edges).  

Find the **maximum number** of dominoes that can be placed under these restrictions.  

---

### Constraints  
- **1 ≤ M ≤ N ≤ 16**  

---

### Input  
A single line containing two integers **M** and **N** — the board dimensions in squares.  

```
M N
```

---

### Output  
Output a single integer — the **maximum number** of dominoes that can be placed.  

---

## Examples  

### **Sample 1**  
#### **Input**  
```
2 4
```
#### **Output**  
```
4
```

---

### **Sample 2**  
#### **Input**  
```
3 3
```
#### **Output**  
```
4
```