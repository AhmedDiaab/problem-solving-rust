# Problem C. George and Accommodation

## Problem Statement  
George has recently entered the **BSUCP (Berland State University for Cool Programmers)**.  
His friend **Alex** has also joined, and now they are looking for a **dormitory room** where they can stay together.

There are **n** rooms in total.  
Each room currently has **pᵢ** people living in it and can accommodate up to **qᵢ** people.  
A room is **suitable** if it has space for **at least two** more people (i.e., if **qᵢ - pᵢ ≥ 2**).  

Determine how many rooms **have space** for both George and Alex.

---

## Constraints  
- **1 ≤ n ≤ 100**  
- **0 ≤ pᵢ ≤ qᵢ ≤ 100**  

---

## Input Format  
- The first line contains **n** — the number of rooms.  
- The next **n** lines each contain two integers **pᵢ** and **qᵢ** —  
  - **pᵢ**: number of people currently in the **i-th** room  
  - **qᵢ**: maximum capacity of the **i-th** room  

---

## Output Format  
Print a **single integer** — the number of rooms where both **George and Alex** can move in.

---

## Examples  

### **Input 1**  
```
3
1 1
2 2
3 3
```  
### **Output 1**  
```
0
```  
**Explanation:**  
All rooms are **already full** (no room has at least 2 free spots).  

---

### **Input 2**  
```
3
1 10
0 5
2 3
```  
### **Output 2**  
```
2
```  
**Explanation:**  
- Room 1: **1/10** → **9 spaces available** ✅  
- Room 2: **0/5** → **5 spaces available** ✅  
- Room 3: **2/3** → **only 1 space available** ❌  
Total valid rooms = **2**.

---

## Solution Approach  
- Read **n** (number of rooms).  
- Iterate through each room and check if **qᵢ - pᵢ ≥ 2**.  
- Count and print the number of valid rooms.  
