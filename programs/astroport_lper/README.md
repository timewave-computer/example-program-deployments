# Astroport Liquidity Provider
This is a Valence program that deploys and withdraws liquidity on Astroport.
<img width="1063" alt="Diagarm" src="https://github.com/user-attachments/assets/3bdcdd14-1d1d-4e07-aa06-8ef4c20fd915" />

The program contains an account to deposit funds, and account which holds the liqudity position, and an account to send funds when the liquidity is withdrawn.

The libraries used by the program are the [Valence Astroport Lper](https://docs.valence.zone/libraries/cosmwasm/astroport_lper.html) and [Valence Astroport Withdrawer](https://docs.valence.zone/libraries/cosmwasm/astroport_withdrawer.html).

Two subroutines are able to be executed by the programL
1. Permissionlessly deploy liquidity to the specified pool
2. With an authorization token, withdraw liquidity from the pool.
