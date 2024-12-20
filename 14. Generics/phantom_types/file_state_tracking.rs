use std::marker::PhantomData;

// Define empty state maker
struct Open;
struct Closed;

// Define a file struct that takes a "state" as type parameter
struct File<State> {
    name: String,
    _state: PhantomData<State>, // Tracks the file's state type
}

impl File<Closed> {
    // Method to open a file and transition it from CLosed to Open
    fn open(self) -> File<Open> {
        println!("Opening file: {}", self.name);
        File {
            name: self.name,
            _state: PhantomData,
        }
    }
}

impl File<Open> {
    // Method to read from an open file
    fn read(&self) {
        println!("Reading from file: {}", self.name);
    }

    // Method to close a file and transition it from Open to Closed
    fn close(self) -> File<Closed> {
        println!("Closing file: {}", self.name);
        File {
            name: self.name,
            _state: PhantomData,
        }
    }
}

fn main() {
    // Create a closed file
    let closed_file: File<Closed> = File {
        name: String::from("example.txt"),
        _state: PhantomData,
    };

    // Open the file (changing state from Closed to Open)
    let open_file = closed_file.open();

    // Read from the open file
    open_file.read();

    // Close the file (changing state from Open to Closed)
    let _closed_file = open_file.close();
}
