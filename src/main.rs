use std::env;

fn main() {
    let input = get_text();
    let out = catspeak(&input);
    println!("{}", out);
}

fn catspeak(text: &str) -> String {
    let cat = include_str!("../res/cat.txt");
    let cat = cat.replace("#Text#", text);
    cat.to_string()
}

fn get_text() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        return "Error: Read the documentation with \"catspeak --help\"".to_string();
    }
    let text = &args[1];
    if text == "--help" {
        return "Usage: catspeak \"<message>\"".to_string();
    }
    text.to_string()
}
