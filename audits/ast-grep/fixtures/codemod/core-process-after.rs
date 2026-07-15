pub fn admitted_shell(shell: &mut ProcessShell, command: &str) {
    shell.spawn(command);
}
