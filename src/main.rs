use std::io::Write;

fn main() {
    while let 1 = 1 {
        print!("$ ");
        std::io::stdout().flush()
            .expect("Unable to flush stdout");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)
            .expect("Failed to take input");
    }
}
