use std::process::Command;

fn main() {
    let output = Command::new("cargo")
        .arg("fmt")
        .output()
        .expect("Failed to run command");

    if output.status.success() {
        let clear_output = String::from_utf8_lossy(&output.stdout);
        println!("{}", clear_output);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Failed to run command. error: {}", stderr);
    }
}
