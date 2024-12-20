// Similar to functions, implementations require care to remain generic
struct S; // Concrete type
struct GenericVal<T>(T); // Generic type `GenericVal`

// impl GenericVal where we explicitly specify type parameters
impl GenericVal<f32> {} // specify `f32`
impl GenericVal<S> {} // specify `S` as defined above

// `<T>` Must precede the type to remain generic
impl<T> GenericVal<T> {}

struct Val {
    val: f64,
}

struct GenVal<T> {
    gen_val: T,
}

// impl of Val
impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}

// impl of GenVal for a generic type `T`
impl<T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}

fn main() {
    let x = Val { val: 3.0 };
    let y = GenVal { gen_val: 3i32 };

    println!("{}, {}", x.value(), y.value())
}
