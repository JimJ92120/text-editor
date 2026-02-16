use std::{
    io::{ Error, ErrorKind, Result },
    any::{ Any },
    fs::{ self, metadata },
    path::{ Path },
    os::{
        unix::{
            fs::{ PermissionsExt }
        }
    }
};

#[derive(Debug)]
pub struct Editor {
    current_path_name: Option<String>,
    content: String,
}

impl Editor {
    pub fn new(current_path_name: Option<String>) -> Self {
        match current_path_name {
            Some(current_path_name) => {
                Self {
                    current_path_name: Some(current_path_name.clone()),
                    content: Self::get_file_content(&current_path_name).unwrap(),
                }
            },

            None => Self {
                current_path_name,
                content: String::new()
            },
        }
    }

    pub fn get<T: Clone + 'static>(&self, field: &str) -> T {
        let result = match field {
            "content" => Box::new(self.content.clone()) as Box<dyn Any>,
            "current_path_name" => Box::new(self.current_path_name.clone()) as Box<dyn Any>,

            _ => panic!("`{}` field doesn't exist.", field),
        } as Box<dyn Any>;

        result
            .downcast_ref::<T>()
            .unwrap()
            .clone()
    }

    fn get_file_content(path_name: &str) -> Result<String> {
        let path = Path::new(path_name);

        if !path.is_file() {
            return Err(Error::new(
                ErrorKind::Other,
                format!("`{}` is not a file.", path_name)
            ));
        }

        let permissions_mode = metadata(path).unwrap().permissions().mode();
        if !Self::can_read(permissions_mode) {
            return Err(Error::new(
                ErrorKind::PermissionDenied,
                format!("Not allowed to read `{}`.\n", path_name)
            ));
        }
        if !Self::can_write(permissions_mode) {
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

    fn can_read(permissions_mode: u32) -> bool {
        permissions_mode & 0o400 != 0
    }

    fn can_write(permissions_mode: u32) -> bool {
        permissions_mode & 0o200 != 0
    }
}
