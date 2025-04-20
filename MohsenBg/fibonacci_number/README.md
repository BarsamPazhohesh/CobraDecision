# Fibonacci Number

The Fibonacci numbers, commonly denoted `F(n)` form a sequence, called the **Fibonacci sequence**, such that each number is the sum of the two preceding ones, starting from `0` and `1`. That is,

```
F(0) = 0  
F(1) = 1  
F(n) = F(n - 1) + F(n - 2), for n > 1  
```

Given `n`, calculate `F(n)`.

---

### Example 1:
**Input:** `n = 2`  
**Output:** `1`  
**Explanation:** `F(2) = F(1) + F(0) = 1 + 0 = 1`

---

### Example 2:
**Input:** `n = 3`  
**Output:** `2`  
**Explanation:** `F(3) = F(2) + F(1) = 1 + 1 = 2`

---

### Example 3:
**Input:** `n = 4`  
**Output:** `3`  
**Explanation:** `F(4) = F(3) + F(2) = 2 + 1 = 3`

---

### Constraints:
- `0 <= n <= 30`




## Answer
the best solution i found is using matrix power and multiplication — this gives us **O(log n)** time complexity.  
but let me explain how it actually works real quick.

---

### 2x2 Matrix Multiplication
first we write a function to multiply two 2x2 matrices using this formula:

we have two 2×2 matrices **A** and **B**:

```
A = | a11  a12 |     B = | b11  b12 |
    | a21  a22 |         | b21  b22 |
```

so the result of this multiplication is **C = A × B** looks like this:

```
C = | c11  c12 |
    | c21  c22 |
```

and formal:

```
c11 = a11 * b11 + a12 * b21  
c12 = a11 * b12 + a12 * b22  
c21 = a21 * b11 + a22 * b21  
c22 = a21 * b12 + a22 * b22  
```

---

### example:

let:

```
A = | 1  2 |     B = | 3  4 |
    | 5  6 |         | 7  8 |
```

then:

```
C = A × B = | (1*3 + 2*7)   (1*4 + 2*8) |  
            | (5*3 + 6*7)   (5*4 + 6*8) |

          = | 17  20 |
            | 51  60 |
```

---

### Now let’s write the function for powering the matrix

but here we use a trick instead of multiplying the matrix by itself multiple times. we can just square the matrix and reduce the number of multiplications.  
for example, if we want to find **M^4** of a matrix, we can do this:

instead of:

```
M = M . M . M . M
```

we can declare it like this:

```
M^2 = M . M
M^4 = (M^2) * (M^2)
```

or, in short, **(M^2)^2**.

for odd values like **M^9**, we can calculate **M^8** using the last formula and multiply it by **M**. see this:

```
M^2 = M * M
M^4 = M^2 * M^2
M^8 = M^4 * M^4 
M^9 = M^8 * M
```

now we only use 4 multiplications instead of 8. this is much more efficient compared to:

```
M^9 = M * M * M * M * M * M * M * M
```

### Matrix Representation of Fibonacci Numbers
now, if we take look at the matrix form of Fibonacci, we can see something really interesting.

the Fibonacci numbers can be calculate with a 2x2 matrix raised to a power. check this out:

```
[1 1]^n = [F(n+1) F(n) ]
[1 0]     [F(n)   F(n-1)]
```

this means, if we take the matrix:

```
[1 1]
[1 0]
```

and raise it to the **n-th** power, we get a new matrix where:

- **F(n+1)** is in the top-left corner
- **F(n)** is in the top-right corner
- **F(n)** is in the bottom-left corner
- **F(n-1)** is in the bottom-right corner

---

### How This Helps

this is really useful cause instead of calculating the Fibonacci numbers one by one (which take linear time **O(n)**), we only need to multiply this 2x2 matrix **n** times, which takes **O(log n)** time using matrix exponentiation!

so for example:

- If you want to find **F(10)**, we can raise the matrix to the **10th** power, and the result will give us **F(11)** and **F(10)** in the matrix.
- The top-right value will give us **F(n)** directly.

instead of doing something like this:

```
F(0) = 0
F(1) = 1
F(2) = F(1) + F(0)
F(3) = F(2) + F(1)
F(4) = F(3) + F(2)
... 
```

we just multiply the matrix by itself and get the result fast.


### Example:

let’s take a quick example to make this clearer. suppose we want to calculate **F(5)**. we can do the following:

start with the matrix:

```
[1 1]
[1 0]
```

then, we raise it to the **5th** power:

```
[1 1]^5 = [F(6) F(5)]
[1 0]     [F(5) F(4)]
```

after performing the matrix multiplication, the result will be:

```
[F(6) F(5)]
[F(5) F(4)]
```

You can check the code here:  
- 🦀 [Rust Code — O(n log n)](./rust/src/lib.rs)  
- ⚙️ [Assembly Code — O(n)](./asm/fibonacci.asm)
