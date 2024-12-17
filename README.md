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

## **Rust Pattern Matching: Why `&` and `*` Can Be Omitted**

---

### **Key Insight: Rust's Match Ergonomics**

Rust automatically handles **borrowing** and **dereferencing** in `match` patterns, making the use of `&` and `*` optional in many cases. This is due to **automatic dereferencing** and **pattern matching ergonomics**.

---
### **Example 1: Matching a Reference Without `&`**  

```rust
let reference = &4;

match reference {
    val => println!("Got a value via destructuring: {:?}", val),
}
```

#### **Why It Works:**

- `reference` is of type `&i32` (a reference).
- The pattern `val` expects `i32`, **not `&i32`**.
- **Rust automatically dereferences** `reference` behind the scenes:

  ```rust
  let val = *reference;  // Auto-dereferencing happens
  ```

- This removes the need to explicitly match `&val`.

---

### **Example 2: Dereferencing Without `*`**

```rust
let reference = &4;

match reference {
    val => println!("Got a value via dereferencing: {:?}", val),
}
```

#### **Why It Works:**

- The value of `reference` is `&i32`.
- Rust sees that `reference` is a reference but expects `i32`.
- Rust automatically **dereferences** `reference` behind the scenes:

  ```rust
  match *reference {  // Auto-dereferencing happens
      val => println!("Got a value via dereferencing: {:?}", val),
  }
  ```

---

### **How Rust's Matching Works**

Rust applies these rules when matching references:

- **If the matched value is a reference (`&T`), and the pattern expects `T`, Rust automatically dereferences.**
- **If the matched value is not a reference but the pattern expects one, Rust automatically borrows.**

---

### **When to Use `&val` or `*reference` Explicitly**

- Use `&val` if you want to **manually destructure** a reference in a pattern.
- Use `*reference` if you want to **manually dereference** a reference in an expression.
- Use these only when **you want explicit control** over how references are handled.

---

### **Summary**

Rust's match ergonomics simplify how you work with references by:

- Automatically borrowing or dereferencing when needed.
- Allowing you to omit `&` or `*` in most match statements.
- Making your code more readable and concise.

## **Rust Match Guards**

- A match guard in Rust is an additional conditional check that you can apply to a match arm using the if keyword. It acts as a filter to decide whether a particular arm should be selected when the pattern matches.

## 🔍 **Difference between `if-let` and `let-else` in Rust

| **Criteria**           | **`if-let`**                            | **`let-else`**                          |
|-----------------------|------------------------------------------|------------------------------------------|
| **Purpose**            | Pattern matching with optional control flow | Pattern matching with early exit  |
| **Syntax**             | `if let Pattern = value { ... } else { ... }` | `let Pattern = value else { ... };` |
| **Usage**              | Used when you want to handle both success and failure cases | Used when you want to exit early on failure |
| **Control Flow**       | Continues with program flow after handling the pattern | Exits early using `return`, `break`, or `panic!` |
| **Requires an `else`?**| No, but you can add it optionally         | **Yes, `else` is mandatory**            |
| **Early Return?**      | ❌ No, unless explicitly written          | ✅ Yes, it requires early return logic  |
| **Readability**        | Cleaner for inline flow control         | Cleaner for early-exit control flow     |
| **When to Use?**       | When you want to handle both branches    | When you want to exit early on failure  |

---

### 🧠 **When to Use Which?**

| **Use Case**           | **Which to Use?**             | **Reason**                              |
|-----------------------|---------------------------------|------------------------------------------|
| Handle both cases (`Some` / `None`) | **`if-let`**     | Cleaner when handling both possibilities |
| Early return or exit on `None`      | **`let-else`**   | Clean, avoids unnecessary indentation   |
| No need for `else` clause           | **`if-let`**     | `else` is optional for `if-let`         |
| Explicit early return, break, or panic | **`let-else`** | Built for early exits like `return`    |

## **Rust Modules**

- Rust provided a powerful module system that can be used to hierarchically split code in logical units
(modules), and manage visibility (public/private) between them.

- A module is a collection of items: functions, structs, trait, impl blocks and ever other modules

## **Rust Crates**

- A crate is Rust's compilation unit, either a binary or library. Modules are inlined before compilation. Only crates are compiled; use `--crate-type` to specify a library.  

    ```rust
    rustc --crate-type=lib rary.rs
    rustc executable.rs --extern rary=library.rlib && executable
    ```

## **Rust Cargo**

`cargo` is the official Rust Package management tool. Features of `cargo` includes :-

- Dependency Management and integration with crates.io.
- Awareness of unit tests.
- Awareness of benchmarks.

## **Rust Conditional Configuration Checks**

Configuration conditional checks are possible through two different operators:

1. **The `cfg` attribute:** `#[cfg(...)]` in attribute position
2. **The `cfg!` macro:** `cfg!(...)` in boolean expressions

While the former enables conditional compilation, the latter conditionally evaluates to `true` or `false` literals allowing for checks at run-time. Both utilize identical argument syntax.

`cfg!`, unlike `#[cfg]`, does not remove any code and only evaluates to `true` or `false`. For example, all blocks in an `if/else` expression need to be valid when `cfg!` is used for the condition, regardless of what `cfg!` is evaluating.
