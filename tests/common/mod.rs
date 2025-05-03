use std::process::Command;
use std::path::PathBuf;
use std::str;

pub fn get_program_output(file: &str) -> (String, String){
    let mut path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    path.push("target/debug/brainstorm");
    let output = Command::new(path)
        .arg(file)
        .output()
        .expect("Failed to execute command");

    let mut log = String::new();
    log.push_str(match str::from_utf8(&output.stdout) {
        Ok(val) => val,
        Err(..) => panic!("got non UTF-8 data from stderr"),
    });

    let mut errors = String::new();
    errors.push_str(match str::from_utf8(&output.stderr) {
        Ok(val) => val,
        Err(..) => panic!("got non UTF-8 data from stderr"),
    });

    (log, errors)
}

pub fn read_file(path: &str) -> String {
    match std::fs::read_to_string(path){
        Ok(s) => s,
        Err(e) => panic!("{}", e)
    }
}