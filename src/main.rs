use rand::Rng;
use std::env;

fn main() {
    let input = get_text();
    let out = catspeak(&input.0, input.1);
    println!("{}", out);
}

fn catspeak(text: &str, cat_index: Option<u8>) -> String {
    let mut cat = text.to_string();
    if cat_index.is_some() {
        let cats = include_str!("../res/cats.txt");
        let start_index = format!("#S{:?}#", cat_index.unwrap());
        let end_index = format!("#E{:?}#", cat_index.unwrap());
        let start = cats.find(&start_index).expect("Error finding start index");
        let end = cats.find(&end_index).expect("Error finding end index");
        cat = cats[start..end].replace("#T#", text);
        cat = cat.replace(&start_index, "");
    }
    cat.to_string()
}

fn get_text() -> (String, Option<u8>) {
    let name = "catspeak";
    let err_msg = format!("Error: See \"{name} --help\"");
    let usage_msg = format!("Usage: {name} [options] \"<message>\"");
    let help_msg = format!("Cowsay like program of a speaking cat\n\n{usage_msg}\n\nOptions:\n  -h, --help\tPrint this message\n  -r, --random\tUse a random cat");

    let mut catid = 0;
    let mut option: Option<String> = None;
    let mut text = usage_msg;

    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            text = args[1].clone();
            if text.starts_with("-") {
                let allow = ["-h", "--help", "-v", "--version"];
                if allow.contains(&text.as_str()) {
                    option = Some(text.clone())
                } else {
                    return (err_msg, None);
                }
            }
        }
        3 => {
            option = Some(args[1].clone());
            text = args[2].clone();
        }
        _ => return (text, None),
    }

    if option.is_some() {
        match option.unwrap().as_str() {
            "-h" | "--help" => return (help_msg, None),
            "-r" | "--random" => {
                let cats = include_str!("../res/cats.txt");
                let cat_count: u8 = cats[cats.find(":").unwrap() + 1..cats.find(";").unwrap()]
                    .parse()
                    .expect("Error getting cat count");
                let mut rng = rand::rng();
                catid = rng.random_range(0..cat_count);
            }
            _ => return (err_msg, None),
        }
    }

    (text, Some(catid))
}
