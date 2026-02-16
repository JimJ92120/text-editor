use std::{
    io::{ stdout, Result },
    env,
    path::{ self }
};
use crossterm::{
    terminal
};

mod editor;

use editor::{ Editor };

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if 2 < args.len() {
        panic!("Too many arguments. Try `cargo run $ARG` or `cargo run`\n");
    }

    let editor = match args.len() {
        1 => {
            println!("new file");
            
            Editor::new(None)
        },
        2 => {
            println!("edit {}", &args[1]);

            let file_path = path::absolute(&args[1])?;

            Editor::new(Some(file_path.display().to_string()))
        },

        _ => panic!("Too many arguments. Try `cargo run $ARG` or `cargo run`\n"),
    };

    println!("editor: {:?}", editor);

    Ok(())
}
