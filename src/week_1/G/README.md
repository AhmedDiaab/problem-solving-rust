# Problem G. Blood Pressure  

**Time limit:** 2000 ms  
**Memory limit:** 1048576 kB  

### Problem Statement  
You are given a person's systolic blood pressure **A** and diastolic blood pressure **B**.  
Find the mean arterial pressure **C**, which we define as follows:  


C = ((A - B) / 3) + B


### Constraints  
- **50 ≤ B ≤ A ≤ 300**  
- All values in input are integers.  

### Input  
Input is given from Standard Input in the following format:  

```
A B
```

### Output  
Print the value **C**.  

Your output is considered correct when its absolute or relative error from our answer is at most **10⁻⁵**.  

---

## Sample 1  

### **Input**  
```
130 100
```

### **Output**  
```
110
```

### **Explanation**  
```
C = (130 - 100) / 3 + 100  
  = 10 + 100  
  = 110
```

---

## Sample 2  

### **Input**  
```
300 50
```

### **Output**  
```
133.3333333
```

### **Explanation**  
```
C = (300 - 50) / 3 + 50  
  = 83.3333333 + 50  
  = 133.3333333
```

---

## Sample 3  

### **Input**  
```
123 123
```

### **Output**  
```
123
```