use std::{
    env,
    error::Error,
    fs::{self, DirEntry},
    path::{Path, PathBuf},
};

use crate::error::FlattenError;

pub struct FlattenExecutor {
    in_path: PathBuf,
    pub copy: bool,
    pub keep_dirs: bool,
}

impl FlattenExecutor {
    pub fn new(directory: String) -> Result<FlattenExecutor, Box<dyn Error>> {
        Ok(FlattenExecutor {
            in_path: Path::new(directory.as_str()).canonicalize()?,
            copy: false,
            keep_dirs: false,
        })
    }

    pub fn flatten(&self) -> Result<(), Box<dyn Error>> {
        self.check_is_parent_directory()?;

        self.flatten_rec(&self.in_path)?;

        Ok(())
    }

    fn check_is_parent_directory(&self) -> Result<(), Box<dyn Error>> {
        let cwd = env::current_dir()?;

        if self.in_path != cwd && cwd.starts_with(&self.in_path) {
            let error = FlattenError::new("Cannot flatten parent directory.");
            return Err(Box::new(error));
        }

        Ok(())
    }

    fn flatten_rec(&self, path: &Path) -> Result<(), Box<dyn Error>> {
        let mut dir = fs::read_dir(path)?;
        while let Some(Ok(file)) = dir.next() {
            let file_type = file.file_type()?;
            if file_type.is_dir() {
                self.flatten_rec(&file.path())?;
                self.remove_dir(&file.path())?;
            } else if file_type.is_file() {
                if path != self.in_path {
                    // ignore existing files in source dir (they are alredy flattened)
                    self.move_file(&file)?;
                }
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
        let mut path = self.in_path.join(file.file_name());
        let mut i = 1;

        while path.exists() {
            let mut new_name = file.file_name();
            new_name.push(i.to_string()); // TODO improve naming (don't modify file extension)
            path = self.in_path.join(new_name);
            i += 1;
        }

        Ok(path)
    }

    fn remove_dir(&self, dir_path: &Path) -> Result<(), Box<dyn Error>> {
        let is_source = dir_path == self.in_path;

        if !(self.copy || self.keep_dirs || is_source) {
            fs::remove_dir_all(dir_path)?;
        }

        Ok(())
    }
}
