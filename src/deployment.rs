use ssh2::Session;
use std::collections::hash_map::Values;
use std::path::Path;
use std::fs;
use std::io::Write;
use crate::config::TargetStack;
use crate::error::WrappedError;

pub fn deploy_stacks(session: Session, remote_dir: String, stacks: Values<String, TargetStack>) -> Result<(), WrappedError> {
    for stack in stacks {
        println!("Deploying stack {}", stack.name);
        let compose_file = load_compose_file(&stack.name)?;
        let remote_path_string = format!("{}/{}/compose.yaml", remote_dir, stack.name);
        let remote_path = Path::new(&remote_path_string);

        write_remote_file(&session, compose_file, remote_path)?;
        println!("Successfully deployed stack {}", stack.name)
    }

    return Ok(());
}

fn load_compose_file(stack_name: &str) -> std::io::Result<String> {
    // If yaml only had one valid file extension, I wouldn't have to do this.
    let yaml_path_string = format!("{}/compose.yaml", stack_name);
    let yaml_path = Path::new(&yaml_path_string);
    let yml_path_string = format!("{}/compose.yml", stack_name);
    let yml_path = Path::new(&yml_path_string);

    fs::read_to_string(if yaml_path.exists() { yaml_path } else { yml_path })
}

fn write_remote_file(session: &Session, contents: String, remote_path: &Path) -> Result<(), WrappedError> {
    let mut remote_file = session.scp_send(remote_path, 0o644, contents.len() as u64, None)?;

    remote_file.write(contents.as_bytes())?;
    remote_file.send_eof()?;
    remote_file.wait_eof()?;
    remote_file.close()?;
    remote_file.wait_close()?;
    return Ok(())
}
