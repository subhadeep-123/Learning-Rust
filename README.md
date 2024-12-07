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