<<<<<<< HEAD
# Program builder

This is a single program builder, structure:

- `output/` - Output directory for deployed program
- `src/` - Program source code
- `src/main.rs` - entry point to the script
- `src/program.rs` - Program builder code, this is rust helper that allows you to build the program config, this is the only file that should be modified to deploy a program.
- `program_params/` - Program parameters that will injected into the program builder function, each environment will have its own program parameters toml file, name with the environment name, Example: `program_params/local.toml` or `program_params/mainnet.toml`
=======
# Program example

This is an example program that transfer funds from one account to another account and vise versa, and allows authorized address to update the config of both forwarders.

## Program parameters

- owner - The address that is the owner of the program
- denom - The denom we are using in our transfers
- max_first_forward_amount - Maximum amount allowed to be transfered in a single message on first forwarder
- max_second_forward_amount - Maximum amount allowed to be transfered in a single message on second forwarder
- authorized_addr - Authorized address that can change config of the forwaders

## Accounts

We have 2 accounts that funds can be transfered between them:
- First account
- Second account

## Libraries

We have 2 forwarder libraries that enable transferring funds from first account to the second account:

- First forwarder
- Second forwarder

## Authorizations

We have 4 authorizations:

- Forward from first to second - Forward funds from first account to second account
- Forward from second to first - Forward funds from second account to first account
- Secure update first forwarder config - Authorized update config for the first forwarder
- Secure uodate second forwarder config - Authorized update config for the second forwarder
>>>>>>> template/main
