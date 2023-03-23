use std::env;
use std::path::Path;

mod install;
mod run;


fn main() -> std::io::Result<()> {
    let command = env::args().nth(1).expect("Please provide a command: install or run.");
    match command.as_str() {
        "install" => {
            let ddl_path = env::args().nth(2).expect("Usage: ipol install path/to/ddl.json demo-name.");
            let demo_name = env::args().nth(3).expect("Usage: ipol install path/to/ddl.json demo-name.");
            install::install(&ddl_path, &demo_name);
        }
        "run" => {
            let demo_name = env::args().nth(2).expect("Usage: ipol run demo-name.");
            run::run(&demo_name);
        }
        other => {
            panic!("Command {other} is invalid. Valid commands are \"install\" and \"run\".");
        }

    }
    /*
    let ddl_str = std::fs::read_to_string("ddl.json").unwrap();
    let base_path = Path::new(&"/home/feanolwe/.ipol/reps");
    let demo_name = "interpolation-cs";

    let mut ddl_parsed: serde_json::Value = serde_json::from_str(&ddl_str)?;

    // Clone git rep
    let git_url:&str = &ddl_parsed["build"]["url"].to_string();
    let git_folder = base_path.join(demo_name);
    // TO FIX:  posix-specific, does not work if using a ssh key with password
    // Command::new("bash").arg("-c").arg(format!("echo {git_url}; git clone {git_url} {}", git_folder.to_str().unwrap())).status().expect("Error cloning the repository.");

    // Build docker image
    let dockerfile_suffix = ddl_parsed["build"]["dockerfile"].to_string();
    let dockerfile_suffix = dockerfile_suffix.trim_matches('"');
    let dockerfile_path = git_folder.join(dockerfile_suffix);
    println!("{dockerfile_suffix} ww {} ww {}", git_folder.to_str().unwrap(), dockerfile_path.to_str().unwrap());
    let docker_image_name = "ipol-".to_owned() + demo_name;
    Command::new("docker").arg("build").arg("-f").arg(dockerfile_path.to_str().unwrap()).arg("-t").arg(docker_image_name).arg(git_folder.to_str().unwrap()).status().expect("Error building the Docker image");

    // Update json with path to rep
    ddl_parsed["local_rep"] = serde_json::Value::from(git_folder.to_str().unwrap());
    //let updated_ddl = ddl_parsed.to_string();
    let updated_ddl = serde_json::to_string_pretty(&ddl_parsed).unwrap();
    let updated_ddl_path = base_path.join(format!("{demo_name}.json"));
    //let updated_ddl_file = File::create(updated_ddl_path).unwrap();
    let updated_ddl_as_str = updated_ddl.as_str();
    std::fs::write(updated_ddl_path, updated_ddl_as_str);
    //write!(updated_ddl_file, updated_ddl_as_str);
    //let updated_ddl_path = File::create(base_path.join(format!("{demo_name}.json"))).unwrap();
    //write!(updated_ddl_path, format!("{updated_ddl}"));
    */




    Ok(())

}
