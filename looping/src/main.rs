use std::io;


fn main() {

    let mut count = 1;

    loop {
        println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");

      
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        if guess.trim() == "The letter e" {
            println!("Number of trials: {}", count);
            break;
        } else {
            count += 1;
            continue;
        }

    }
}