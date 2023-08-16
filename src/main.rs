mod config;
mod project;
mod venv;
mod version;

use clap::{Arg, ArgAction, Command};

use std::process::exit;

fn cli() -> Command {
    Command::new("pacify")
        .version("0.1.0")
        .author("Skuld Norniern. <skuldnorniern@gmail.com>")
        .about("Yet another Python build/package manager")
        .subcommand_required(true)
        .subcommand(
            Command::new("init").about("Initialize a new project").arg(
                Arg::new("path")
                    .help("Project path")
                    .action(ArgAction::Set)
                    .num_args(1..)
                    .default_value("./"), //.required(true),
            ),
        )
        //.subcommand(Command::new("build").about("Build a project"))
        .subcommand(Command::new("new").about("Create a new project"))
        .subcommand(Command::new("run"))
        //.subcommand(Command::new("test"))
        .subcommand(Command::new("clean"))
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
                        .action(ArgAction::Set),
                ),
        )

        .subcommand(
            Command::new("update").about("Update a package").arg(
                Arg::new("package name")
                    .help("Package name")
                    .action(ArgAction::Set)
                    .num_args(1..)
                    .required(true),
            ),
        )
        .subcommand(Command::new("remove"))
}

fn main() {
    let args = cli().get_matches();

    match args.subcommand() {
        Some(("init", arg)) => {
            project::new_project(
                arg.get_one::<String>("path")
                    .expect("unable to get project path"),
            )
            .expect("failed to create project");
        }
        Some(("run", _)) => {
            project::run_project().expect("failed to run project");
        }
        Some(("add", arg)) => {
            let version_info = arg.get_one::<String>("version");
            project::add_dependency(
                arg.get_one::<String>("package name")
                    .expect("unable to get package name"),
                version_info.map(|v| v.as_str()),
            )
            .expect("failed to add dependency");
        }
        Some(("clean", _)) => {
            project::clean_project().expect("failed to clean project");
        }
        _ => {
            println!("WIP");
            exit(1);
        }
    }
}
