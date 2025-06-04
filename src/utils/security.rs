pub fn validate_command(command: &str) -> Result<(), ValidationError> {
    let dangerous_patterns = [
        "rm -rf",
        "mkfs",
        "dd",
        "chmod",
        "> /dev",
        "mv /",
        "cp /",
        "shutdown",
        "reboot",
        "kill -9",
        "exec",
        ":(){:|:&};:",
        "format",
        "fdisk",
        "mkfs",
        "> /etc",
        "chown root",
    ];

    if dangerous_patterns.iter().any(|p| command.contains(p)) {
        return Err(ValidationError::DangerousCommand);
    }

    if command.contains("sudo") || command.contains("doas") {
        return Err(ValidationError::ElevatedPrivileges);
    }

    Ok(())
}

#[derive(Debug)]
pub enum ValidationError {
    DangerousCommand,
    ElevatedPrivileges,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ValidationError::DangerousCommand => write!(f, "Command requires special confirmation"),
            ValidationError::ElevatedPrivileges => {
                write!(f, "Command requires elevated privileges")
            }
        }
    }
}

impl std::error::Error for ValidationError {}
