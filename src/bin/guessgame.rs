use crossterm::cursor::EnableBlinking;
use crossterm::{execute, terminal};
use rand::Rng;
use std::error::Error;
use std::io::{self, Write, stdout};
use std::cmp::Ordering::*;
use crossterm::style::{Stylize, SetBackgroundColor, SetForegroundColor, Color};

fn main() -> Result<(), Box<dyn Error>> {
    let mut secret_number = rand::thread_rng().gen_range(1..=100);

    let mut stdout = stdout();

    execute!(stdout, SetBackgroundColor(Color::Black))?;
    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
    execute!(stdout, EnableBlinking)?;

    loop {
        // execute!(stdout, SetBackgroundColor(Color::Black))?;
        // execute!(stdout, terminal::Clear(terminal::ClearType::All))?;

        println!("LOG: {secret_number}");

        print_info();

        io::stdout().flush()?;

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)?;

        let guess: i32 = match guess.trim().parse() {
            Ok(value) => value,
            Err(_) => continue,
        };

        if guess < 1 || guess > 100 {
            println!("{}", "\nThe number should be between 1 and 100. Please, try again!".white().on_black());
            continue;
        }

        match guess.cmp(&secret_number) {
            Less => println!("\n{}", "Too small.".white().on_black()),
            Greater => println!("\n{}", "Too big.".white().on_black()),
            Equal => {
                print_win_info();
                io::stdout().flush()?;

                let mut buf = String::new();
                io::stdin().read_line(&mut buf)?;

                let ch = buf.chars().next().unwrap();

                if ch == 'y' || ch == 'Y' {
                    secret_number = rand::thread_rng().gen_range(1..=100);
                    continue;
                }

                break;
            }
        }
    }

    Ok(())
}

fn print_win_info() {
    println!("\n{}", "=---- You Win!!! ----=".green().on_black());

    println!("\n{}", "Do you want play again?".white().on_black());
    println!("\n{}", "Press 'y' to continue or any key to quit.".white().on_black());
    print!("\n{}", "> ".white().on_black());
}

fn print_info() {
    println!("\n{}", "=---- Welcome to game \"Guess the Number\" ----=".bold().blue().on_black());
    println!("\n{}", "Enter a number between 1 and 100 and press [Enter] Key.".white().on_black());
    print!("\n{}", "Good lucky! > ".red().on_black());
}
