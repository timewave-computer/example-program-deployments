# Astroport Liquidity Provider
[This builder](src/program_builder.rs) demonstrates how a simple Valence program can be constructed to deploy and withdraw liquidity on Astroport. The program is on a single domain and has no crosschain functionality. It demonstrates the use of 
two libraries:
- [Valence Astroport Lper](https://docs.valence.zone/libraries/cosmwasm/astroport_lper.html) that provides liquidity; and
- [Valence Astroport Withdrawer](https://docs.valence.zone/libraries/cosmwasm/astroport_withdrawer.html) that withdraws the liquidity

## Program Design
The program is designed with three Valence Accounts
1. An *input account* where deposited funds are received
2. A *liquidity position* account which holds the LP tokens issued after the pool is joined
3. An *output account* where tokens are withdrawn into

Two subroutines are able to be executed by the program
1. **Provide Liquidity**: Permissionlessly deploy liquidity to the specified pool. Tokens from the input account are used to join the pool and the issued LP tokens are sent to the liquidity position account.
2. **Withdraw Liquidity**: With an authorization token, withdraw liquidity from the pool. LP tokens from the liquidity position account are used up and withdrawn tokens are sent to the output account.

<img width="1063" alt="Diagarm" src="https://github.com/user-attachments/assets/3bdcdd14-1d1d-4e07-aa06-8ef4c20fd915" />
