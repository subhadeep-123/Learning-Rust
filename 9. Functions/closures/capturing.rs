fn main() {
    use std::mem;

    let color = String::from("green");

    // A closures to print `color` which immediately borrows (`&`) `color` and stores the
    // borrow and closure in the `print` variable. It will remain borrowed until `print` is used the last time
    let print = || println!("Color: {}", color);

    // Call the closure using the borrow
    print();

    // `color` can be borrowed immutably again, because the closure only holds an immutable reference to `color`
    let _reborrow = &color;

    print();

    let mut count = 0;
    // A closure to increment `count` could take either `&mut count` or `count`
    // but `&mut count` is less restrictive so it takes that. Immediately
    // borrows `count`.

    // A `mut` is required on `inc` because a `&mut` is stored inside. Thus, calling the closure
    // mutates `count` which requires a `mut`.
    let mut inc = || {
        count += 1;
        println!("count: {}", count);
    };

    inc();

    // cannot borrow `count` as immutable because it is also borrowed as mutable
    // let _reborrow = &count;

    inc();

    // The closure no longer needs to borrow `&mut count`.
    // Therefore, it is possible to reborrow without an error
    let _count_reborrowed = &mut count;

    // A non-copy type
    let movable = Box::new(3);

    // `mem::drop` requires `T` so this must take by value. A copy type
    // would copy into the closure leaving the original untouched.
    // A non-copy must move and so `movable` immediately moves into
    // the closure.
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    consume();
    // closure cannot be invoked more than once because it moves the variable `movable` out of its environment
    // consume();

    // Using `move` before vertical pipes forces closures to take ownership of captured variables
    let haystack = vec![1, 2, 3];
    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));

    // println!("There're {} elements in vec", haystack.len());
    // ^ Uncommenting above line will result in compile-time error
    // because borrow checker doesn't allow re-using variable after it
    // has been moved.
}
