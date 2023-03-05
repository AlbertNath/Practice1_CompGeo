use std::fs;

pub struct Writter;

impl Writter {

    fn write_file(&self, data: &mut str) {
        fs::write("./results.txt", data).expect("Unable to write file.")
    }

}
