use std::io;


fn main() {
    loop {
        println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");

        let mut count = 0;
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
            
        println!({guess});
        count += 1;

        if guess.trim() == "The letter e" {
            println!("Number of trials: {}", count);
            break;
        } else {
            continue;
        }

    }
}