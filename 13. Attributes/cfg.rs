// This function only gets compiled if the target OS is linux
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!")
}

// And this function only gets compiled if the target OS is *not* linux
#[cfg(not(target_os = "linux"))]
fn are_you_not_on_linux() {
    println!("You are *not* running linux!");
}

// Conditionals like target_os are implicitly provided by rustc, but custom conditionals must be passed to rustc using the `--cfg` flag
// rustc cfg.rs -o main.exe --cfg some_condition && main.exe
// Otherwise not found in scope error will be shown
#[cfg(some_condition)]
fn conditional_function() {
    println!("condition met!");
}

fn main() {
    // This will not run, It will give (not found in this scope)
    // are_you_on_linux();

    are_you_not_on_linux();

    println!("Are you sure?");
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }

    conditional_function();
}
