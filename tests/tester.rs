use std::env;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::process::Command;

#[derive(Default)]
pub struct Tester {
    exe: PathBuf,
}

impl Tester {
    pub fn new() -> Self {
        let root = env::current_exe()
            .expect("tests executable")
            .parent()
            .expect("tests executable directory")
            .parent()
            .expect("pp executable directory")
            .to_path_buf();

        let name = env!("CARGO_PKG_NAME");
        let exe_name = if cfg!(windows) { format!("{}.exe", name) } else { String::from(name) };

        Tester {
            exe: root.join(exe_name),
        }
    }

    pub fn test(&self, input: &str, prettified: &str) {
        let input = format!("tests/json/{}", input);
        let prettified = format!("tests/json/{}", prettified);

        let root_dir = env!("CARGO_MANIFEST_DIR");

        let output = Command::new(&self.exe)
            .arg(input)
            .current_dir(root_dir)
            .output()
            .expect("pp failed")
            .stdout;

        let actual = String::from_utf8_lossy(&output);
        let expected = read_to_string(prettified).expect("could not read prettified file");

        assert_eq!(expected, actual);
    }
}
