use std::{
    io::{ Result, Error, ErrorKind },
    env,
    path
};

mod app;

use app::{ App };

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if 2 < args.len() {
        return Err(Error::new(
            ErrorKind::Other,
            String::from("Too many arguments. Try `cargo run $ARG` or `cargo run`\n")
        ));
    } else if 2 > args.len() {
        return Err(Error::new(
            ErrorKind::Other,
            String::from("Missing $ARG. Try `cargo run $ARG` or `cargo run`\n")
        ));
    }

    let file_name = path::absolute(&args[1])?
        .display()
        .to_string();
    
    let mut app = App::new(file_name);
    app.run()?;

    Ok(())
}
