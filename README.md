# Examples of Valence Programs being deployed

This repository contains simple example programs and tooling to demonstrate how to build and deploy programs using Valence's suite of tools.

## Included Examples

1. **[Astroport Liquidity Deployer](programs/astroport_lper)**: Demonstrates a simple program to provide liquidity to and withdraw liquidity from a pool on Astroport.

# How to build and deploy a Program 

## .env

You must include `.env` file in the root of the project with the following variables:

```env
# The mnemonic that will be used by the manager to deploy the program on chains
MANAGER_MNEMONIC=""
```

## Clone the template

A template is provided to help you get started, you can clone it and modify it to deploy your program.

Don't forget to rename the new program directory and the `Cargo.toml` file.

## Program builder

In your new program directory you will find `src/program_builder.rs` file, this is the file that you will modify to build your program using our rust builder pattern.

## Program parameters

Your program might need some parameters that are unique for a specific environment, you can provide these parameters in the `program_params/` directory.

You should include a file for each environment, the file name should be the environment name, Example: `program_params/local.toml` or `program_params/mainnet.toml`.

Any parameter that is included there will be available in the program builder function, and can be retrieved using the `.get(String)` function, Example: `params.get("my_param")`.

## Run the script

You can deploy your program using the following command:

```bash
cargo run -p *PROGRAM_NAME*
```

Use the name you gave your pgoram in `Cargo.toml` file.

## Output

After running the script, you will find the output in the `output/` directory.

Each deployed program will have its own directory with the date and time of the deployment, inside each directory you will find:

- `instantiated-program-config.json` - The instantiated program config which includes all the addresses of the contracts of the deployed program
- `raw-program-config.json` - The generated raw program config before instantiation.

By default each deployed program directory is ignored in git, you can remove the ignore rule in the `.gitignore` file if you want to keep the output in your cloned repository.
