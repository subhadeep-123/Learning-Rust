// For some example using `match` is awkward, so we use `if let`

fn with_match() {}

fn main() {
    // See if match is any better
    let optional = Some(7);

    match optional {
        Some(i) => println!("This is a really long string and `{:?}`", i),
        _ => {} // < Required because `match` is exhaustive. Doesn't it seem
                // like wasted space?
    }

    // Using If-let
    // All have type Option
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // The `if let` construct reads: "if `let` destructures `number` into `Some(i)`, evaluate the block (`{}`)"
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    // If we need to specify a failure we can use else
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        println!("Didn't match a number. Let's fo with a letter");
    }

    // Provide an altered failing condition
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        println!("I don't like letters. Let's go with an emoticon :)!");
    }

    // If let with enums
    enum Foo {
        Bar,
        Baz,
        Qux(u32),
    }
    // create example  variables
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(00);

    // variable b does not match Foo::Bar
    // so this will print nothing
    if let Foo::Bar = a {
        println!("a is foobar");
    }

    // Variable b does not match Foo::Bar
    // so this will print nothing
    if let Foo::Bar = b {
        println!("b is foobar");
    }

    // Variable c matches Foo::Qux which has a value
    // Similar to Some() in the previous example
    if let Foo::Qux(value) = c {
        println!("C is {}", value);
    }

    // Binding also works with `if let`
    if let Foo::Qux(value @ 100) = c {
        println!("c is one hundred");
    }

    // Change this into a comment
    // Another benefit is that if let allows us to match non-parameterized enum variants.
    // This is true even in cases where the enum doesn't implement or derive PartialEq.
    // In such cases if Foo::Bar == a would fail to compile, because instances of the enum cannot be equated,
    // however if let will continue to work.
    enum Fo0o {
        Bar,
    }
    let a = Fo0o::Bar;

    // Variable a matches Foo::Bar
    if let Fo0o::Bar = a {
        println!("a if foobar");
    }
}
