use std::process::Command;
use std::path::Path;

pub fn activate_venv() -> Result<(), ()> {
    //check if venv exists
    let path = Path::new("env");
    if !path.exists() {
        println!("not able to find venv directory\n run 'pcy init' to initialize a new venv");
    }

    #[cfg(windows)]
    let _ = Command::new("./env/bin/activate.bat").status()?;

    #[cfg(posix)]
    let _ = Command::new("source ./env/bin/activate").status()?;

    Ok(())
}   

pub fn deactivate_venv() -> Result<(), ()> {
    //#[cfg(windows)]
    //Command("./env/bin/deactivate.bat").status()?;
    //#[cfg(posix)]
    let _ = Command::new("deactivate").status().expect("deactivate failed");
    Ok(())
}

pub fn new_venv() -> Result<(), ()> {
    //let path = Path::new("env");
    //if !path.exists() {
    //    std::fs::create_dir("env").expect("failed to create env directory");
    //}
    let te = Command::new("virtualenv").arg("env").output();
    println!("{:?}",te);
    Ok(())
}
