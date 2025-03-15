# Problem J. Elephant  

**Time limit:** 1000 ms  
**Memory limit:** 262144 kB  

### Problem Statement  
An elephant decided to visit his friend.  
The elephant's house is located at **point 0**, and his friend's house is at **point x** (**x > 0**) on a coordinate line.  

In one step, the elephant can move **1, 2, 3, 4, or 5** positions forward.  

Determine the **minimum number of steps** the elephant needs to reach his friend's house.  

---

### Constraints  
- **1 ≤ x ≤ 1,000,000**  

---

### Input  
A single integer **x** — the coordinate of the friend's house.  

```
x
```

---

### Output  
Print the **minimum number of steps** the elephant needs to make to reach **x**.  

---

## Examples  

### **Sample 1**  
#### **Input**  
```
5
```
#### **Output**  
```
1
```

---

### **Sample 2**  
#### **Input**  
```
12
```
#### **Output**  
```
3
```

---

### **Note**  
- In the first sample, the elephant can take one step of length **5** to reach point **x**.  
- In the second sample, the elephant can reach **x** by taking steps of **3, 5, and 4** (other optimal combinations exist, but no solution requires fewer than **3 moves**).  