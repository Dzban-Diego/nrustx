use cli_select::Select;
use std::fs;
use std::io::stdout;
use std::process::Command;

const FILE_NAME: &str = "package.json";
const NPM_MANAGER: &str = "pnpx";

fn main() {
    let contents = fs::read_to_string(FILE_NAME).expect("Should have been able to read the file");
    let parsed: serde_json::Value = serde_json::from_str(&contents).unwrap();

    let mut items: Vec<String> = Vec::new();

    for key in parsed["scripts"].as_object().unwrap() {
        let test = format!("{} => {}", key.0, key.1).to_string();
        items.push(test)
    }

    let mut select = Select::new(&items, stdout());
    let selected_item = select.start();

    let answer_vec: Vec<&str> = selected_item.split(">").collect();
    let script = answer_vec[1].trim().replace('"', "");

    let output = format!("{} {}", NPM_MANAGER, script);
    println!("\n");
    Command::new("sh")
        .arg("-c")
        .arg(output)
        .spawn()
        .expect("lol");
}
