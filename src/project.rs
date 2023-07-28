use indicatif::{ProgressBar, ProgressState, ProgressStyle};

use std::{
    env::current_dir,
    fs::File,
    path::Path,
    process::{Command, Stdio},
};

use crate::config::Config;
use crate::venv::{new_venv};

pub fn new_project(path: &str) -> Result<(), ()> {
    if Path::new("pacify.toml").exists() {
        println!("pacify enviroment already exists");
        std::process::exit(1);
    }
    // Create venv
    new_venv()?;

    // Create src directory
    let src_path = Path::new("src");
    if !src_path.exists() {
        std::fs::create_dir("src").expect("failed to create src directory");
    }

    // Create Sample main.py
    let main_file_path = Path::new("src/main.py");
    let _main_file = File::create(main_file_path).expect("failed to create main file");

    // Add Sample HelloWorld code
    std::fs::write(
        main_file_path,
        "if __name__ == '__main__':\n    print('Hello World')",
    )
    .expect("failed to write file");

    // Save Config
    let project_path = match path {
        "./" => current_dir().unwrap(),
        _ => Path::new(path).to_path_buf(),
    };
    let project_name = project_path.parent().unwrap().file_name().unwrap();
    let raw_system_python_version = Command::new("python3")
        .arg("-V")
        .stdout(Stdio::inherit())
        .stdin(Stdio::inherit())
        .output()
        .unwrap()
        .stdout;
    let system_python_version = String::from_utf8_lossy(&raw_system_python_version)
        .to_string()
        .replace("Python ", "");

    let _config = Config::new(
        path.clone(),
        project_name.to_str().unwrap(),
        &system_python_version.as_str(),
    );
    let _config_path = Path::new("pacify.toml");
    //Config::save(config,config_path.to_str().unwrap().to_string()).expect("failed to save config");

    Ok(())
}
pub fn run_project() -> Result<(), ()> {
    let config_path = Path::new("pacify.toml");
    if !config_path.exists() {
        println!("pacify enviroment does not exist");
        std::process::exit(1);
    }
    let _ = install_project_dependencies();
    let _ = Command::new("bash")
        .arg("-c")
        .arg("source env/bin/activate; python3 src/main.py")
        .stdout(Stdio::inherit())
        .stdin(Stdio::inherit())
        .output()
        .expect("failed to run project");

    Ok(())
}

fn install_project_dependencies() -> Result<(), ()> {
    let config_path = Path::new("pacify.toml");
    if !config_path.exists() {
        println!("pacify enviroment does not exist!");
        std::process::exit(1);
    }
    let config = Config::load(config_path.to_str().unwrap().to_string()).unwrap();
    let dependencies = config.dependencies.dependencies.clone();

    // Install dependencies
    let pb = ProgressBar::new(dependencies.len() as u64);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {count}/{total_dependencies} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn std::fmt::Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));

    for dependency in dependencies {
        let install_arg = format!("\"{}=={}\"", dependency.name, dependency.version);
        let install_command = "pip3".to_string() + " install " + &install_arg;
        let term = Command::new("bash")
            .arg("-c")
            .arg("source env/bin/activate;".to_string() + install_command.as_str())
            .stdout(Stdio::piped())
            .stdin(Stdio::inherit())
            .output();

        term.expect("failed to install dependency");
        pb.inc(1);
    }
    pb.finish_with_message("installed dependencies");

    Ok(())
}
