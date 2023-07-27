


use std::path::Path;
use std::fs::File;
use std::process::Command;
use std::env::current_dir;

use crate::venv::new_venv;
use crate::config::Config;



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
    std::fs::write(main_file_path,"if __name__ == '__main__':\n    print('Hello World')").expect("failed to write file");
    
    // Save Config
    let project_path = match path {
        "./" => current_dir().unwrap(),
        _ =>Path::new(path).to_path_buf(),
    };
    let project_name = project_path.parent().unwrap().file_name().unwrap();
    let _system_python_version = Command::new("python3").arg("-V").output().unwrap().stdout;

    let _config = Config::new(path.clone(),project_name.to_str().unwrap(),"3.11.4");
    let _config_path = Path::new("pacify.toml");
    //Config::save(config,config_path.to_str().unwrap().to_string()).expect("failed to save config");


    Ok(())
}
