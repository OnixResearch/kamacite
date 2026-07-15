pub fn hidden_shell(command: &str) {
    std::process::Command::new(command);
}
