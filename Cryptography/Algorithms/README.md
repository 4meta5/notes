# Algorithms
> Notes from [Competitive Programmer's Handbook](https://cses.fi/book/index.html) by Antti Laaksonen; my goal is to implement in Rust using C++ code provided in the book...I'll throw down some notes here though.

* [Maximum subarray sum](#maxsubsum)

## Complexity Classes

- *O(1)* The running time of a **constant-time** algorithm doesn't depend on the input size. This is typically a direct formula that calculates the answer.
- *O(log(n))* A **logarithmic** algorithm often halves the input size at each step.
- *O(\sqrt{n})* A **square root** algorithm is interesting because *\sqrt{n} = \frac{n}{\sqrt{n}}*, so the square root *\sqrt{n}* usually lies in the middle of the input.
- *O(n)* A **linear** algorithm goes through the input a constant number of times.
- *O(nlog(n))* A **quasi-linear** time complexity indicates that the algorithm sorts the input. Another possibility is that the algorithm uses a data structure where each operation takes *O(log(n))* time.
- *O(n^{2})* A **quadratic** algorithm often contains two nested loops. It is possible to go through all pairs of the input elements in *O(n^{2})* time.
- *O(n^{3})* A **cubic** algorithm contains three nested loops. It is possible to go through all triplets of the input elements in *O(n^{3})* time.
- *O(2^{n})* **exponential** time complexity indicates that the algorithm iterates through all subsets of the input elements.
- *O(n!)* This time complexity indicates that the algorithm iterates through all permutations of the input elements.

An algorithm is **polynomial** if the time complexity is at most *O(n^{k})* where *k* is constant. **NP-hard** problems are an important set of problems for which no polynomial algorithm is known.

## Maximum Subarray Sum <a name="maxsubsum"></a>

Given an array of *n* numbes, can we calculate the **maximum subarray sum**, i.e. the largest possible sum of a sequence of consecutive values in the array. This problem becomes interesting when the array contains both positive and negative values.

The straightforward solution entails going through all possible subarrays, calculating the sum of values in each subarray, and maintaining the maximum sum.

> consider implementing in Rust and placing the code in a separate folder; then linking to the given code