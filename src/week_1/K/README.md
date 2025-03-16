# Problem K. Wizards' Duel  

**Time limit:** 2000 ms  
**Memory limit:** 262144 kB  

---

### **Problem Statement**  
Harry Potter and He-Who-Must-Not-Be-Named are dueling again.  

They are positioned at **opposite ends** of a **corridor** of length **l**.  
Each wizard casts a **magical spell impulse** toward the enemy.  

- Harry's spell moves at a speed of **p** meters per second.  
- Voldemort's spell moves at a speed of **q** meters per second.  

The spells move toward each other and **collide**. At the moment of collision, they turn around and **return** to the caster at the same speed.  

Each wizard then **reflects** the returning spell, sending it back toward the opponent.  

After the **second collision**, the spells **disappear** in a powerful explosion.  

Harry asks you to calculate the **distance from his position** to the place where the **second collision** occurs.

---

### **Constraints**  
- **1 ≤ l ≤ 1000**  
- **1 ≤ p, q ≤ 500**  

---

### **Input**  
The input consists of **three integers**, one per line:  
```
l
p
q
```
- **l** — the length of the corridor.  
- **p** — speed of Harry’s spell.  
- **q** — speed of Voldemort’s spell.  

---

### **Output**  
Print a **single real number** — the distance from **Harry's position** to the place where the **second collision** occurs.  

Your answer is considered correct if its **absolute or relative error does not exceed 10⁻⁴**.  

---

## **Examples**  

### **Sample 1**  
#### **Input**  
```
100
50
50
```
#### **Output**  
```
50
```

---

### **Sample 2**  
#### **Input**  
```
199
60
40
```
#### **Output**  
```
119.4
```

---

### **Explanation**  
- In the first example, since both spells travel at **equal speeds**, they collide at the **midpoint** of the corridor (50 meters from Harry's position).  
- In the second example, the spells travel at different speeds, so the **first collision** occurs at a position determined by the ratio of their speeds.  

The **formula** for the second collision point from Harry's position:  
$$
\text{Distance} = \frac{2 \times p \times l}{p + q}
$$

