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
        .about("More info at github.com/alvivar/jam\nExample github.com/alvivar/lions")
        .setting(ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("new")
                .setting(ArgRequiredElseHelp)
                .about("Create a Component & System")
                .arg(Arg::with_name("name").help("Name").required(true).index(1))
                .arg(
                    Arg::with_name("include_dependency")
                        .short("d")
                        .help("Include a dependency example using Start()"),
                )
                .arg(
                    Arg::with_name("include_queue")
                        .short("q")
                        .help("Include a queue between the component and system"),
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
        let use_start = matches.is_present("include_dependency");
        let use_queue = matches.is_present("include_queue");
        let output = matches.is_present("output_files");
        let nosys = matches.is_present("no_system");
        let nocomp = matches.is_present("no_component");

        let system_file = format!("{}System.cs", name);
        let system = generate_system_string(name, use_queue);

        let component_file = format!("{}.cs", name);
        let component = get_component(name, use_start, use_queue);

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
            if !nosys || !nocomp {
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

fn generate_system_string(name: &str, use_queue: bool) -> String {
    let queue = r#"if (c.queue.Count > 0)
            {
                var value = c.queue.Dequeue();
            }"#;

    let template = r#"
using UnityEngine;
using System.Collections.Generic;

// #jam
public class @ComponentSystem : MonoBehaviour
{
    public static List<@Component> components = new List<@Component>();

    private void Update()
    {
        foreach (var c in components)
        {
            @Queue
        }
    }
}
"#;

    let mut template = template.to_string();

    template = if use_queue {
        template.replace("@Queue", queue)
    } else {
        template.replace("@Queue", "")
    };

    template.trim().replace("@Component", name)
}

fn get_component(name: &str, use_start: bool, use_queue: bool) -> String {
    let queue_using = r#"
using System.Collections.Generic;"#;

    let queue = r#"
    public Queue<bool> queue = new Queue<bool>();
"#;

    let start = r#"
    public Transform dependency;

    private void Start()
    {
        dependency = GetComponent<Transform>();
    }
"#;

    let template = r#"
using UnityEngine;@QueueUsing

// #jam
public class @Component : MonoBehaviour
{@Queue@Start
    private void OnEnable()
    {
        @ComponentSystem.components.Add(this);
    }

    private void OnDisable()
    {
        @ComponentSystem.components.Remove(this);
    }
}"#;

    let mut template = template.to_string();

    template = if use_start {
        template.replace("@Start", start)
    } else {
        template.replace("@Start", "")
    };

    template = if use_queue {
        template
            .replace("@QueueUsing", queue_using)
            .replace("@Queue", queue)
    } else {
        template.replace("@QueueUsing", "").replace("@Queue", "")
    };

    template.trim().replace("@Component", name)
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
