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
        .about("Info github.com/alvivar/jam\nExample github.com/alvivar/lions")
        .setting(ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("new")
                .setting(ArgRequiredElseHelp)
                .about("Create a Component & System")
                .arg(Arg::with_name("name").help("Name").required(true).index(1))
                .arg(
                    Arg::with_name("include_start")
                        .short("s")
                        .help("Include void Start()"),
                )
                .arg(
                    Arg::with_name("output_files")
                        .short("o")
                        .help("Create the .cs files"),
                )
                .arg(
                    Arg::with_name("no_system")
                        .long("ns")
                        .help("Ignore the System"),
                )
                .arg(
                    Arg::with_name("no_component")
                        .long("nc")
                        .help("Ignore the Component"),
                ),
        )
        .subcommand(SubCommand::with_name("update").about("Self updates to the latest version"))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("new") {
        let name = matches.value_of("name").unwrap();
        let start = matches.is_present("include_start");
        let output = matches.is_present("output_files");
        let nosys = matches.is_present("no_system");
        let nocomp = matches.is_present("no_component");

        let system_file = format!("{}System.cs", name);
        let system = generate_system_string(name);

        let component_file = format!("{}.cs", name);
        let component = if start {
            get_extended_component(name)
        } else {
            get_component(name)
        };

        if output {
            let current_dir = env::current_dir().unwrap();

            if !nosys {
                write_file(system.as_str(), current_dir.join(&system_file));
            }

            if !nocomp {
                write_file(component.as_str(), current_dir.join(&component_file));
            }
        }

        if !nosys {
            println!("\n\n{}", system);
        }

        if !nocomp {
            println!("\n\n{}", component);
        }

        if output {
            if !nocomp || !nosys {
                println!();
                println!();
            }

            if !nosys {
                println!("{} generated", system_file);
            }

            if !nocomp {
                println!("{} generated", component_file);
            }
        }

        if !output && (!nosys || !nocomp) {
            println!();
        }
        println!("\nDone!");
    }

    if let Some(_matches) = matches.subcommand_matches("update") {
        println!();

        if update().is_err() {
            panic!("Error updating.")
        }
    }
}

fn generate_system_string(name: &str) -> String {
    let template = r#"

using System.Collections.Generic;
using UnityEngine;

// #jam
public class @ComponentSystem : MonoBehaviour
{
    public static List<@Component> components = new List<@Component>();

    private void Update()
    {
        foreach (var c in components)
        {

        }
    }
}
"#;

    template.replace("@Component", name).trim().to_string()
}

fn get_component(name: &str) -> String {
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

fn get_extended_component(name: &str) -> String {
    let template = r#"

using UnityEngine;

// #jam
public class @Component : MonoBehaviour
{
    public Transform collider;

    private void Start()
    {
        collider = GetComponent<Transform>();
    }

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
