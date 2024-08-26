use crate::error::err_code;
use crate::run::run;
use crate::show::show;

pub fn start_cli() {
    query();
}

//stores command
struct CommandBuf {
    com: String,
    args: Vec<String>,
}


impl CommandBuf {
    fn new(com: String, args: Vec<String>) -> CommandBuf {
        CommandBuf {
            com,
            args,
        }
    }
}

fn query() -> Result<(), String> {
    let commands;

    let mut r: Result<(), String> = Ok(());

    match get_input() {
        Ok(c) => commands = c,
        Err(e) => return Err(e)
    };

    for command in &commands {
        match command.com.as_str() {
            "help" | "h" => {
                help()
            }
            "run" => r = run(command.args.clone()),
            "show" => r = show(command.args.clone()),

            "quit" | "exit" => {
                return Ok(());
            }

            _ => println!("Invalid command. Type 'h' for help")
        }
    }

    match r {
        Ok(()) => println!("\nCommand successful!\n"),
        Err(e) => eprintln!("\nERROR: {e}\n")
    }

    query()
}

fn get_input() -> Result<Vec<CommandBuf>, String> {
    let mut input = String::new();

    std::io::stdin().read_line(&mut input)
        .map_err(|e| err_code("000", e))?;

    let mut commands: Vec<CommandBuf> = Vec::new();

    let chunks = input.split([',',';'])
        .map(|s| s.trim())
        .filter(|s| !s.is_empty());

    for chunk in chunks {
        let split_in: Vec<&str> = chunk.split_whitespace()
            .filter(|s| !s.is_empty())
            .collect();

        if split_in.is_empty() { continue; };

        let mut command = String::new();
        let mut args: Vec<String> = Vec::new();

        for s in split_in {
            if command.is_empty() {
                command = s.to_string()
            } else {
                args.push(s.to_string())
            }
        }

        commands.push(CommandBuf::new(command, args));
    }

    if commands.is_empty() {
        return Err("No commands found".into());
    }

    Ok(commands)
}

fn help(){
    println!("< Commands >");

    println!("TO BUG REPORT: visit 'https://github.com/rtificr/stager-rust'");
    println!();
    println!("run <filepath> abs?: runs act located at filepath
        (relative to the 'input' folder, but has an option to use an absolute path)
        USAGE: 'run example.act' or 'run C:\\Documents\\example.act'\
    ");
    println!("show <filepath> abs?: displays act located at filepath (as Stager sees it)
        Useful for debugging.
        (relative to the 'input' folder, but has an option to use an absolute path)
        USAGE: 'show example.act' or 'show C:\\Documents\\example.act'\
    ");
}
