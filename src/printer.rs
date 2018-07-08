use std::io::{self, Write};
use std::process::exit;
use std::thread;
use std::time::Duration;

pub fn confirmation() {
    print!("If you are unlucky, this will delete random file(s) in above path. Are you sure you want to play? <Y,y,YES,yes>: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");

    match input.trim().as_ref() {
        "Y" | "YES" | "y" | "yes" => {
            return;
        }
        _ => {
            println!("Probably a wise choice...");
            exit(0);
        }
    };
}

pub fn print_prob(path: &str, chambers: usize) {
    println!(
        "Path = {}, Chances = 1/{} ({:.2}%)",
        path,
        chambers,
        (1.0 / chambers as f32) * 100.0,
    );
}
pub fn count_down(from: usize, file: &str) {
    for i in (1..=from).rev() {
        print!("\r");
        io::stdout().flush().unwrap();
        print!(
            "{} {} {}{} \x1b[0;31m{}\x1b[0m ",
            "_".repeat(file.len() + 4),
            '\u{1F52B}',
            "-".repeat(i - 1),
            '\u{1F525}',
            i
        );
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
    }
    print!("\r");
    io::stdout().flush().unwrap();
    print!("{}\n", " ".repeat(100));
}

pub fn rest_in_peace(file: &str) {
    println!(
        "{}{}\x1b[0;31m{}\x1b[0m{}{} *BANG*\n",
        '\u{271D}', '\u{1F480}', file, '\u{1F480}', '\u{271D}'
    );
}

pub fn alive(file: &str) {
    println!(
        "{}{}\x1b[033;92m{}\x1b[0m{}{} *CLICK*\n",
        '\u{1F340}', '\u{1F340}', file, '\u{1F340}', '\u{1F340}'
    );
}
