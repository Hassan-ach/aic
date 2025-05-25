use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

#[derive(Debug)]
pub struct CommandExecutor;

impl CommandExecutor {
    pub fn execute(args: &str, v: bool) -> Result<(String, i32), String> {
        //build the command
        let mut cmd = get_shell_command(args, true);
        cmd.stdin(Stdio::inherit());
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        //run command async
        let mut child = cmd
            .spawn()
            .map_err(|e| format!("Failed to start shell: {}", e))?;

        // collecte chrunk from the output or error and creat a buffer reader
        let stdout = child.stdout.take().ok_or("Failed to capture stdout")?;
        let stdout_buff = BufReader::new(stdout);
        let stderr = child.stderr.take().ok_or("Failed to capture stdout")?;
        let stderr_buff = BufReader::new(stderr);

        // push the chrunks to full string
        let mut full_output = String::new();
        let mut full_error = String::new();
        for line in stdout_buff.lines() {
            let line = line.map_err(|e| e.to_string())?;
            if v {
                println!("{}", line); /* Real-time output*/
            }
            full_output.push_str(&line);
            full_output.push('\n');
        }
        for line in stderr_buff.lines() {
            let line = line.map_err(|e| e.to_string())?;
            full_error.push_str(&line);
            full_error.push('\n');
        }

        let status = child.wait().map_err(|e| e.to_string())?;
        if status.success() {
            Ok((full_output, status.code().unwrap_or(-1)))
        } else {
            Err(format!(
                "Command failed with status: {}, error: {}",
                status, full_error
            ))
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
