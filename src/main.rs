use std::env;

fn main() {
    let input = get_text();
    let out = catspeak(&input.0, input.1);
    println!("{}", out);
}

fn catspeak(text: &str, cat_index: Option<u8>) -> String {
    let mut cat = text.to_string();
    if cat_index.is_some() {
        cat = include_str!("../res/cat.txt").replace("#Text#", text);
    }
    cat
}

fn get_text() -> (String, Option<u8>) {
    let err_msg = "Error: See \"catspeak --help\"".to_string();
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        return (err_msg, None);
    }
    let mut option = String::new();
    let mut text = &args[1];
    if args[1].starts_with("-") {
        if args.len() <= 2 {
            if &args[1] != "--help" && &args[1] != "-h"{
                return (err_msg, None);
            }
        } else {
            text = &args[2];
        }
        option = args[1].clone();
    }
    if &args[1] == "--help" || &args[1] == "-h" {
        return ("Usage: catspeak <option> \"<message>\"".to_string(), None);
    }
    if option == "--random" || option == "-r" {
        return (text.to_string(), Some(0));
    }
    (text.to_string(), Some(0))
}
