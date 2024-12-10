# Rust Notes

`self` refers to the current instance of a type (like `this` in other languages).  

`Self` Refers to the current type of the `impl` block or trait.  

## 🔥 Summary of Differences

| **Feature**       | **`self` (lowercase)**                        | **`Self` (uppercase)**                      |
|-------------------|-----------------------------------------------|--------------------------------------------|
| **What it refers to** | Current instance (like `this` in OOP)       | The current type of the `impl` block       |
| **Scope**           | Used in method signatures (`&self`, `self`)   | Used in type definitions (return, types)  |
| **Usage**           | Borrow, mutate, or consume the instance      | Return types, constructor, type aliasing  |
| **When used**       | Inside methods (requires an instance)       | Inside `impl` blocks (type context)       |
| **Example**         | `fn do_something(&self)`                     | `fn new() -> Self`                        |  

## 📝 **Rust Concepts: `*self` and `ref`**

---

### **1️⃣ `*self`**  

#### **Definition**:

- **`*self`** dereferences a reference (like `&self`) to get access to the underlying value.  
- It allows pattern matching on the **actual value** rather than the reference.

#### **Usage**:

```rust
match *self {
    Cons(_, ref tail) => { 
        // *self gives access to the actual enum value
    },
    Nil => { 
        // Pattern match on the Nil variant 
    }
}
```

## 📝 **Rust Concept: `ref`**

---

### **🔍 Definition**

- **`ref`** creates a reference to a variable in pattern matching, allowing access to its value **without taking ownership**.  
- It enables borrowing of the matched value instead of moving it.

---

### **💡 Usage**

```rust
match self {
    Cons(_, ref tail) => {
        // `ref tail` creates a reference to `tail` instead of moving it
    },
    Nil => { 
        // Base case 
    }
}
```



## 📝 **Difference Between `static` and `const` in Rust**

---

| **Feature**         | **`const`**                      | **`static`**                      |
|--------------------|-----------------------------------|-------------------------------------|
| **Memory Location** | No fixed memory location (inlined at each use) | Fixed memory location (same address for entire program) |
| **Lifetime**        | Exists only in the scope where it's used | Lives for the entire lifetime of the program |
| **Mutability**      | Always immutable                  | Can be mutable with `static mut` (unsafe) |
| **Initialization**  | Must be initialized with a **constant expression** | Can be initialized with runtime-computed values |
| **Usage**           | Compile-time constants (like `PI`, limits) | Global shared variables (like config, large data) |
| **Memory Overhead** | Each use has its own copy         | Single copy in memory, accessible from multiple places |
| **Access**          | Copied into each usage location   | Referenced via a fixed memory address |
| **Mutability Safety**| Safe (no unsafe required)        | Unsafe required for `static mut` access |
| **Data Size**       | Suitable for small, fixed-size data | Can hold large, complex data structures |
| **Common Use Cases**| Constants, magic numbers, simple expressions | Global configuration, shared state, large data |

---

### **📘 Summary**

- Use **`const`** for **small, compile-time constants** like numbers, limits, or magic values.  
- Use **`static`** for **global variables or large data** shared across the entire program.  
- **Avoid `static mut`** unless absolutely necessary, as it requires `unsafe` and can lead to data races.  

---

## `From` & `Into`

- From and Into traits are inherently linked  and this is actually part of its implementation
- If we can convert type A to type B then we can easily convert type B to type A

## Difference Between `..` and `..=` in Rust

| Operator | Name              | Description                        | Example        |
|----------|-------------------|------------------------------------|----------------|
| `..`     | Exclusive Range   | Creates a range excluding the end | `1..5` → `1, 2, 3, 4` |
| `..=`    | Inclusive Range   | Creates a range including the end | `1..=5` → `1, 2, 3, 4, 5` |
