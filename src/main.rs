use std::{
    env,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use clap::{App, AppSettings::ArgRequiredElseHelp, Arg, SubCommand};
use self_update::cargo_crate_version;

fn main() {
    let matches = App::new("jam")
        .version(cargo_crate_version!())
        .about("Check out github.com/alvivar/jam for more info!")
        .setting(ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("new")
                .setting(ArgRequiredElseHelp)
                .about("Create a Component & System")
                .arg(
                    Arg::with_name("name")
                        .help("Name for the Component & System")
                        .required(true)
                        .index(1),
                )
                .arg(Arg::with_name("output").short("o").help("Create the files"))
                .arg(
                    Arg::with_name("nocomp")
                        .long("nocomp")
                        .help("Ignore the Component"),
                )
                .arg(
                    Arg::with_name("nosys")
                        .long("nosys")
                        .help("Ignore the System"),
                ),
        )
        .subcommand(
            SubCommand::with_name("update").about("Self updates to the latest release on Github"),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("new") {
        let name = matches.value_of("name").unwrap();
        let output = matches.is_present("output");
        let nocomp = matches.is_present("nocomp");
        let nosys = matches.is_present("nosys");

        let component = generate_component_string(name);
        let system = generate_system_string(name);

        let component_file = format!("{}.cs", name);
        let system_file = format!("{}System.cs", name);

        if output {
            let current_dir = env::current_dir().unwrap();

            if !nocomp {
                write_file(component.as_str(), current_dir.join(&component_file));
            }

            if !nosys {
                write_file(system.as_str(), current_dir.join(&system_file));
            }
        }

        if !nocomp {
            println!("\n\n{}", component);
        }

        if !nosys {
            println!("\n\n{}", system);
        }

        if output {
            if !nocomp || !nosys {
                println!();
                println!();
            }

            if !nocomp {
                println!("{} generated", component_file);
            }

            if !nosys {
                println!("{} generated", system_file);
            }
        }

        if !output && (!nocomp || !nosys) {
            println!();
        }
        println!("\nDone!");
    }

    if let Some(_matches) = matches.subcommand_matches("update") {
        println!();

        match update() {
            Ok(_) => {}
            Err(_) => {
                panic!("Error updating.")
            }
        }
    }
}

fn generate_component_string(name: &str) -> String {
    let template = r#"

using UnityEngine;

// #jam
public class @Component : MonoBehaviour
{
    private void OnEnable()
    {
        @ComponentSystem.entities.Add(this);
    }

    private void OnDisable()
    {
        @ComponentSystem.entities.Remove(this);
    }
}
"#;

    template.replace("@Component", name).trim().to_string()
}

fn generate_system_string(name: &str) -> String {
    let template = r#"

using System.Collections.Generic;
using UnityEngine;

// #jam
public class @ComponentSystem : MonoBehaviour
{
    public static List<@Component> entities = new List<@Component>();

    private void Update()
    {
        foreach (var e in entities)
        {

        }
    }
}
"#;

    template.replace("@Component", name).trim().to_string()
}

fn write_file(data: &str, filepath: impl AsRef<Path>) {
    let f = File::create(filepath).unwrap();
    let mut f = BufWriter::new(f);
    f.write_all(data.as_bytes()).unwrap();
}

fn update() -> Result<(), Box<dyn std::error::Error>> {
    let status = self_update::backends::github::Update::configure()
        .repo_owner("alvivar")
        .repo_name("jam")
        .bin_name("jam")
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;

    println!("Current version... v{}", status.version());

    Ok(())
}
