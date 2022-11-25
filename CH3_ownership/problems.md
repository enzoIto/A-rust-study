# Questions about Ownership and Memory Management

## 1 -  If a resource has multiple owners, what can be an issue with it? How does Rust solve this problem? Provide an example.

It depends. If the data type that is being passed is **integer**, **Bool** or **chars**, rust then copies the value. For data types like **vector** and **String** it moves the value.

### Example:
      let a = 1;
      let b = a;
Here, **a** has the value **1** and them is copied to **b**. **a** has the ownership of **1** and **b** is a copy of **a**.

## 2 - Among stack and heap memory allocation and de-allocation, which one is typically faster, and why?

Stack is faster because doesn't need a pointer to reach the data, like a heap.

## 3 - Write a program in Rust, which takes as input a reference to a String and returns a reference to the last word in the given input string. Assume that the input string contains one or more words separated by single space characters.

