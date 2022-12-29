use std::{
    env,
    error::Error,
    fs::{self, DirEntry, ReadDir},
    path::{Path, PathBuf},
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
        let source_path = Path::new(&self.source).canonicalize()?;

        let dir = fs::read_dir(&source_path)?;

        self.flatten_rec(dir)?;
        self.remove_dir(&source_path)?;

        Ok(())
    }

    fn flatten_rec(&self, contents: ReadDir) -> Result<(), Box<dyn Error>> {
        for file in contents {
            let file = file?;
            let file_type = file.file_type()?;
            if file_type.is_dir() {
                let dir = fs::read_dir(file.path())?;
                self.flatten_rec(dir)?;
                self.remove_dir(&file.path())?;
            } else if file_type.is_file() {
                self.move_file(&file)?;
            }
        }

        Ok(())
    }

    fn move_file(&self, file: &DirEntry) -> Result<(), Box<dyn Error>> {
        let old_path = file.path();
        let new_path = self.create_new_file_name(file)?;

        if self.copy {
            fs::copy(old_path, new_path)?;
        } else {
            fs::rename(old_path, new_path)?;
        }

        Ok(())
    }

    fn create_new_file_name(&self, file: &DirEntry) -> Result<PathBuf, Box<dyn Error>> {
        let mut path = Path::new(".").join(file.file_name());
        let mut i = 1;

        while path.exists() {
            let mut new_name = file.file_name();
            new_name.push(i.to_string()); // TODO improve naming (don't modify file extension)
            path = Path::new(".").join(new_name);
            i += 1;
        }

        Ok(path)
    }

    fn remove_dir(&self, dir_path: &Path) -> Result<(), Box<dyn Error>> {
        let is_cwd = dir_path == env::current_dir()?;

        if !(self.copy || self.keep_dirs || is_cwd) {
            fs::remove_dir_all(dir_path)?;
        }

        Ok(())
    }
}
