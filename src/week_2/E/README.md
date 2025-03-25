# Problem E. Theatre Square  

## Time limit  
1000 ms  

## Memory limit  
262144 kB  

## Problem Statement  
Theatre Square in the capital city of Berland has a rectangular shape with dimensions **n × m** meters.  
For the city's anniversary, a decision was made to pave the Square with square granite flagstones.  
Each flagstone is of size **a × a** meters.  

What is the **minimum** number of flagstones needed to completely cover the Square?  
- You **can** cover an area **larger** than the Square if necessary.  
- You **cannot** break the flagstones.  
- The sides of the flagstones must be **parallel** to the sides of the Square.  

## Constraints  
- **1 ≤ n, m, a ≤ 10⁹**  

## Input  
A single line containing three positive integers:  
```
n m a
```  
where:  
- **n** is the length of the Square in meters.  
- **m** is the width of the Square in meters.  
- **a** is the side length of each square flagstone in meters.  

## Output  
Print a single integer — the **minimum** number of flagstones required.  

## Example  
**Input**  
```
6 6 4
```  
**Output**  
```
4
```  

### Explanation  
- The Theatre Square is **6 × 6** meters.  
- Each flagstone is **4 × 4** meters.  
- You need **2 flagstones** to cover the length (**6 ÷ 4 = 2** flagstones, rounded up).  
- You need **2 flagstones** to cover the width (**6 ÷ 4 = 2** flagstones, rounded up).  
- **Total flagstones needed = 2 × 2 = 4**.  