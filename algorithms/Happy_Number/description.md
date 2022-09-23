Write an algorithm to determine if a number `n` is happy.

A **happy number** is a number defined by the following process:

- Starting with any positive integer, replace the number by the sum of the squares of its digits.
- Repeat the process until the number equals 1 (where it will stay), or it **loops endlessly in a cycle** which does not include 1.
- Those numbers for which this process **ends in 1** are happy.

Return `true` *if* `n` *is a happy number, and* `false` *if not*.

**Example 1:**

<code style="white-space: break-spaces;">
  <strong>Input:</strong> n = 19
  
  <strong>Output:</strong> true
  <strong>Explanation:</strong>
  1² + 9² = 82
  8² + 2² = 68
  6² + 8² = 100
  1² + 0² + 0² = 1
</code>

**Example 2:**
```
**Input:** n = 2
Output: false
```

**Constraints:**
- <code>1 <= n <= 231 - 1</code>
