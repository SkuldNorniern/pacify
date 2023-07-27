mod venv;
mod config;
mod project;

use clap::{Arg, ArgAction, Command};

use std::process::exit;

fn cli() -> Command {
    Command::new("pacify")
        .version("0.1.0")
        .author("Skuld Norniern. <skuldnorniern@gmail.com>")
        .about("Yet another Python build/package manager")
        .subcommand_required(true)
        .subcommand(
            Command::new("init")
                .about("Initialize a new project")
                .arg(
                    Arg::new("path")
                        .help("Project path")
                        .action(ArgAction::Set)
                        .num_args(1..)
                        .default_value("./")
                        //.required(true),
                ),
        )
        .subcommand(
            Command::new("build")
                .about("Build a project")
        ) 
        .subcommand(
            Command::new("new")
                .about("Create a new project")
        )
        .subcommand(
            Command::new("run")
        )
        .subcommand(
            Command::new("test")
        )
        .subcommand(
            Command::new("clean")
        )
        .subcommand(
            Command::new("add")
                .about("Add a dependency")
                .arg(
                    Arg::new("package name")
                        .help("Package name")
                        .action(ArgAction::Set)
                        .num_args(1..)
                        .required(true),
                )
                .arg(
                    Arg::new("version")
                        .help("Version")
                        .short('v')
                        .long("version")
                        .action(ArgAction::Set)
                )
        )
        .subcommand(
            Command::new("update")
                .about("Update a package")
                .arg(
                    Arg::new("package name")
                        .help("Package name")
                        .action(ArgAction::Set)
                        .num_args(1..)
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("remove")
        )
}

fn main() {

    let args = cli().get_matches();


    match args.subcommand() {
       Some(("init", arg)) => {
           project::new_project(arg.get_one::<String>("path").expect("unable to get project path"));
       } 
        _ => {
            println!("WIP");
            exit(1);
        }
}
}
