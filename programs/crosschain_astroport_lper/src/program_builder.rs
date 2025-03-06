use std::collections::BTreeMap;

use valence_authorization_utils::{
    authorization::{ AuthorizationModeInfo, PermissionTypeInfo },
    authorization_message::{ Message, MessageDetails, MessageType, ParamRestriction },
    builders::{ AtomicFunctionBuilder, AtomicSubroutineBuilder, AuthorizationBuilder },
};
use valence_generic_ibc_transfer_library::msg::{IbcTransferAmount, RemoteChainInfo};
use valence_ibc_utils::types::PacketForwardMiddlewareConfig;
use valence_program_manager::{
    account::{ AccountInfo, AccountType },
    library::{ LibraryConfig, LibraryInfo },
    program_config::ProgramConfig,
    program_config_builder::ProgramConfigBuilder,
};
use valence_astroport_utils::PoolType;
use valence_astroport_lper::msg::LiquidityProviderConfig;
use valence_astroport_withdrawer::msg::LiquidityWithdrawerConfig;
use valence_library_utils::{liquidity_utils::AssetData, GetId};

/// Write your program using the program builder
pub fn program_builder(params: deployer_lib::ProgramParams) -> ProgramConfig {
    // program params
    let owner = params.get("owner");
    let pool_addr = params.get("pool_addr");
    let ntrn_on_neutron = params.get("ntrn_on_neutron");
    let atom_on_neutron = params.get("atom_on_neutron");
    let atom_on_juno = params.get("atom_on_juno");
    let permissioned_withdrawer = params.get("permissioned_withdrawer");
    let juno_gaia_ibc_channel_id = params.get("juno_gaia_ibc_channel_id");
    let gaia_neutron_ibc_channel_id = params.get("gaia_neutron_ibc_channel_id");

    // Initialize builder
    let mut builder = ProgramConfigBuilder::new(owner);

    // Domains
    let neutron_domain = valence_program_manager::domain::Domain::CosmosCosmwasm(
        "neutron".to_string()
    );
    let juno_domain = valence_program_manager::domain::Domain::CosmosCosmwasm(
        "juno".to_string()
    );

    // Accounts
    let juno_input_account = builder.add_account(
        AccountInfo::new("juno_input_account".to_string(), &juno_domain, AccountType::default())
    );
    let neutron_input_account = builder.add_account(
        AccountInfo::new("neutron_input_account".to_string(), &neutron_domain, AccountType::default())
    );
    let liquidity_position_account = builder.add_account(
        AccountInfo::new("liquidity_position_account".to_string(), &neutron_domain, AccountType::default())
    );
    let withdraw_output_account = builder.add_account(
        AccountInfo::new("withdraw_output_account".to_string(), &neutron_domain, AccountType::default())
    );


    let mut juno_to_neutron_pfm_map:BTreeMap<String, PacketForwardMiddlewareConfig> = BTreeMap::new();
    juno_to_neutron_pfm_map.insert(atom_on_juno.clone(),PacketForwardMiddlewareConfig {
        local_to_hop_chain_channel_id: juno_gaia_ibc_channel_id.to_string(),
        hop_to_destination_chain_channel_id: gaia_neutron_ibc_channel_id.to_string(),
        hop_chain_receiver_address: format!("|account_id|:{}", neutron_input_account.get_account_id()), // assign unknown address
     });

    // Libraries
    let juno_ibc_transfer_library = builder.add_library(
        LibraryInfo::new(
            "juno_ibc_transfer".to_string(),
            &juno_domain,
            LibraryConfig::ValenceGenericIbcTransferLibrary({
                valence_generic_ibc_transfer_library::msg::LibraryConfig {
                    input_addr: juno_input_account.clone(),
                    output_addr: neutron_input_account.clone(),
                    denom: valence_library_utils::denoms::UncheckedDenom::Native(atom_on_juno.to_string()),
                    amount: IbcTransferAmount::FullAmount,
                    remote_chain_info: RemoteChainInfo {
                        channel_id: juno_gaia_ibc_channel_id.to_string(),
                        ibc_transfer_timeout: Some(600u64.into()),
                    },
                    denom_to_pfm_map:  juno_to_neutron_pfm_map,
                    memo: "".to_owned(),
                }
            })
        )
    );

    let pool_type = PoolType::NativeLpToken(
        valence_astroport_utils::astroport_native_lp_token::PairType::Custom("concentrated".to_string())
    );

    let astroport_lper_library = builder.add_library(LibraryInfo::new(
        "astroport_lper".to_string(),
        &neutron_domain,
        LibraryConfig::ValenceAstroportLper(
            valence_astroport_lper::msg::LibraryConfig {
                input_addr: neutron_input_account.clone(),
                output_addr: liquidity_position_account.clone(),
                pool_addr: pool_addr.to_string(),
                lp_config:LiquidityProviderConfig {
                   pool_type: pool_type.clone(),
                    asset_data: AssetData {
                        asset1: ntrn_on_neutron.clone(),
                        asset2: atom_on_neutron.clone(),
                    },
                    max_spread: None,
                }
            }
        )
    ));

    let astroport_withdrawer_library = builder.add_library( LibraryInfo::new(
        "astroport_withdrawer".to_string(),
        &neutron_domain,
        LibraryConfig::ValenceAstroportWithdrawer(
            valence_astroport_withdrawer::msg::LibraryConfig {
                input_addr: liquidity_position_account.clone(),
                output_addr: withdraw_output_account.clone(),
                pool_addr: pool_addr.to_string(),
                withdrawer_config: LiquidityWithdrawerConfig {
                    pool_type: pool_type.clone(),
                    asset_data: AssetData {
                        asset1: ntrn_on_neutron.clone(),
                        asset2: atom_on_neutron.clone()
                    },
                },
            }
        )
    ));

    // links
    builder.add_link(&juno_ibc_transfer_library, vec![&juno_input_account], vec![&neutron_input_account]);
    builder.add_link(&astroport_lper_library, vec![&neutron_input_account], vec![&liquidity_position_account]);
    builder.add_link(&astroport_withdrawer_library, vec![&liquidity_position_account], vec![&withdraw_output_account]);

    // TODO: add subroutine for transfer.
    // TODO: move subroutines into variables

    // authorizations
    builder.add_authorization(
        AuthorizationBuilder::new()
            .with_label("provide_liquidity")
            .with_subroutine(
                AtomicSubroutineBuilder::new().with_function(AtomicFunctionBuilder::new()
                .with_contract_address(astroport_withdrawer_library.clone())
                .with_message_details(MessageDetails {
                    message_type: MessageType::CosmwasmExecuteMsg,
                    message: Message {
                        name: "process_function".to_string(),
                        params_restrictions: Some(
                            vec![
                                ParamRestriction::MustBeIncluded(
                                    vec![
                                        "process_function".to_string(),
                                        "provide_double_sided_liquidity".to_string()
                                    ]
                                )
                            ]
                        ),
                    },
                })
                .build()).build()
            )
            .build()
    );
    builder.add_authorization(
        AuthorizationBuilder::new()
            .with_mode(
                AuthorizationModeInfo::Permissioned(
                    PermissionTypeInfo::WithoutCallLimit(vec![permissioned_withdrawer])
                )
            )
            .with_label("withdraw_liquidity")
            .with_subroutine(
                AtomicSubroutineBuilder::new().with_function(AtomicFunctionBuilder::new()
                .with_contract_address(astroport_withdrawer_library.clone())
                .with_message_details(MessageDetails {
                    message_type: MessageType::CosmwasmExecuteMsg,
                    message: Message {
                        name: "process_function".to_string(),
                        params_restrictions: Some(
                            vec![
                                ParamRestriction::MustBeIncluded(
                                    vec!["process_function".to_string(), "withdraw_liquidity".to_string()]
                                )
                            ]
                        ),
                    },
                })
                .build()).build()
            )
            .build()
    );

    builder.build()
}