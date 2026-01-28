use std::process::Command;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TerminalError {
    #[error("No supported terminal emulator found")]
    NoTerminalFound,
    #[error("Failed to launch terminal: {0}")]
    LaunchFailed(String),
}

#[cfg(target_os = "linux")]
const LINUX_TERMINALS: &[(&str, &[&str])] = &[
    ("gnome-terminal", &["--", "ssh"]),
    ("konsole", &["-e", "ssh"]),
    ("alacritty", &["-e", "ssh"]),
    ("kitty", &["ssh"]),
    ("xterm", &["-e", "ssh"]),
];

pub fn open_ssh_in_terminal(username: &str, host: &str, port: u16) -> Result<(), TerminalError> {
    let ssh_target = if port == 22 {
        format!("{}@{}", username, host)
    } else {
        format!("{}@{}", username, host)
    };

    let port_args: Vec<String> = if port != 22 {
        vec!["-p".to_string(), port.to_string()]
    } else {
        vec![]
    };

    #[cfg(target_os = "linux")]
    {
        for (terminal, base_args) in LINUX_TERMINALS {
            if which::which(terminal).is_ok() {
                let mut args: Vec<&str> = base_args.to_vec();

                let port_arg_refs: Vec<&str> = port_args.iter().map(|s| s.as_str()).collect();
                args.extend(port_arg_refs.iter());
                args.push(&ssh_target);

                Command::new(terminal)
                    .args(&args)
                    .spawn()
                    .map_err(|e| TerminalError::LaunchFailed(e.to_string()))?;

                return Ok(());
            }
        }
        Err(TerminalError::NoTerminalFound)
    }

    #[cfg(target_os = "macos")]
    {
        let ssh_cmd = if port != 22 {
            format!("ssh -p {} {}", port, ssh_target)
        } else {
            format!("ssh {}", ssh_target)
        };

        let script = format!(
            r#"tell application "Terminal"
                activate
                do script "{}"
            end tell"#,
            ssh_cmd
        );

        Command::new("osascript")
            .args(["-e", &script])
            .spawn()
            .map_err(|e| TerminalError::LaunchFailed(e.to_string()))?;

        Ok(())
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    {
        Err(TerminalError::NoTerminalFound)
    }
}
