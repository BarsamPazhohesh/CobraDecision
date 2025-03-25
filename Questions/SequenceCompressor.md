# Sequence Compressor  
**March 24, 2025**  

This could be the easiest interview question you might be asked if you were applying for Microsoft.  

Imagine if you have a sequence of repeated characters like:  
```
AAAABBB1111CCCDD
```
Your goal is to find a way to compress this sequence and make it shorter while still being able to expand it back to the original.  

### Example:  
```
AAAABBB1111CCCDD => 4A 3B 41 3C 2D
```

## Challenges:  
- What if each character repeats in this range: \(10^3 < \text{char} < 10^{10}\)  
- How would we know each character's meaning? Example:  
  ```
  199A => 199*A or 1*9 9*A or 1 2*9 A
  ```
- If we use a delimiter like `;`, how would we know that it is a character of the sequence or it's actually the delimiter?  
