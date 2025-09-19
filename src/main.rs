use std::io::{self, Write};

fn get_new_number(number: &mut i32) -> Result<(), &'static str> {
    print!("Enter a new number: ");
    io::stdout().flush().unwrap();
    let mut new_number = String::new();
    io::stdin().read_line(&mut new_number).map_err(|_| "Failed to read input")?;
    let parsed: i32 = new_number.trim().parse().map_err(|_| "Invalid number")?;
    if parsed < 0 {
        return Err("Number must be positive");
    }
    *number = parsed;
    Ok(())
}

fn main() {
    println!("The current directory is: {:?}", std::env::current_dir().unwrap());

    let mut number = 42;

    println!("Number is {}", number);
    println!("=== Would you like to change it? ===");
    println!("(y/n)");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    if input.trim() == "y" {
        loop {
            if let Err(e) = get_new_number(&mut number) {
                println!("Error getting new number: {}", e);
            } else {
                println!("Number changed to {}", number);
                break;
            }
        }



    } else {
        println!("Number remains {}", number);
    }

    println!("Number is {}", number);

    println!("=== End of Program ===");
}
