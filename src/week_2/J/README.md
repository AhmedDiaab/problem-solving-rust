# Problem J. Very Very Primitive Game

## Problem Statement  
Takahashi and Aoki will play a game against each other.  
Initially, Takahashi has **A** candies, and Aoki has **B** candies.  

They will take turns eating **one** candy at a time.  
- If **C = 0**, Takahashi goes first.  
- If **C = 1**, Aoki goes first.  

The person who cannot eat a candy (i.e., runs out of candies first) **loses**.  
Determine who will **win** the game.

## Constraints  
- **0 ≤ A, B ≤ 100**  
- **C ∈ {0,1}**  
- All values in input are integers.  

## Input Format  
A single line containing three integers:  
```
A B C
```

## Output Format  
- If Takahashi wins, print `"Takahashi"`.  
- If Aoki wins, print `"Aoki"`.  

## Examples  

### **Input 1**  
```
2 1 0
```  
### **Output 1**  
```
Takahashi
```  
**Explanation:**  
1. Takahashi eats a candy. *(A = 1, B = 1)*  
2. Aoki eats a candy. *(A = 1, B = 0)*  
3. Takahashi eats a candy. *(A = 0, B = 0)*  
4. Aoki has no more candies left → **Takahashi wins**.  

---

### **Input 2**  
```
2 2 0
```  
### **Output 2**  
```
Aoki
```  
**Explanation:**  
1. Takahashi eats a candy. *(A = 1, B = 2)*  
2. Aoki eats a candy. *(A = 1, B = 1)*  
3. Takahashi eats a candy. *(A = 0, B = 1)*  
4. Aoki eats a candy. *(A = 0, B = 0)*  
5. Takahashi has no more candies left → **Aoki wins**.  

---

### **Input 3**  
```
2 2 1
```  
### **Output 3**  
```
Takahashi
```  
**Explanation:**  
1. Aoki eats a candy first. *(A = 2, B = 1)*  
2. Takahashi eats a candy. *(A = 1, B = 1)*  
3. Aoki eats a candy. *(A = 1, B = 0)*  
4. Takahashi eats a candy. *(A = 0, B = 0)*  
5. Aoki has no more candies left → **Takahashi wins**.  
