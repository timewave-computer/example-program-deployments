use valence_astroport_lper::msg::LiquidityProviderConfig;
use valence_astroport_utils::PoolType;
use valence_astroport_withdrawer::msg::LiquidityWithdrawerConfig;
use valence_authorization_utils::{
    authorization::{ AuthorizationModeInfo, PermissionTypeInfo },
    authorization_message::{ Message, MessageDetails, MessageType, ParamRestriction },
    builders::{ AtomicFunctionBuilder, AtomicSubroutineBuilder, AuthorizationBuilder },
};
use valence_library_utils::liquidity_utils::AssetData;
use valence_program_manager::{
    account::{ AccountInfo, AccountType },
    library::{ LibraryConfig, LibraryInfo },
    program_config::ProgramConfig,
    program_config_builder::ProgramConfigBuilder,
};

/// Write your program using the program builder
pub fn program_builder(params: deployer_lib::ProgramParams) -> ProgramConfig {
    // Read program params
    let owner = params.get("owner");
    let pool_addr = params.get("pool_addr");
    let ntrn_denom = params.get("ntrn_denom");
    let atom_denom = params.get("atom_denom");
    let permissioned_withdrawer = params.get("permissioned_withdrawer");

    // Initialize the program builder
    let mut builder = ProgramConfigBuilder::new(owner);

    // Domains
    let neutron_domain = valence_program_manager::domain::Domain::CosmosCosmwasm(
        "neutron".to_string()
    );

    // Accounts
    let input_account = builder.add_account(
        AccountInfo::new("input_account".to_string(), &neutron_domain, AccountType::default())
    );
    let liquidity_position_account = builder.add_account(
        AccountInfo::new(
            "liquidity_position_account".to_string(),
            &neutron_domain,
            AccountType::default()
        )
    );
    let output_account = builder.add_account(
        AccountInfo::new("output_account".to_string(), &neutron_domain, AccountType::default())
    );

    // Libraries
    let pool_type = PoolType::NativeLpToken(
        valence_astroport_utils::astroport_native_lp_token::PairType::Custom(
            "concentrated".to_string()
        )
    );
    let liquidity_provider_library = builder.add_library(
        LibraryInfo::new(
            "deploy_liquidity".to_string(),
            &neutron_domain,
            LibraryConfig::ValenceAstroportLper(valence_astroport_lper::msg::LibraryConfig {
                input_addr: input_account.clone(),
                output_addr: liquidity_position_account.clone(),
                pool_addr: pool_addr.to_string(),
                lp_config: LiquidityProviderConfig {
                    pool_type: pool_type.clone(),
                    asset_data: AssetData {
                        asset1: ntrn_denom.clone(),
                        asset2: atom_denom.clone(),
                    },
                    max_spread: None,
                },
            })
        )
    );

    let liquidity_withdrawer_library = builder.add_library(
        LibraryInfo::new(
            "withdraw_liquidity_position".to_string(),
            &neutron_domain,
            LibraryConfig::ValenceAstroportWithdrawer(
                valence_astroport_withdrawer::msg::LibraryConfig {
                    input_addr: liquidity_position_account.clone(),
                    output_addr: output_account.clone(),
                    pool_addr: pool_addr.to_string(),
                    withdrawer_config: LiquidityWithdrawerConfig {
                        pool_type: pool_type.clone(),
                        asset_data: AssetData {
                            asset1: ntrn_denom.clone(),
                            asset2: atom_denom.clone(),
                        },
                    },
                }
            )
        )
    );

    // Links
    builder.add_link(
        &liquidity_provider_library,
        vec![&input_account],
        vec![&liquidity_position_account]
    );
    builder.add_link(
        &liquidity_withdrawer_library,
        vec![&liquidity_position_account],
        vec![&output_account]
    );

    // Authorizations
    let provide_liqudity_function = AtomicFunctionBuilder::new()
        .with_contract_address(liquidity_provider_library.clone())
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
        .build();

    builder.add_authorization(
        AuthorizationBuilder::new()
            .with_label("provide_liquidity")
            .with_subroutine(
                AtomicSubroutineBuilder::new().with_function(provide_liqudity_function).build()
            )
            .build()
    );

    let withdraw_liqudity_function = AtomicFunctionBuilder::new()
        .with_contract_address(liquidity_withdrawer_library.clone())
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
        .build();

    builder.add_authorization(
        AuthorizationBuilder::new()
            .with_mode(
                AuthorizationModeInfo::Permissioned(
                    PermissionTypeInfo::WithoutCallLimit(vec![permissioned_withdrawer])
                )
            )
            .with_label("withdraw_liquidity")
            .with_subroutine(
                AtomicSubroutineBuilder::new().with_function(withdraw_liqudity_function).build()
            )
            .build()
    );

    builder.build()
}
