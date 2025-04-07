mod helpers;
mod manager_config;
mod program_config;
mod program_params;

use std::{error::Error, fmt::Display, io::Write, path::PathBuf};

use chrono::Utc;
use clap::{command, Parser};
use dotenvy::dotenv;
use helpers::verify_path;
<<<<<<< HEAD
=======
use log::info;
>>>>>>> template/main
use manager_config::set_manager_config;
use program_config::read_program_config_from_json;
use program_params::get_program_params;
use valence_program_manager::program_config::ProgramConfig;

<<<<<<< HEAD
// Reexport params to programs
pub use program_params::ProgramParams;

// |X| - Read or get the manager config
// |X| - read program parameters into a map
// |X| - helper function to read a parameter from the map
// |X| - call the program builder with the parameters
// deploy the program using the manager
// save program json files (Raw and instantiated)
=======
// Re-export params to programs
pub use helpers::EMPTY_VEC;
pub use program_params::ProgramParams;

#[derive(Debug, PartialEq)]
enum Status {
    Process,
    Success,
    Fail,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Process => write!(f, "process"),
            Status::Success => write!(f, "success"),
            Status::Fail => write!(f, "fail"),
        }
    }
}
>>>>>>> template/main

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Enviroment config to use Ex: mainnet, testnet, local
    #[arg(short, long, default_value = "mainnet")]
    target_env: String,
    /// Absolute path to the program config json file
    #[arg(short, long)]
    program_config_path: Option<String>,
}

pub async fn main<F>(program_path: &str, builder: F) -> Result<(), Box<dyn Error>>
where
    F: Fn(ProgramParams) -> ProgramConfig,
{
<<<<<<< HEAD
    // Load .env file environment variables
=======
    // Enable logs
    env_logger::init();

    println!("Starting program deployment...");

    // Load .env file environment variables
    info!("Loading environment variables from .env file");
>>>>>>> template/main
    dotenv().expect(".env file not found");

    let args = Args::parse();
    let timestamp = Utc::now().format("%Y-%m-%d_%H:%M:%S").to_string();

    // Get and verify paths
    let curr_dir = std::env::current_dir()?;
    let program_path = curr_dir.join(
        PathBuf::from(program_path)
            .parent()
            .unwrap()
            .parent()
            .unwrap(),
    );

<<<<<<< HEAD
    verify_path(program_path.clone())?;

    // Set manager config for the chosen environment
    set_manager_config(&args.target_env).await?;
    
    // If a path to program_config.json was passed, use it
    let mut program_config = if let Some(program_config_path) = args.program_config_path {
        read_program_config_from_json(&program_config_path)
    } else {
        // Else build the program config from the builder
=======
    info!("Verifying program path");
    verify_path(program_path.clone())?;

    let output_path = program_path
        .join("output")
        .join(format!("{}-{}", args.target_env, timestamp));

    // Set manager config for the chosen environment
    info!("Setting manager config for the chosen environment");
    set_manager_config(&args.target_env).await?;

    // If a path to program_config.json was passed, use it
    let mut program_config = if let Some(program_config_path) = args.program_config_path {
        info!("Reading program config from file");
        read_program_config_from_json(&program_config_path)
    } else {
        // Else build the program config from the builder
        info!("Building program config from builder");
>>>>>>> template/main
        let program_params = get_program_params(&program_path, &args.target_env)?;

        builder(program_params)
    };

    // Write the raw program config to file
<<<<<<< HEAD
    write_to_output(program_config.clone(), &program_path, &timestamp, "raw")?;

    // Use program manager to deploy the program
    valence_program_manager::init_program(&mut program_config).await?;

    // Write instantiated program to file
    write_to_output(program_config, &program_path, &timestamp, "instantiated")?;
=======
    info!("Writing raw program config to file");
    write_to_output(&program_config, output_path.clone(), Status::Process, "raw")?;

    // Use program manager to deploy the program
    println!("Instantiating program...");
    match valence_program_manager::init_program(&mut program_config).await {
        Ok(_) => (),
        Err(e) => {
            write_to_output(&program_config, output_path.clone(), Status::Fail, "debug")?;

            return Err(Box::new(e));
        }
    };

    // Write instantiated program to file
    write_to_output(
        &program_config,
        output_path,
        Status::Success,
        "instantiated",
    )?;

    print_success_msg(&program_config).await;
>>>>>>> template/main

    Ok(())
}

fn write_to_output(
<<<<<<< HEAD
    program_config: ProgramConfig,
    program_path: &PathBuf,
    time: &str,
    prefix: &str,
) -> Result<(), Box<dyn Error>> {
    let path = program_path.join("output").join(time);

    if !path.exists() {
        std::fs::create_dir_all(path.clone())?;
=======
    program_config: &ProgramConfig,
    mut path: PathBuf,
    status: Status,
    prefix: &str,
) -> Result<(), Box<dyn Error>> {
    if !path.exists() {
        std::fs::create_dir_all(path.clone())?;
    } else if status != Status::Process {
        let new_path = PathBuf::from(format!("{}-{}", path.to_str().unwrap(), status));
        std::fs::rename(path, new_path.clone())?;
        path = new_path;
>>>>>>> template/main
    }

    // Construct the full file path
    let file_name = format!("{}-program-config.json", prefix);
    let file_path = path.join(file_name.clone());

    // Create and write to the file
    let mut file = std::fs::File::create(file_path.clone())?;

    // Serialize the data to a string
<<<<<<< HEAD
    let content = serde_json::to_string(&program_config)?;
=======
    let content = serde_json::to_string(program_config)?;
>>>>>>> template/main

    file.write_all(content.as_bytes())?;

    Ok(())
}
<<<<<<< HEAD
=======

async fn print_success_msg(program_config: &ProgramConfig) {
    let gc = valence_program_manager::config::GLOBAL_CONFIG.lock().await;
    let ui_link_main = format!("https://app.valence.zone/programs/{}", program_config.id);
    let ui_link = format!("{}?queryConfig={{\"main\":{{\"registryAddress\":\"{}\",\"name\":\"neutron\",\"chainId\":\"neutron-1\",\"rpcUrl\":\"{}\"}},\"external\":[]}}", ui_link_main, gc.general.registry_addr, gc.chains.get("neutron").unwrap().rpc);

    let success_msg = format!(
        "Program deployed successfully! 
Program id: {} 
View program on Valence UI: {}",
        program_config.id, ui_link
    );

    println!("{success_msg}");
}
>>>>>>> template/main
