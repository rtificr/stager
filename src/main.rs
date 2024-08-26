mod act;
mod element;
mod choice;
mod element_holder;
mod cli;
mod run;
mod error;
mod runtime_util;
mod show;

use crate::cli::start_cli;

fn main() {
    let version = "2.0.0";

    //this says "STAGeR" if you squint hard enough
    println!("   _____ _______       _____      _____  ");
    println!("  / ____|__   __|/\\   / ____|    |  __ \\ ");
    println!(" | (___    | |  /  \\ | |  __  ___| |__) |");
    println!("  \\___ \\   | | / /\\ \\| | |_ |/ _ \\  _  / ");
    println!("  ____) |  | |/ ____ \\ |__| |  __/ | \\ \\ ");
    println!(" |_____/   |_/_/    \\_\\_____|\\___|_|  \\_\\");
    println!();
    println!("STAGeR v{version}");
    println!("Static Text-based Adventure Game Runtime");

    start_cli();
}
