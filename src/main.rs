use std::fs::OpenOptions;
use std::fs;
use std::io;
use std::io::Write;

fn main() {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("fil.txt")
        .expect("Kunne ikke åbne fil");

    let mut choice_input = String::new();
    let mut input = String::new();
    
    loop {
        choice_input.clear();

        println!();
        println!("1. Læs tekstfilen.");
        println!("2. Tilføj 5 linjer til tekstfilen.");
        println!("3. Slet alt fra tekstfilen.");
        println!("4. Afslut programmet.");

        io::stdin()
            .read_line(&mut choice_input)
            .expect("Fejl ved input");

        // konverter input til tal
        let number: i32 = choice_input.trim().parse().unwrap_or(0);

        match number {
            1 => {
                let content = fs::read_to_string("fil.txt")
                    .expect("Kunne ikke læse filen");

                println!("\n--- Indhold starter ---");
                println!("{}", content);
                println!("--- Indhold slutter ---");
            }

            2 => for _ in 0..5 {
                println!("Tilføj noget til tekstfilen. ");
                input.clear();

                io::stdin()
                    .read_line(&mut input)
                    .expect("Fejl ved læsning af input");

                write!(file, "{}", input).expect("Kunne ikke skrive til filen");
            }

            3 => {
                fs::write("fil.txt", "").expect("Kunne ikke rydde filen");
                println!("Filen er blevet ryddet.");
            }

            4 => {
                println!("Afslutter programmet.");
                break;
            }

            _ => {}
        }
    }
}
