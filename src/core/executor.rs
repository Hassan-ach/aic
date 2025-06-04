use std::process::{Command, Stdio};

macro_rules! wrap_error {
    ($msg:expr, $err:expr) => {
        CommandOutput {
            status_code: -1,
            output_str: Some(format!("{}: {}", $msg, $err)),
        }
    };
}

#[derive(Debug)]
pub struct CommandExecutor;

#[derive(Debug)]
pub struct CommandOutput {
    pub status_code: i32,
    pub output_str: Option<String>,
}
impl CommandExecutor {
    pub fn execute_at_once(args: &str) -> Result<CommandOutput, CommandOutput> {
        //build the command
        let mut cmd = get_shell_command(args, false);
        cmd.stdin(Stdio::inherit());
        cmd.stdout(Stdio::inherit());
        cmd.stderr(Stdio::inherit());

        //run command async
        let mut child = cmd
            .spawn()
            .map_err(|e| wrap_error!("Failed to start shell", e))?;

        let status = match child.wait() {
            Ok(s) => s,
            Err(e) => {
                return Err(CommandOutput {
                    status_code: -1,
                    output_str: Some(format!("Failed to wait for child: {e}")),
                });
            }
        };
        let status_code = status.code().unwrap_or(-1);
        if status.success() {
            Ok(CommandOutput {
                status_code,
                output_str: None,
            })
        } else {
            Err(CommandOutput {
                status_code,
                output_str: None,
            })
        }
    }
}

#[cfg(target_family = "unix")]
fn get_shell_command(command_line: &str, use_sudo: bool) -> Command {
    let cmd = if use_sudo {
        let mut c = Command::new("sudo");
        c.arg("sh").arg("-c").arg(command_line);
        c
    } else {
        let mut c = Command::new("sh");
        c.arg("-c").arg(command_line);
        c
    };
    cmd
}

#[cfg(target_family = "windows")]
fn get_shell_command(command_line: &str, _use_sudo: bool) -> Command {
    // NOTE: Sudo is not natively supported on Windows
    let mut c = Command::new("cmd");
    c.arg("/C").arg(command_line);
    c
}
