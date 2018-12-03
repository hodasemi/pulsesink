use std::process::{Command, Stdio};
use std::str::from_utf8;

pub fn bash(programm: &str, args: &[&str]) -> Result<Vec<String>, String> {
    let out = match Command::new(programm)
        .args(args)
        .stdin(Stdio::null())
        .output()
    {
        Ok(out) => out,
        Err(why) => return Err(format!("{} error {:?}", programm, why)),
    };

    if !out.status.success() {
        let stdout = match String::from_utf8(out.stdout) {
            Ok(string) => string,
            Err(_) => String::new(),
        };

        let stderr = match String::from_utf8(out.stderr) {
            Ok(string) => string,
            Err(_) => String::new(),
        };

        return Err(format!(
            "Shell error\nstdout: {}\nstderr: {}",
            stdout, stderr
        ));
    }

    match from_utf8(out.stdout.as_slice()) {
        Ok(string) => Ok(string.split("\n").map(|s| s.trim().to_string()).collect()),
        Err(_) => Err("error converting output".to_string()),
    }
}
