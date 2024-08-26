use crate::runtime_util::*;

pub fn show(args: Vec<String>) -> Result<(), String> {
    range(args.clone(), 1, 1)?;

    let act = get_act(args[0].clone())?;

    println!();
    println!("Title: {}", act.title);
    println!("Author: {}", act.author);
    println!("Description: {}", act.desc);

    for (name, element) in act.elements {
        println!();
        let content = element.content;
        let choices = element.choices;
        println!("{name}〈{content}〉");

        if choices.len() == 0 {
            println!("  ╰─> END")
        }
        for (i, choice) in choices.iter().enumerate() {
            let text = &choice.text;
            let dest = &choice.dest;

            let pre = if i == choices.len()-1 {"╰─>"} else {"├─>"};

            println!("  {pre} {text} => {dest}");
        }
    }
    println!();
    Ok(())
}
