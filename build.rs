use std::{env, error::Error, process::Command};

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=static_src");
    println!("cargo:rerun-if-changed=templates");
    println!("cargo:rerun-if-changed=build.rs");

    env::set_current_dir("static_src")?;

    Command::new("yarn").args(&["install"]).output()?;
    Command::new("yarn").args(&["check"]).output()?;

    if Ok("release".to_owned()) == env::var("PROFILE") {
        Command::new("yarn").args(&["compile:prod"]).output()?;
    } else {
        Command::new("yarn").args(&["compile"]).output()?;
    }

    Ok(())
}
