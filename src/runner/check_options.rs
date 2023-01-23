use std::process::Command;

use camino::Utf8PathBuf;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct CheckOptions {
    pub path_to_run: Utf8PathBuf,
    pub directory_to_use: Option<Utf8PathBuf>,
}

#[cfg(target_os = "windows")]
fn harness_check(check_options: CheckOptions) -> Command {
    let current_dir = Utf8PathBuf::from_path_buf(std::env::current_dir().unwrap()).unwrap();

    let mut cmd = Command::new("powershell");
    cmd.arg("-ExecutionPolicy")
        .arg("RemoteSigned")
        .arg("-File")
        .arg(current_dir.join(&check_options.path_to_run));

    if let Some(d2u) = check_options.directory_to_use {
        let dir_to_use = current_dir.join(d2u);
        cmd.current_dir(dir_to_use);
    }

    cmd
}

#[cfg(not(target_os = "windows"))]
fn harness_check(check_options: CheckOptions) -> Command {
    let current_dir = std::env::current_dir().unwrap();
    let path = current_dir.join(&check_options.path_to_run);
    let mut cmd = Command::new(path);

    if let Some(d2u) = check_options.directory_to_use {
        let dir_to_use = current_dir.join(d2u);
        cmd.current_dir(dir_to_use);
    }

    cmd
}

/// Run the check option
pub fn run_check(check_options: CheckOptions) -> Result<(), ()> {
    let mut cmd = harness_check(check_options);
    let output = cmd.output().expect("Failed to execute command");

    if let Ok(value) = String::from_utf8(output.stderr) {
        if !value.is_empty() {
            println!("{value}");
        }
    }
    if let Ok(value) = String::from_utf8(output.stdout) {
        print!("{value}");
    }

    if output.status.success() {
        Ok(())
    } else {
        println!(
            "{}: check FAILED with {}",
            console::style("adam error").bright().red(),
            output.status
        );

        Err(())
    }
}
