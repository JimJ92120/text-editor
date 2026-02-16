use std::{
    io::{ Error, ErrorKind, Result },
    fs::{ self, metadata },
    path::{ Path },
    os::{
        unix::{
            fs::{ PermissionsExt }
        }
    }
};

#[derive(Debug)]
pub struct Controller {}

impl Controller {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_file_content(&self, path_name: String) -> Result<String> {
        let path = Path::new(&path_name);

        if !path.is_file() {
            return Err(Error::new(
                ErrorKind::Other,
                format!("`{}` is not a file.", path_name)
            ));
        }

        let permissions_mode = metadata(path).unwrap().permissions().mode();
        if !self.can_read(permissions_mode) {
            return Err(Error::new(
                ErrorKind::PermissionDenied,
                format!("Not allowed to read `{}`.\n", path_name)
            ));
        }
        if !self.can_write(permissions_mode) {
            return Err(Error::new(
                ErrorKind::PermissionDenied,
                format!("Not allowed to write `{}`.\n", path_name)
            ));
        }

        match fs::read_to_string(path) {
            Ok(content) => Ok(content),
            Err(error) => Err(Error::new(
                ErrorKind::Other,
                format!("Unable to read `{}` content.\n{}\n", path_name, error)
            )),
        }
    }

    fn can_read(&self, permissions_mode: u32) -> bool {
        permissions_mode & 0o400 != 0
    }

    fn can_write(&self, permissions_mode: u32) -> bool {
        permissions_mode & 0o200 != 0
    }
}
