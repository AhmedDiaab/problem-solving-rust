# Problem G. Day of Takahashi  

## Time limit  
2000 ms  

## Memory limit  
262144 kB  

## Problem Statement  
In **AtCoder Kingdom**, the **Gregorian calendar** is used, and dates are written in:  
- **"year-month-day"** format, or  
- **"month-day"** format (without the year).  

For example, May 3, 2018, is written as **2018-5-3** or simply **5-3** (without the year).  

A date is called **Takahashi** when the **month** and the **day** are equal as numbers.  
For example, **5-5** (May 5) is a **Takahashi** date.  

Your task is to count how many **Takahashi** dates exist **from** `2018-1-1` **to** `2018-a-b`.  

## Constraints  
- **1 ≤ a ≤ 12** (Month)  
- **1 ≤ b ≤ 31** (Day)  
- **2018-1-1 to 2018-a-b** is a valid date range in the **Gregorian calendar**.  

## Input  
A single line containing two integers:  
```
a b
```  
where **a** is the month and **b** is the day.  

## Output  
Print the **number of Takahashi dates** from **2018-1-1** through **2018-a-b**.  

## Example  

### **Input 1**  
```
5 5
```  
### **Output 1**  
```
5
```  
**Explanation:**  
The **Takahashi** dates in the range **2018-1-1 to 2018-5-5** are:  
- **1-1**  
- **2-2**  
- **3-3**  
- **4-4**  
- **5-5**  
There are **5** such dates.  

### **Input 2**  
```
1 1
```  
### **Output 2**  
```
1
```  
**Explanation:**  
The only **Takahashi** date in this range is **1-1**.  

### **Input 3**  
```
11 30
```  
### **Output 3**  
```
11
```  
**Explanation:**  
The **Takahashi** dates in the range **2018-1-1 to 2018-11-30** are:  
- **1-1**  
- **2-2**  
- **3-3**  
- **4-4**  
- **5-5**  
- **6-6**  
- **7-7**  
- **8-8**  
- **9-9**  
- **10-10**  
- **11-11**  
There are **11** such dates.  