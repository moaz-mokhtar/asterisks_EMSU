// ===

// ===

fn main() {
    println!("Hello, world!");

    // validate_input(-5);
    // validate_input(100);
    // // validate_input(100_000_000_000);

    loop {
        let mut input = String::new();
        println!("Note: You are allowed to enter range from 0 to 100 Billion (100,000,000,000)");
        // println!("Enter range: ");
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim_end().to_string();
        dbg!(&input);
        let input = input.parse::<i64>().unwrap();
        dbg!(input);
        validate_input(input);
    }
}

fn validate_input(input: i64) {
    match input {
        i if i < 0 => {
            println!("Wrong entry (input less than range). You are allowed to enter range from 0 to 100 Billion (100,000,000,000)");
        }
        i if i > 100_000_000_000 => {
            println!("Wrong entry (input more than range). You are allowed to enter range from 0 to 100 Billion (100,000,000,000)");
        }
        i if i > 0 && i <= 100_000_000_000 => {
            println!("You are in range.");
            print_asterisks(i);
        }
        _ => {
            println!(
                "Wrong entry. You are allowed to enter from 0 to 100 Billion (100,000,000,000)"
            );
        }
    }
}

fn print_asterisks(range: i64) {
    for i in 0..range + 1 {
        let stars_count = i * 2 + 1;
        let stars_string = "*".repeat(stars_count as usize);
        println!("[{i:12}]: {stars_string}");
    }
}
