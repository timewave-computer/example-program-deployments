<<<<<<< HEAD
# Examples of Valence Programs being deployed

This repository contains simple example programs and tooling to demonstrate how to build and deploy programs using Valence's suite of tools.

## Included Examples

1. **[Astroport Liquidity Deployer](programs/astroport_lper)**: Demonstrates a simple program to provide liquidity to and withdraw liquidity from a pool on Astroport.

# How to build and deploy a Program 
=======
# Program deployer

This is a CLI tool to deploy programs using the manager on different environments

It will allow you to use rust to build your program config for deployment, but also allow you to provide a program config in json format.

# Structure

- **Root** - The root folder, all scripts should be ran from here
  - **lib** - Core functionality of the deployer
  - **manager_configs** - Holds different environment manager configs
  - **programs** - Holds all different programs you want to deploy
    - **program_template** - Specific program folder
      - **output** - Holds all past deployments of the program
      - **program_params** - Holds program parameters toml files per environment
      - **src** 
        - **main.rs** - Entry point to the script
        - **program_builder.rs** - Holds your core program specific core using a rust builder pattern
      
# How to
>>>>>>> template/main

## .env

You must include `.env` file in the root of the project with the following variables:

```env
# The mnemonic that will be used by the manager to deploy the program on chains
# The manager is a tool that instantiate the program on all chains and then transfers ownership to the owner
# This should NOT be the owner MNEMONIC, but address that holds enough funds to instantiate the program on all chains.
MANAGER_MNEMONIC=""
```

## Clone the template

A template is provided to help you get started, you can clone it and modify it to deploy your program.

* Don't forget to rename the new program directory and the program name in `Cargo.toml` file.

## Program builder

In your new program directory you will find `src/program_builder.rs` file, this is the file that you will modify to build your program using our rust builder pattern.

## Program parameters

Your program might need some parameters that are unique for a specific environment, you can provide these parameters in the `program_params/` directory.

You should include a file for each environment, the file name should be the environment name, Example: `program_params/local.toml` or `program_params/mainnet.toml`.

Any parameter that is included there will be available in the program builder function, and can be retrieved using the `.get(String)` function, Example: `params.get("my_param")`.

## Run the script

<<<<<<< HEAD
You can deploy your program using the following command:
=======
You can run your program using the following command:
>>>>>>> template/main

```bash
cargo run -p *PROGRAM_NAME*
```

Use the name you gave your program in `Cargo.toml` file.

## Output

After running the script, you will find the output in the `output/` directory.

Each deployed program will have its own directory with the date and time of the deployment, inside each directory you will find:

- `instantiated-program-config.json` - The instantiated program config which includes all the addresses of the contracts of the deployed program
- `debug-program-config.json`- If deployment failed, this file will include the updated program config up to the failed point, this is helpful for debugging failed deployments.
- `raw-program-config.json` - The generated raw program config before instantiation.

By default each deployed program directory is ignored in git, you can remove the ignore rule in the `.gitignore` file if you want to keep the output in your cloned repository.
