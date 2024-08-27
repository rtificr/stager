use crate::act::Act;
use crate::element::Element;
use crate::runtime_util::*;
use std::io::stdin;

pub fn run(args: Vec<String>) -> Result<(), String> {
    range(args.clone(), 1, 1)?;

    let act = get_act(args[0].clone())?;

    let title = &act.title;
    let author = &act.author;
    let desc = &act.desc;

    println!();
    println!("'{title}' by {author}");
    println!("Description:\n{desc}");

    step(&act, "start").map_err(|e| e)?;

    Ok(())
}

    fn step(act: &Act, key: &str) -> Result<(), String> {
        let el: &Element = get_element(act, key)?;

        println!();
        println!("{}", el.content);

        if el.choices.is_empty() {
            if key != "end"{
                return step(act, "end").or(Ok(()));
            } else {
                return Ok(());
            }
        }

        for (i, choice) in el.choices.iter().enumerate() {
            println!("  {}: {}", i + 1, choice.text);
        }

        let mut choice_i: usize;

        println!();
        println!("Pick a choice:");

        loop {
            let num = get_input() -1;

            if num > 0 || num < el.choices.len() {
                choice_i = num;
                break;
            } else {
                println!("Choice out of range! Trying again...")
            }
        }

        step(act, el.choices[choice_i].dest.as_str())
    }

fn get_element<'a>(act: &'a Act, key: &str) -> Result<&'a Element, String> {
    match act.elements.get(key) {
        Some(ref e) => Ok(e),
        None => Err(format!("Element '{key}' not found!"))
    }
}

fn get_input() -> usize {
    loop {
        let mut strinput = String::new();
        if let Err(e) = stdin().read_line(&mut strinput) {
            eprintln!("Unable to read input:\n\t{e}\nTrying again...");
            continue;
        }

        match strinput.trim().parse::<usize>(){
            Ok(n) => return n,
            Err(_) => eprintln!("Input must be a number. Trying again...")
        }
    }
}

