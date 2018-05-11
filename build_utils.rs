use cc::Build;
use std::path::Path;
use walkdir::WalkDir;

pub trait BuildExt {
    fn all_files<P: AsRef<Path>>(&mut self, p: P, ext: &str) -> &mut Self;
}

impl BuildExt for Build {
    fn all_files<P: AsRef<Path>>(&mut self, p: P, ext: &str) -> &mut Self {
        for entry in WalkDir::new(p).into_iter().filter_map(|e| e.ok()) {
            if let Some(ext_str) = entry.path().extension() {
                if ext_str == ext {
                    self.file(entry.path());
                }
            }
        }

        self
    }
}
