use std::fs::OpenOptions;
use std::io::Read;

#[derive(Debug, Default)] // Derive is cool, I have no idea how it works!
pub struct FileLoader {
    path: String,
    content: String,
}

impl FileLoader {
    pub fn new() -> FileLoader {
        FileLoader::default()
    }

    pub fn set_path(&mut self, s: &str) {
        self.path = s.to_string();
        self.read_contents();
    }

    pub fn _get_path(&mut self) -> &mut String {
        &mut self.path
    }

    pub fn get_content(&self) -> &String {
        &self.content
    }

    fn read_contents(&mut self) {
        let handle = OpenOptions::new().read(true).open(&self.path);
        let mut s = String::new();
        if let Ok(mut i) = handle {
            let result = i.read_to_string(&mut s);
            if let Err(e) = result {
                eprintln!("Warning! could not read file with: {}", e);
            }
            self.content = s.to_string();
        } else {
            eprintln!("Warning! failed to open file: {}", &self.path);
        };
    }
}
