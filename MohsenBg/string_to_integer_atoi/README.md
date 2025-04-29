# 🧠 Problem 8: String to Integer (atoi)

**Difficulty**: Medium  
**Tags**: String, Parsing, Simulation

---

## 📝 Description

Implement the `myAtoi(string s)` function, which converts a string to a **32-bit signed integer**.

### Conversion Steps

1. **Whitespace**: Ignore any leading whitespace (`' '`).
2. **Signedness**: Check for `'-'` or `'+'` sign (assume positive if neither present).
3. **Conversion**: Convert digits into an integer until a non-digit character is found.
4. **Rounding**: Clamp the result to the 32-bit signed integer range:  
   `[−2³¹, 2³¹ − 1] => [-2147483648, 2147483647]`.

---

## 📥 Examples

### Example 1
**Input**:  
```text
s = "42"
```

**Output**:  
```text
42
```

### Example 2
**Input**:  
```text
s = "   -042"
```

**Output**:  
```text
-42
```

### Example 3
**Input**:  
```text
s = "1337c0d3"
```

**Output**:  
```text
1337
```

### Example 4
**Input**:  
```text
s = "0-1"
```

**Output**:  
```text
0
```

### Example 5
**Input**:  
```text
s = "words and 987"
```

**Output**:  
```text
0
```

---

## 📌 Constraints

- `0 <= s.length <= 200`
- `s` consists of English letters (upper and lowercase), digits, `' '`, `'+'`, `'-'`, and `'.'`.


# Awnser
#### [Rust Code](./src/lib.rs) 📄💻