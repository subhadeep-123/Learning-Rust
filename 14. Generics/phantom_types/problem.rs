// A phantom type parameter is one that does'nt show up at runtime, but checked statically (and only) as compile time
// Data types can use extra generic type parameters to act as markers or to perform type checking at compile time.
// These extra parameters hold no storage values, and have no runtime behavior.
use std::marker::PhantomData;

// A phantom tuple struct which is generic over `A` with hidden parameter `B`.
#[derive(PartialEq)] // Allow equality test for this type.
struct PhantomTuple<A, B> {
    first: A,
    phantom: PhantomData<B>,
}

// A phantom type struct which is generic over `A` with hidden parameter `B`.
#[derive(PartialEq)] // Allow equality test for this type
struct PhantomStruct<A, B> {
    first: A,
    phantom: PhantomData<B>,
}

// Note: Storage is allocated for generic type 'A` bus not for `B`.
//       Therefore, `B` cannot be used in computations
fn main() {
    // Here, `f32` and `f64` are the hidden parameters.
    // PhantomTuple type specified as `<char, f32>`
    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    // PhantomTuple type specified as `<char, f64>`.
    let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

    // Type specified as `<char, f32>`
    let _struct1: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q', phantom: PhantomData
    }
    // Type specified as `<char, f64>`
    let _struct2: PhantomStruct<char, f64> = PhantomStruct {
        first: 'Q', phantom: PhantomData
    }
}
