use ssh2::Session;
use std::collections::hash_map::Values;
use std::path::Path;
use std::fs;
use std::io::{Read, Write};
use crate::config::TargetStack;
use crate::error::WrappedError;

pub fn deploy_stacks(session: Session, remote_dir: String, stacks: Values<String, TargetStack>) -> Result<(), WrappedError> {
    for stack in stacks {
        println!("Deploying stack {}", stack.name);
        send_compose_file(&session, stack, remote_dir.clone())?;
        restart_stack(&session, &*remote_dir, &*stack.name)?;
        println!("Successfully deployed stack {}", stack.name)
    }

    return Ok(());
}

fn send_compose_file(session: &Session, stack: &TargetStack, remote_dir: String) -> Result<(), WrappedError> {
    let compose_file = load_compose_file(&stack.name)?;
    let remote_path_string = format!("{}/{}/compose.yml", remote_dir, stack.name);
    let remote_path = Path::new(&remote_path_string);

    write_remote_file(&session, compose_file, remote_path)?;

    return Ok(())
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

fn restart_stack(session: &Session, remote_dir: &str, stack_name: &str) -> Result<(), WrappedError> {
    println!("Restarting stack {}.", stack_name);
    let remote_path_string = &*format!("{}/{}", remote_dir, stack_name);

    compose_exec(session, remote_path_string, &*"down")?;
    compose_exec(session, remote_path_string, &*"pull")?;
    compose_exec(session, remote_path_string, &*"up -d")?;

    return Ok(())
}

fn compose_exec(session: &Session, path: &str, command: &str) -> Result<(), WrappedError> {
    let mut channel = session.channel_session()?;
    let mut buf = String::new();

    channel.exec(&*format!("cd {} ; docker compose {}", path, command))?;
    channel.read_to_string(&mut buf)?;
    println!("{}", buf);
    channel.close()?;
    channel.wait_close()?;

    return Ok(())
}
