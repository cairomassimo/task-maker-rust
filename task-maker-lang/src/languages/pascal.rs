use crate::languages::Language;
use std::path::{Path, PathBuf};
use task_maker_dag::*;

/// The Pascal language.
#[derive(Debug)]
pub struct LanguagePascal;

impl LanguagePascal {
    /// Make a new LanguageC using the specified version.
    pub fn new() -> LanguagePascal {
        LanguagePascal {}
    }
}

impl Language for LanguagePascal {
    fn name(&self) -> &'static str {
        "Pascal / fpc"
    }

    fn extensions(&self) -> Vec<&'static str> {
        vec!["pas"]
    }

    fn need_compilation(&self) -> bool {
        true
    }

    fn compilation_command(&self, _path: &Path) -> ExecutionCommand {
        ExecutionCommand::system("fpc")
    }

    fn compilation_args(&self, path: &Path) -> Vec<String> {
        let exe_name = self.executable_name(path);
        let exe_name = exe_name.to_string_lossy();
        let args = vec!["-dEVAL", "-Fe/dev/stderr", "-O2", "-XS"];
        let mut args: Vec<_> = args.into_iter().map(|s| s.to_string()).collect();
        args.push("-o".to_owned() + exe_name.as_ref());
        args.push(
            path.file_name()
                .expect("Invalid source file name")
                .to_string_lossy()
                .to_string(),
        );
        args
    }

    fn compilation_add_file(&self, mut args: Vec<String>, file: &Path) -> Vec<String> {
        args.push(file.to_string_lossy().to_string());
        args
    }

    /// The executable name is the source file's one without the extension.
    fn executable_name(&self, path: &Path) -> PathBuf {
        let name = PathBuf::from(path.file_name().expect("Invalid source file name"));
        PathBuf::from(name.file_stem().expect("Invalid source file name"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use spectral::prelude::*;

    #[test]
    fn test_compilation_args() {
        let lang = LanguagePascal::new();
        let args = lang.compilation_args(Path::new("foo.pas"));
        assert_that!(args).contains("foo.pas".to_string());
        assert_that!(args).contains("-ofoo".to_string());
    }

    #[test]
    fn test_compilation_add_file() {
        let lang = LanguagePascal::new();
        let args = lang.compilation_args(Path::new("foo.pas"));
        let new_args = lang.compilation_add_file(args.clone(), Path::new("bar.pas"));
        assert_that!(new_args.iter()).contains_all_of(&args.iter());
        assert_that!(new_args.iter()).contains("bar.pas".to_string());
    }

    #[test]
    fn test_executable_name() {
        let lang = LanguagePascal::new();
        assert_that!(lang.executable_name(Path::new("foo.pas"))).is_equal_to(PathBuf::from("foo"));
    }
}