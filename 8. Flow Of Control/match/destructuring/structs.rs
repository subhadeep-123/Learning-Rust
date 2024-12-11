fn main() {
    #[derive(Debug)]
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    // Changing the values in the struct to see what happens
    let foo = Foo {
        x: (1 + 1, 2),
        y: 2,
    };

    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {}, y = {}", b, y),

        // We can destructure struct and rename the variables
        // Order is not important
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),

        // We can also ignore some variables
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
    }

    let faa = foo;
    println!("Faa: {:?}", faa);

    // Destructuring structs without match block
    let Foo { x: x0, y: y0 } = faa;
    println!("Outsize: x0 = {x0:?}, y0 = {y0}");

    // Destructuring works with nested structs as well
    #[derive(Debug)]
    struct Bar {
        foo: Foo,
    }

    let bar = Bar { foo: faa };
    let Bar {
        foo: Foo {
            x: nested_x,
            y: nested_y,
        },
    } = bar;

    println!("Nested: nested_x = {nested_x:?}, nested_y = {nested_y:?}");
}
