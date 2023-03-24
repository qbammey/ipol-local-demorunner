use serde_json::Value;
use std::fmt::format;
use std::path::Path;
use std::process::Command;
use std::{env, fs};

pub fn run(demo_name: &str) {
    let home_path = std::env::home_dir().expect("Impossible to get the home directory");
    let base_path = home_path.join(Path::new(".ipol/reps"));
    let ddl_path = base_path.join(format!("{demo_name}.json"));
    let ddl_as_str = std::fs::read_to_string(ddl_path).expect("Error reading the DDL.");
    let ddl_parsed: serde_json::Value =
        serde_json::from_str(&ddl_as_str).expect("Error parsing the provided DDL.");

    // Retrieve the run command
    let binding = &ddl_parsed["run"].to_string();
    let mut run_command = format!("{} | tee stdout.txt", binding.trim_matches('"'));
    //let mut run_command = String::from(binding.trim_matches('"'));
    run_command = run_command.replace("$bin", "/workdir/bin");

    // get the inputs
    let expected_inputs: Vec<Value> =
        serde_json::from_str(&ddl_parsed["inputs"].to_string()).expect("Error reading DDL inputs.");
    //let expected_inputs: &Value = &ddl_parsed["inputs"];
    let out_folder = env::args()
        .nth(3)
        .expect("Usage: ipol run demo-name out-folder ...");
    let out_folder = Path::new(&out_folder);
    if out_folder.exists() {
        panic!(
            "output folder {} already exists! Exiting to prevent overwriting",
            out_folder.to_str().unwrap()
        );
    }
    fs::create_dir(out_folder).expect("Error creating output folder");
    let mut n_current_arg = 4;
    let mut n_input = 0;
    for expected_input in expected_inputs.iter() {
        let binding: String = expected_input["type"].to_string();
        let input_type: &str = binding.trim_matches('"');
        match input_type {
            "image" => {
                let input_name = env::args()
                    .nth(n_current_arg)
                    .expect("Usage: ipol run demo-name out_folder input0 input1 ... out-folder");
                let binding = expected_input["ext"].to_string();
                let ext = binding.trim_matches('"');
                let input_store_name = format!("input_0{ext}");
                let input_store_path = out_folder.join(input_store_name);
                Command::new("convert")
                    .arg(input_name)
                    .arg(input_store_path.to_str().unwrap())
                    .status()
                    .expect("Error converting or storing the input");
                env::set_var(
                    format!("input{n_input}"),
                    input_store_path.to_str().unwrap(),
                );
                let input_name = format!("$input{n_input}");
                run_command = run_command.replace(input_name.as_str(), input_store_path.to_str().unwrap());
                n_input += 1;
                n_current_arg += 1;
            }
            _ => panic!("Only image inputs are supported for now."),
        }
    }

    // Set params values
    let args: Vec<String> = env::args().collect();
    for i_arg in (n_current_arg..args.len()).step_by(2) {
        let name = args[i_arg].trim_matches('-');
        let value = args[i_arg + 1].as_str();
        let name = "$".to_owned() + name;
        run_command = run_command.replace(name.as_str(), value);
    }

    // Set default params values where unset
    let parameters: Vec<Value> = serde_json::from_str(&ddl_parsed["params"].to_string())
        .expect("Error reading DDL parameters.");
    for param in parameters {
        let binding = {
            let binding2 = &param["type"].to_string();
            let param_type = binding2.trim_matches('"');
            match param_type {
                "numeric" | "range" | "text" => &param["values"]["default"],
                "selection_collapsed" | "selection_radio" | "checkbox" | "text_area" => {
                    &param["default_value"]
                }
                "label" => continue,
                other => panic!("Unknown param type {other}.")
            }
                .to_string()
        };
        let default = binding.trim_matches('"');
        let binding = &param["id"].to_string();
        let param_name = binding.trim_matches('"');
        let param_name = ("$".to_owned()+param_name);
        run_command = run_command.replace(param_name.as_str(), default);
    }



    //Run
    let docker_image_name = "ipol-".to_owned() + demo_name;
    let arg_v = format!("{}:/workdir/exec", std::fs::canonicalize(out_folder).unwrap().to_str().unwrap());
    println!("{arg_v}");
    Command::new("docker")
        .arg("run")
        .arg("--rm")
        .arg("-v")
        .arg(arg_v)
        .arg("--workdir")
        .arg("/workdir/exec")
        .arg("--user")
        //.arg("$(id -u):$(id -g)")
        .arg("1000:1000")
        .arg(docker_image_name)
        .arg("bash")
        .arg("-c")
        .arg(run_command)
        .status()
        .expect("Error running the docker image");


}
