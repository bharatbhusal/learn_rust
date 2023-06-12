pub mod errors {
    use std::fs::File;
    use std::io::{self, ErrorKind, Read};
    pub fn primitive_error_handling() {
        let mut file = match File::open("hello.txt") {
            Ok(file) => file,
            Err(err) => match err.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(f) => f,
                    Err(e) => panic!("Error creating file."),
                },
                _ => panic!("Error opening file."),
            },
        };
        let mut content = String::new();
        match file.read_to_string(&mut content) {
            Ok(content) => println!("content: {}", content),
            Err(err) => panic!("error while reading"),
        }
    }

    pub fn read_username_from_file() -> Result<String, io::Error> {
        let username_file_result = File::open("hello.txt");

        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut username = String::new();

        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
        // username_file.read_to_string(&mut username)?;
        // Ok(username)
    }

    pub fn error_propagation() -> Result<String, io::Error> {
        let mut file = match File::open("hello.txt") {
            Ok(file) => file,
            Err(err) => return Err(err),
        };
        let mut content = String::new();
        match file.read_to_string(&mut content) {
            Ok(_) => Ok(content),
            Err(err) => Err(err),
        }
    }

    pub fn error_propagation_shortcut() -> Result<String, io::Error> {
        let mut username_file = File::open("hello.txt")?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }
}
