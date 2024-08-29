use std::{fs::File, io::ErrorKind};

fn main() {
    let my_file = match File::open("hello.txt") {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(new_file) => new_file,
                Err(e) => panic!("Problim creating the file: {e:?}"),
            },
            other => {
                panic!("Problem Opening the file : {other:?}");
            }
        },
    };
}
