use std::process::Command;

/*pub fn activate_venv() -> Result<(), ()> {
    //check if venv exists
    let path = Path::new("env");
    if !path.exists() {
        println!("not able to find venv directory\n run 'pcy init' to initialize a new venv");
    }

    #[cfg(target_os = "windows")]
    let _ = Command::new("./env/bin/activate.bat")
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .output()
        .expect("activate failed");

    #[cfg(target_os = "linux")]
    //let sd = Command::new("exa").arg("-T").stdin(Stdio::inherit()).stdout(Stdio::inherit()).output().expect("activate failed");
    //println!("{:?}",sd);
    let te = Command::new("bash")
        .arg("-c")
        .arg("source env/bin/activate")
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .output()
        .expect("activate failed");
    println!("{:?}", te);

    #[cfg(target_os = "macos")]
    let _ = Command::new("source")
        .arg("env/bin/activate")
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .output()
        .expect("activate failed");

    Ok(())
}*/

/*pub fn deactivate_venv() -> Result<(), ()> {
    //#[cfg(windows)]
    //Command("./env/bin/deactivate.bat").status()?;
    //#[cfg(posix)]
    let _ = Command::new("deactivate")
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .output()
        .expect("deactivate failed");
    Ok(())
}*/

pub fn new_venv() -> Result<(), ()> {
    //let path = Path::new("env");
    //if !path.exists() {
    //    std::fs::create_dir("env").expect("failed to create env directory");
    //}
    let _ = Command::new("virtualenv").arg("env").output();
    Ok(())
}
