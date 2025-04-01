## Problem F. Bear and Big Brother

### Problem Statement

Limak, the bear, wants to become strictly larger (heavier) than his brother Bob. 

Initially:
- Limak's weight is **a**.
- Bob's weight is **b**.
- It is guaranteed that `1 ≤ a ≤ b ≤ 10`.

Every year:
- Limak's weight triples (`a = a * 3`).
- Bob's weight doubles (`b = b * 2`).

Find the number of years it takes for Limak to become **strictly heavier** than Bob.

---

### Input
- A single line containing two integers **a** and **b** (`1 ≤ a ≤ b ≤ 10`).

### Output
- Print a single integer — the number of years after which Limak becomes strictly heavier than Bob.

---

### Examples

#### **Input 1**
```
4 7
```
#### **Output 1**
```
2
```
**Explanation:**
- Year 1: Limak = `4 * 3 = 12`, Bob = `7 * 2 = 14`
- Year 2: Limak = `12 * 3 = 36`, Bob = `14 * 2 = 28`
- Limak is now strictly heavier → **Answer: 2**

---

#### **Input 2**
```
4 9
```
#### **Output 2**
```
3
```
**Explanation:**
- Year 1: Limak = `4 * 3 = 12`, Bob = `9 * 2 = 18`
- Year 2: Limak = `12 * 3 = 36`, Bob = `18 * 2 = 36` (Equal, not strictly heavier)
- Year 3: Limak = `36 * 3 = 108`, Bob = `36 * 2 = 72` (Now strictly heavier)
- **Answer: 3**

---

#### **Input 3**
```
1 1
```
#### **Output 3**
```
1
```
**Explanation:**
- Year 1: Limak = `1 * 3 = 3`, Bob = `1 * 2 = 2` (Now strictly heavier)
- **Answer: 1**
