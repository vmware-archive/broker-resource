#[cfg(test)]
mod tests {
    use std::process::Command;

    fn build_check() {
        Command::new("cargo").arg("build").status().unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e)
        });
    }

    #[test]
    fn it_outputs_empty_array() {
        build_check();

        let output = Command::new("./target/debug/check").output().unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e)
        });

        assert_eq!(String::from_utf8_lossy(&output.stdout), "[]");
    }
}
