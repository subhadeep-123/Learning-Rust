#[derive(Debug)]
struct Details {
    name: String,
    age: i32,
    profession: Option<String>,
}

impl Details {
    fn new(age: i32) -> Self {
        Details {
            name: String::from("Subhadeep Banerjee"),
            age,
            profession: if age > 18 {
                Some(String::from("Software Engineer"))
            } else {
                None
            },
        }
    }
}

fn main() {
    let detail = Details::new(19);
    println!("Detail: {:#?}", detail);

    let Details { ref profession, .. } = detail;

    if let Some(profession) = profession {
        println!("Congratulation on your job: {:?}", profession)
    } else {
        println!("You have to wait!");
        return;
    };
}
