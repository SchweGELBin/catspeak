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
    let err_msg = "Error: See \"catspeak --help\"".to_string();
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        return (err_msg, None);
    }
    let mut option = String::new();
    let mut text = &args[1];
    if args[1].starts_with("-") {
        if args.len() <= 2 {
            if &args[1] != "--help" && &args[1] != "-h" {
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
        let cats = include_str!("../res/cats.txt");
        let start = cats.find(":").unwrap();
        let end = cats.find(";").unwrap();
        let cat_count: u8 = cats[start+1..end].parse().expect("Error getting cat count");
        let mut rng = rand::thread_rng();
        return (text.to_string(), Some(rng.gen_range(0..cat_count)));
    }
    (text.to_string(), Some(0))
}
