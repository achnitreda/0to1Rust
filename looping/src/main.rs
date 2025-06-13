use std::io;

fn main() {
    let mut count : u8 = 1;
    loop  {
        let mut input : String = String::new();

        println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");

        io::stdin().read_line(&mut input).expect("Error reading Input");

        if input.trim() == "The letter e" {
            println!("Number of trials: {}", count);
            break
        }
        count += 1
    }
}
