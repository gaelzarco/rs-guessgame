// cargo doc --open to view documentation on project crates

// Prelude
use std::io;
use rand::Rng;
use std::cmp::Ordering;

//Code (Binary Crate: executable; Library Crate: Intended to be used in other programs)
fn main () {
    println!("Guess the number!");

    // Local to the current thread of execution; Seeded by the operating system
    // start..=end (inclusive on upper and lower bounds)
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("the secret number is: {}", &secret_number);

    loop {
        println!("Please input your guess.");

        // Bind variable to new instance of the type 'String'; String::new()
        // :: indicates that new() is a funtions associated with the String type.
        let mut guess = String::new();
        io::stdin()
        // .read_line(&mut guess) tells what string to store the user input in.
        // The & indicates a reference.
        // Allows code to access data w/o needing to copy that data into memory multiple times.
        // References are immutable by default.

            // References are enums and returns multiple states which are called variants
            // The Result enum has two states: 'Ok' and 'Err'
            // It also has multiple methods like '.expect()'
            .read_line(&mut guess)
            .expect("Failed to read line");

        // This variable shadows over the first guess variable; enables you to use the same name rather than writing a new one.
        // Match expression to handle non-number inputs
        // _ is a catchall value; we are matching all err values no matter what info is inside of them.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        // String interpolation
        println!("You guessed: {guess}");

        // Ordering is an enum with 3 variants
        // Match consists of arms that contain a pattern to match against.
        // When a match is made: the code in the arm is run.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering:: Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}