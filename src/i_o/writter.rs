use std::{fs, process};

pub struct Writter {}

impl Writter {

    fn write_file(&self, data: String) {
        match fs::write("results.txt", data) {
            Ok(()) => (),
            Err(_) => {
                println!("Unable to write resulting file");
                process::exit(1);
            }
        }
    }

    pub fn write_result(&self, data: &Vec<String>) {
        let mut to_write: String = String::new();
        for i in data {
            to_write.push_str(format!("{}\n", i).as_str());
        }


        self.write_file(to_write)
    }

}
