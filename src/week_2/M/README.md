# Problem M. Vanishing Pitch

## Problem Statement  
Takahashi and Aoki are playing **baseball**.  
- **Takahashi** is the **pitcher**.  
- **Aoki** is the **batter**.  

Takahashi can throw an **invisible pitch**.  
- The ball moves **linearly** at a **constant speed (V meters per second)**.  
- The ball becomes **invisible** between **T** and **S** seconds after being thrown (inclusive).  
- The ball keeps moving even when it is **invisible**.

If the ball is **not invisible** when it is exactly **D meters away** from Takahashi, Aoki can **hit** it.  
Otherwise, he **cannot hit** it.  

---

## Constraints  
- **1 ≤ V ≤ 1000** (Speed of the ball in m/s)  
- **1 ≤ T < S ≤ 1000** (Invisible time range)  
- **1 ≤ D ≤ 1000** (Distance to Aoki)  
- All values in input are **integers**.

---

## Input Format  
A single line with four integers:  

```
V T S D
```
- **V** (speed of the ball in m/s)  
- **T** (start of invisibility in seconds)  
- **S** (end of invisibility in seconds)  
- **D** (distance to Aoki in meters)  

---

## Output Format  
Print **"Yes"** if Aoki can hit the ball, otherwise print **"No"**.  

---

## Examples  

### **Input 1**  
```
10 3 5 20
```  
### **Output 1**  
```
Yes
```  
**Explanation:**  
- The ball moves at **10 m/s**.  
- The ball reaches **20 meters** at **t = 20 / 10 = 2 seconds**.  
- The ball is invisible between **t = 3 and t = 5** seconds.  
- Since **t = 2** is **before** the invisible period, Aoki **can hit** the ball.  

---

### **Input 2**  
```
10 3 5 30
```  
### **Output 2**  
```
No
```  
**Explanation:**  
- The ball moves at **10 m/s**.  
- The ball reaches **30 meters** at **t = 30 / 10 = 3 seconds**.  
- The ball is invisible between **t = 3 and t = 5** seconds.  
- Since **t = 3** is within the invisible period, Aoki **cannot hit** the ball.  
