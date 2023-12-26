use clap::Parser;
use console::Term;
use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Parser)]
pub struct Cli {
    op: String,
    items: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct AppState {
    item_list: Vec<String>,
}

fn load() -> Vec<String> {
    if let Ok(file_content) = fs::read_to_string("state.json") {
        if let Ok(state) = serde_json::from_str::<AppState>(&file_content) {
            return state.item_list;
        }
    }
    Vec::new()
}

fn save(item_list: &[String]) {
    let state = AppState {
        item_list: item_list.to_vec(),
    };

    if let Ok(serialized_state) = serde_json::to_string(&state) {
        if let Err(err) = fs::write("state.json", serialized_state) {
            eprintln!("Error writing to file: {}", err);
        }
    }
}

fn main() {
    let term = Term::stdout();
    let args = Cli::parse();
    let mut item_list = load();

    match args.op.as_str() {
        "add" => {
            for item in args.items.into_iter() {
                item_list.push(item);
            }

            for item in item_list.iter() {
                if let Err(err) = term.write_line(item) {
                    eprintln!("Output Error: {}", err);
                }
            }
        }

        "delete" => {
            for item_to_remove in args.items.into_iter() {
                item_list.retain(|item| *item != item_to_remove);
            }

            for item in item_list.iter() {
                if let Err(err) = term.write_line(item) {
                    eprintln!("Output Error: {}", err);
                }
            }
        }

        "list" => {
            for item in item_list.iter() {
                if let Err(err) = term.write_line(item) {
                    eprintln!("Output Error: {}", err);
                }
            }
        }

        _ => {
            term.write_line(&format!("Error: Unrecognized operation '{}'", args.op)).unwrap();
            std::process::exit(1);
        }
    }

    save(&item_list);
}

