use std::path::Path;
use std::process::Command;
use std::ptr::write;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
//#[serde(rename_all="camelCase")]
struct Config {
    version: u32,
}

pub fn install(ddl_path: &str, demo_name: &str) {
    let base_path = std::env::home_dir()
        .expect("Impossible to get the home directory")
        .join(Path::new(".ipol/reps/"));
    let ddl_as_str = std::fs::read_to_string(ddl_path).expect("Error reading the DDL.");

    let mut ddl_parsed: serde_json::Value =
        serde_json::from_str(&ddl_as_str).expect("Error parsing the provided DDL.");

    // Clone git rep
    let git_url: &str = &ddl_parsed["build"]["url"].to_string();
    let git_folder = base_path.join(demo_name);
    if git_folder.exists() {
        println!("Git repo already exists, deleting it...");
        std::fs::remove_dir_all(&git_folder).expect("Error deleting the existing git directory.");
    }
    // TODO for now, posix-specific, and does not work if using a ssh key with password
    Command::new("bash")
        .arg("-c")
        .arg(format!(
            "git clone {git_url} {}",
            &git_folder.to_str().expect("git folder is incorrectly set.")
        ))
        .status()
        .expect("Error cloning the repository.");

    // Build docker image
    let dockerfile_suffix = &ddl_parsed["build"]["dockerfile"];
    let binding = dockerfile_suffix.to_string();
    let dockerfile_suffix = binding.trim_matches('"');
    let dockerfile_path = git_folder.join(dockerfile_suffix);
    let docker_image_name = "ipol-".to_owned() + demo_name;
    Command::new("docker")
        .arg("build")
        .arg("-f")
        .arg(
            dockerfile_path
                .to_str()
                .expect("Dockerfile path is incorrectly set"),
        )
        .arg("-t")
        .arg(&docker_image_name)
        .arg(git_folder.to_str().unwrap())
        .status()
        .expect("Error building the Docker image");

    // Update json
    ddl_parsed["local_rep"] = serde_json::Value::from(git_folder.to_str().unwrap());
    ddl_parsed["docker_image"] = serde_json::Value::from(docker_image_name);
    let updated_ddl =
        serde_json::to_string_pretty(&ddl_parsed).expect("error formatting back the ddl");
    let updated_ddl_path = base_path.join(format!("{demo_name}.json"));
    std::fs::write(updated_ddl_path, updated_ddl.as_str()).expect("Error writing the updated ddl");
}
