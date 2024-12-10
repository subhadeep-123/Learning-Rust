use std::io;

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let mut target = String::new();

    println!("Enter target number 1>= x =<10: ");

    io::stdin()
        .read_line(&mut target)
        .expect("Did not enter a correct string");

    let target: i32 = target.trim().parse().unwrap();

    println!("Entered number: {target}");

    let res = 'outer: loop {
        for &num in &numbers {
            if num == target {
                break 'outer format!("Found number: {num}");
            }
        }
        break String::from("No needs to be in range 1..10");
    };

    println!("{}", res);
}
