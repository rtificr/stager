use crate::cli::start_cli;

mod process;
mod cli;
mod types;
mod err;

fn main() {
    let version = "3.0.0";

    println!(r"   _____ _______       _____      _____  ");
    println!(r"  / ____|__   __|/\   / ____|    |  __ \ ");
    println!(r" | (___    | |  /  \ | |  __  ___| |__) |");
    println!(r"  \___ \   | | / /\ \| | |_ |/ _ \  _  / ");
    println!(r"  ____) |  | |/ ____ \ |__| |  __/ | \ \ ");
    println!(r" |_____/   |_/_/    \_\_____|\___|_|  \_\");
    println!();
    println!("STAGeR v{version}");
    println!("Static Text-based Adventure Game Runtime");

    start_cli();
}
