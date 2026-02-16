use std::{
    io::{ Result },
    env,
    path
};

mod app;

use app::{ App };

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if 2 < args.len() {
        panic!("Too many arguments. Try `cargo run $ARG` or `cargo run`\n");
    }

    let file_name = match args.len() {
        1 => String::new(),
        2 => {
            let file_path = path::absolute(&args[1])?;

            file_path.display().to_string()
        },

        _ => panic!("Too many arguments. Try `cargo run $ARG` or `cargo run`\n"),
    };
    
    let mut app = App::new(file_name);
    app.run()?;

    Ok(())
}
