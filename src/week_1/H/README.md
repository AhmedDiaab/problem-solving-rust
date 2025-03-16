# Problem H. kcal  

**Time limit:** 2000 ms  
**Memory limit:** 1048576 kB  

### Problem Statement  
We have a drink that contains **A** kilocalories of energy per **100** milliliters.  
How many kilocalories of energy does **B** milliliters of this drink have?  

### Constraints  
- **0 ≤ A, B ≤ 1000**  
- All values in input are integers.  

### Input  
Input is given from Standard Input in the following format:  

```
A B
```

### Output  
Print a number representing the answer.  

Your output is considered correct when its absolute or relative error from our answer is at most **10⁻⁶**.  

---

## Sample 1  

### **Input**  
```
45 200
```

### **Output**  
```
90
```

### **Explanation**  
```
(45 / 100) * 200 = 90
```

---

## Sample 2  

### **Input**  
```
37 450
```

### **Output**  
```
166.5
```

### **Explanation**  
```
(37 / 100) * 450 = 166.5
```
*Note: The answer may not be an integer.*

---

## Sample 3  

### **Input**  
```
0 1000
```

### **Output**  
```
0
```

---

## Sample 4  

### **Input**  
```
50 0
```

### **Output**  
```
0
```