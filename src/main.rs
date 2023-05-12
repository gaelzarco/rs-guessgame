// the io input/output library comes from the standard library, known as std
// By default, Rust has a set of items defined in the standard library that it brings into the scope of every program. This set is called the prelude
// You can see everything in it here: https://doc.rust-lang.org/std/prelude/index.html
use std::io;
// The Ordering type is another enum and has the variants Less, Greater, and Equal
use std::cmp::Ordering;
// The Rng trait defines methods that random number generators implements
use rand::Rng;

// cargo check (checks if code compiles but does not create an executable)
// cargo run (compiles the code and runs the .\target\debug\guessing_game.exe executable)
// cargo update (updates creates (dependencies) listed in cargo.toml and cargo.lock)
// cargo build (builds debug build of project)
// cargo build --release (builds end-user build of project)

fn main() {
    println!("Guess the number");

    // instantiating an immutable variable called secret_number and binding it to a random integer between 1 - 100 (inclusive)
    // We call rand::thread_rng() that gives us the particular random number generator we’re going to use: 
    // one that is local to the current thread of execution and is seeded by the operating system

    // Then, the gen_range method takes a range expression as an argument and generates a random number in the range.
    // gen_range is degined by the Rng trait that we brought into scope at the top of the document
    // type inference is used here and secret_number is given an i32 number type which is what Rust defaults to
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess");

        // We are instanstiating a new mutable variable called guess and binding it to a value of a new instance of String.
        // The :: syntax in the ::new line indicates that new is an associated function of the String type
        let mut guess = String::new();
        
        // Here we are using the standard rust prelude crate to receive an input from the user in the terminal
        // The read_line method receives a user input and returns a Result type.
        // Result is an enumeration type that consists of the variants Ok or Err.
        // .expect("") is a method that exists within the Result type. If this instance of Result is an Err value,
        // expect will cause the program to crash and display the message that you passed as an argument to expect.
        // If this instance of Result is an Ok value, expect will take the return value that Ok is holding and return just that value 
        // to you so you can use it. In this case, that value is the number of bytes in the user’s input.
        io::stdin()
            //  the line .read_line(&mut guess) calls the read_line method on the standard input handle (std::io) to get input from the user. 
            // We’re also passing &mut guess as the argument to read_line to tell it what string to store the user input in. 
            // The full job of read_line is to take whatever the user types into standard input and append that into a string (without overwriting its contents), 
            //so we therefore pass that string as an argument. The string argument needs to be mutable so the method can change the string’s content.

            //The & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data 
            // without needing to copy that data into memory multiple times. 
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        // This shadows over the original, mutable, guess variable and instantiates it as an immutable u32 integer
        // Its given a value of the original guess string trimmed and parsed into a u32 integer.
        // Match statement called on the Result enum type of the .parse() method to continue the loop if guess cannot be parsed into a number
        // or return the number if guess can be parsed into a number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("Your guess: {guess}");
    
        // Match statment that compares the user's guess to a reference of the secret_number
        // References are used in order to prevent the re-storing of a variable in memory
        // If the value returns a variant of the Ordering enum that fits the arm's pattern (an arm consists of a pattern to match against),
        // its respective code is executed
        // if the comparison of the user's guess and secret number returns the Equal variant, we let the user know he/she won
        // and we break the loop to end the game.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
