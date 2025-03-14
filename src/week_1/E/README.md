# Problem E. Discount Fare  

**Time limit:** 2000 ms  
**Memory limit:** 1048576 kB  

### Problem Statement  
There is a train going from Station A to Station B that costs **X** yen (the currency of Japan).  
Also, there is a bus going from Station B to Station C that costs **Y** yen.  

Joisino got a special ticket. With this ticket, she can take the bus for half the fare if she  
travels from Station A to Station B by train and then travels from Station B to Station C by bus.  

How much does it cost to travel from Station to Station if she uses this ticket?  

### Constraints  
- **X** and **Y** are integers.  
- **Y** is an even number.  
- **1 ≤ X, Y ≤ 100**  

### Input  
Input is given from Standard Input in the following format:  

```
X Y
```

### Output  
If it costs **Z** yen to travel from Station to Station, print **Z**.  

---

## Sample 1  

### **Input**  
```
81 58
```

### **Output**  
```
110
```

---

# Intro to Programming - Mar 11, 2025  

The train fare is **X** yen.  
The bus fare is **Y / 2** yen with the **50% discount**.  
Thus, it costs **Z** yen to travel from Station to Station.  

---

## Sample 2  

### **Input**  
```
4 54
```

### **Output**  
```
31
```

### **Explanation**  
```
Train fare: 81 yen  
Discounted bus fare: 58 / 2 = 29 yen  
Total cost: 81 + 29 = 110 yen
```