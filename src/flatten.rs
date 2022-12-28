use std::{
    error::Error,
    fs::{self, DirEntry, ReadDir},
    path::Path,
};

pub struct FlattenExecutor {
    source: String,
    pub copy: bool,
    pub keep_dirs: bool,
}

impl FlattenExecutor {
    pub fn new(directory: String) -> FlattenExecutor {
        FlattenExecutor {
            source: directory,
            copy: false,
            keep_dirs: false,
        }
    }

    pub fn flatten(&self) -> Result<(), Box<dyn Error>> {
        let dir = fs::read_dir(&self.source)?;

        self.flatten_rec(dir)?;

        if !(self.copy || self.keep_dirs) {
            // TODO don't try to delete if current_dir == source or "."
            fs::remove_dir_all(&self.source)?;
        }

        Ok(())
    }

    fn flatten_rec(&self, contents: ReadDir) -> Result<(), Box<dyn Error>> {
        for file in contents {
            let file = file?;
            let file_type = file.file_type()?;
            if file_type.is_dir() {
                let dir = fs::read_dir(file.path())?;
                self.flatten_rec(dir)?;
            } else if file_type.is_file() {
                self.move_file(&file)?;
            }
        }

        Ok(())
    }

    fn move_file(&self, file: &DirEntry) -> Result<(), Box<dyn Error>> {
        let old_path = file.path();
        let mut new_path = Path::new(".").join(file.file_name());
        let mut i = 1;

        while new_path.exists() {
            let mut new_name = file.file_name();
            new_name.push(i.to_string()); // TODO improve naming (don't modify file extension)
            new_path = Path::new(".").join(new_name);
            i += 1;
        }

        if self.copy {
            fs::copy(old_path, new_path)?;
        } else {
            fs::rename(old_path, new_path)?;
        }

        Ok(())
    }
}
