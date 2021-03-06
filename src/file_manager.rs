use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::io;
use walkdir::{WalkDir, IntoIter};
pub struct FileManager {

}

impl FileManager {
    pub fn new() -> Self {
        FileManager{}
    }
}
#[cfg_attr(test,mockall::automock)]
pub trait HandleFiles {
  fn remove_file(&self, path: &str) -> io::Result<()>;
  fn get_full_path(&self, srcdir: &PathBuf) -> io::Result<PathBuf>;
  fn walkdir(&self, srcdir: &str) -> IntoIter;
  fn get_file(&self, path: &PathBuf) -> io::Result<File>;
}

impl HandleFiles for FileManager {
    fn remove_file(&self, path: &str) -> io::Result<()>{
        fs::remove_file(path)
    }
    fn get_full_path(&self, srcdir: &PathBuf) -> io::Result<PathBuf>{
        fs::canonicalize(&srcdir)
    }
    fn walkdir(&self, srcdir: &str) -> IntoIter{
        WalkDir::new(srcdir).into_iter()
    }
    fn get_file(&self, path: &PathBuf) -> io::Result<File> {
        File::open(path)
    }
}