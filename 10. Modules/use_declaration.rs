// The use declaration can be used to bind a full path to a new name, for easier access

use crate::deeply::nested::function as other_function;

fn function() {
    println!("Called `function()`");
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::function()` ");
        }
    }
}

fn main() {
    // Easier access to `deeply::nested::function`
    other_function();

    println!("Entering block");
    {
        // This is equivalent to `use deeply::nested::function as function`.
        // This `function()` will shadow the outer one.
        use crate::deeply::nested::function;

        // `use` bindings have a local scope.
        // In this case, the shadowing of `function()` is only in this block.
        function();

        println!("Leaving block");
    }
    function();
}