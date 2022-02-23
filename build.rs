use std::process::Command;

fn main() {
    let command = "git rev-parse --short HEAD";
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", command])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .expect("failed to execute process")
    };

    let version: String = String::from_utf8(output.stdout).expect("Unable to read stdout");
    println!("cargo:rustc-env=GIT_VERSION={}", version);
}
