use valence_authorization_utils::{
    authorization_message::{Message, MessageDetails, MessageType, ParamRestriction},
    builders::{AtomicFunctionBuilder, AtomicSubroutineBuilder, AuthorizationBuilder},
    domain::Domain,
};
use valence_library_utils::denoms::UncheckedDenom;
use valence_program_manager::{
    account::{AccountInfo, AccountType},
    library::{LibraryConfig, LibraryInfo},
    program_config::ProgramConfig,
    program_config_builder::ProgramConfigBuilder,
};
use valence_forwarder_library::msg::UncheckedForwardingConfig;
use valence_neutron_ibc_transfer_library::msg::LibraryConfig as NeutronIbcConfig;
use valence_generic_ibc_transfer_library::msg::LibraryConfig as GenericIbcConfig;
use valence_osmosis_cl_lper::msg::LibraryConfig as OsmosisClLperConfig;
use valence_osmosis_cl_withdrawer::msg::LibraryConfig as OsmosisClWithdrawerConfig;
use valence_osmosis_utils::utils::cl_utils::TickRange;
use cosmwasm_std::{Uint128, Uint64, Int64};
use std::str::FromStr;
use std::collections::BTreeMap;
use valence_ibc_utils::types::PacketForwardMiddlewareConfig;

/// Write your program using the program builder
pub fn program_builder(params: deployer_lib::ProgramParams) -> ProgramConfig {
    // program params
    let owner = params.get("owner");
    let neutron_stride_channel = params.get("neutron_stride_channel");
    let stride_osmosis_channel = params.get("stride_osmosis_channel");
    let osmosis_stride_channel = params.get("osmosis_stride_channel");
    let stride_neutron_channel = params.get("stride_neutron_channel");
    let neutron_cosmos_channel = params.get("neutron_cosmos_channel");
    let cosmos_osmosis_channel = params.get("cosmos_osmosis_channel");
    let osmosis_cosmos_channel = params.get("osmosis_cosmos_channel");
    let cosmos_neutron_channel = params.get("cosmos_neutron_channel");
    let ibc_timeout_seconds = params.get("ibc_timeout_seconds");
    let pool_id = params.get("pool_id");
    let lower_tick = params.get("lower_tick");
    let upper_tick = params.get("upper_tick");
    let neutron_atom_denom = params.get("neutron_atom_denom");
    let neutron_statom_denom = params.get("neutron_statom_denom");
    let osmosis_atom_denom = params.get("osmosis_atom_denom");
    let osmosis_statom_denom = params.get("osmosis_statom_denom");
    let max_forward_amount = params.get("max_forward_amount");

    // Domains
    let neutron_domain = valence_program_manager::domain::Domain::CosmosCosmwasm("neutron".to_string());
    let osmosis_domain = valence_program_manager::domain::Domain::CosmosCosmwasm("osmosis".to_string());

    // Write your program
    let mut builder = ProgramConfigBuilder::new("neutron_to_osmosis_crosschain_lper", &owner);

    // Neutron Accounts
    let account_n1 = builder.add_account(AccountInfo::new(
        "account_N1".to_string(),
        &neutron_domain,
        AccountType::default(),
    ));
    let account_n2 = builder.add_account(AccountInfo::new(
        "account_N2".to_string(),
        &neutron_domain,
        AccountType::default(),
    ));

    // Osmosis Accounts
    let account_o1 = builder.add_account(AccountInfo::new(
        "account_O1".to_string(),
        &osmosis_domain,
        AccountType::default(),
    ));
    let account_o2 = builder.add_account(AccountInfo::new(
        "account_O2".to_string(),
        &osmosis_domain,
        AccountType::default(),
    ));
    let account_o3 = builder.add_account(AccountInfo::new(
        "account_O3".to_string(),
        &osmosis_domain,
        AccountType::default(),
    ));

    // Create denom-to-PFM mappings for Neutron -> Osmosis via Stride (stATOM)
    let mut neutron_to_osmosis_statom_pfm_map = BTreeMap::new();
    neutron_to_osmosis_statom_pfm_map.insert(
        neutron_statom_denom.clone(),
        PacketForwardMiddlewareConfig {
            local_to_hop_chain_channel_id: neutron_stride_channel.clone(),
            hop_to_destination_chain_channel_id: stride_osmosis_channel.clone(),
            hop_chain_receiver_address: "stride1qxatg2nkmsf26cymcg2saeh9l2cqp0s2p0eqgx".to_string(),
        }
    );

    // Create denom-to-PFM mappings for Neutron -> Osmosis via Cosmos Hub (ATOM)
    let mut neutron_to_osmosis_atom_pfm_map = BTreeMap::new();
    neutron_to_osmosis_atom_pfm_map.insert(
        neutron_atom_denom.clone(),
        PacketForwardMiddlewareConfig {
            local_to_hop_chain_channel_id: neutron_cosmos_channel.clone(),
            hop_to_destination_chain_channel_id: cosmos_osmosis_channel.clone(),
            hop_chain_receiver_address: "pfm".to_string(),
        }
    );

    // Neutron IBC Transfer Library for ATOM (N1 -> O1) via Cosmos Hub
    let neutron_ibc_atom_config = NeutronIbcConfig {
        input_addr: account_n1.clone(),
        output_addr: account_o1.clone(),
        denom: UncheckedDenom::Native(neutron_atom_denom.clone()),
        amount: valence_neutron_ibc_transfer_library::msg::IbcTransferAmount::FullAmount,
        memo: "Transfer ATOM from Neutron to Osmosis via Cosmos Hub".to_string(),
        remote_chain_info: valence_neutron_ibc_transfer_library::msg::RemoteChainInfo {
            channel_id: neutron_cosmos_channel.clone(),
            ibc_transfer_timeout: Some(Uint64::from(ibc_timeout_seconds.parse::<u64>().unwrap_or(3600))),
        },
        denom_to_pfm_map: neutron_to_osmosis_atom_pfm_map,
    };

    let neutron_ibc_atom_transfer = builder.add_library(LibraryInfo::new(
        "neutron_ibc_atom_transfer".to_string(),
        &neutron_domain,
        LibraryConfig::ValenceNeutronIbcTransferLibrary(neutron_ibc_atom_config.clone()),
    ));

    builder.add_link(&neutron_ibc_atom_transfer, vec![&account_n1], vec![&account_o1]);

    // Neutron IBC Transfer Library for stATOM (N1 -> O1) via Stride
    let neutron_ibc_statom_config = NeutronIbcConfig {
        input_addr: account_n1.clone(),
        output_addr: account_o1.clone(),
        denom: UncheckedDenom::Native(neutron_statom_denom.clone()),
        amount: valence_neutron_ibc_transfer_library::msg::IbcTransferAmount::FullAmount,
        memo: "Transfer stATOM from Neutron to Osmosis via Stride".to_string(),
        remote_chain_info: valence_neutron_ibc_transfer_library::msg::RemoteChainInfo {
            channel_id: neutron_stride_channel.clone(),
            ibc_transfer_timeout: Some(Uint64::from(ibc_timeout_seconds.parse::<u64>().unwrap_or(3600))),
        },
        denom_to_pfm_map: neutron_to_osmosis_statom_pfm_map,
    };

    let neutron_ibc_statom_transfer = builder.add_library(LibraryInfo::new(
        "neutron_ibc_statom_transfer".to_string(),
        &neutron_domain,
        LibraryConfig::ValenceNeutronIbcTransferLibrary(neutron_ibc_statom_config.clone()),
    ));

    builder.add_link(&neutron_ibc_statom_transfer, vec![&account_n1], vec![&account_o1]);

    // Osmosis CL LPer Library (O1 -> O2)
    let osmosis_cl_lper_config = OsmosisClLperConfig {
        input_addr: account_o1.clone(),
        output_addr: account_o2.clone(),
        lp_config: valence_osmosis_cl_lper::msg::LiquidityProviderConfig {
            pool_id: Uint64::from(pool_id.parse::<u64>().unwrap_or(1)),
            pool_asset_1: osmosis_atom_denom.clone(),
            pool_asset_2: osmosis_statom_denom.clone(),
            global_tick_range: TickRange {
                lower_tick: Int64::from(lower_tick.parse::<i64>().unwrap_or(-100000)),
                upper_tick: Int64::from(upper_tick.parse::<i64>().unwrap_or(100000)),
            },
        },
    };

    let osmosis_cl_lper = builder.add_library(LibraryInfo::new(
        "osmosis_cl_lper".to_string(),
        &osmosis_domain,
        LibraryConfig::ValenceOsmosisClLper(osmosis_cl_lper_config.clone()),
    ));

    builder.add_link(&osmosis_cl_lper, vec![&account_o1], vec![&account_o2]);

    // Osmosis CL Withdrawer Library (O2 -> O3)
    let osmosis_cl_withdrawer_config = OsmosisClWithdrawerConfig {
        input_addr: account_o2.clone(),
        output_addr: account_o3.clone(),
        pool_id: Uint64::from(pool_id.parse::<u64>().unwrap_or(1)),
    };

    let osmosis_cl_withdrawer = builder.add_library(LibraryInfo::new(
        "osmosis_cl_withdrawer".to_string(),
        &osmosis_domain,
        LibraryConfig::ValenceOsmosisClWithdrawer(osmosis_cl_withdrawer_config.clone()),
    ));

    builder.add_link(&osmosis_cl_withdrawer, vec![&account_o2], vec![&account_o3]);

    // Create denom-to-PFM mappings for Osmosis -> Neutron via Stride (stATOM)
    let mut osmosis_to_neutron_statom_pfm_map = BTreeMap::new();
    osmosis_to_neutron_statom_pfm_map.insert(
        osmosis_statom_denom.clone(),
        PacketForwardMiddlewareConfig {
            local_to_hop_chain_channel_id: osmosis_stride_channel.clone(),
            hop_to_destination_chain_channel_id: stride_neutron_channel.clone(),
            hop_chain_receiver_address: "stride1qxatg2nkmsf26cymcg2saeh9l2cqp0s2p0eqgx".to_string(),
        }
    );

    // Create denom-to-PFM mappings for Osmosis -> Neutron via Cosmos Hub (ATOM)
    let mut osmosis_to_neutron_atom_pfm_map = BTreeMap::new();
    osmosis_to_neutron_atom_pfm_map.insert(
        osmosis_atom_denom.clone(),
        PacketForwardMiddlewareConfig {
            local_to_hop_chain_channel_id: osmosis_cosmos_channel.clone(),
            hop_to_destination_chain_channel_id: cosmos_neutron_channel.clone(),
            hop_chain_receiver_address: "pfm".to_string(),
        }
    );

    // Generic IBC Transfer Library for ATOM (O3 -> N2) via Cosmos Hub
    let generic_ibc_atom_config = GenericIbcConfig {
        input_addr: account_o3.clone(),
        output_addr: account_n2.clone(),
        denom: UncheckedDenom::Native(osmosis_atom_denom.clone()),
        amount: valence_generic_ibc_transfer_library::msg::IbcTransferAmount::FullAmount,
        memo: "Transfer ATOM from Osmosis to Neutron via Cosmos Hub".to_string(),
        remote_chain_info: valence_generic_ibc_transfer_library::msg::RemoteChainInfo {
            channel_id: osmosis_cosmos_channel.clone(),
            ibc_transfer_timeout: Some(Uint64::from(ibc_timeout_seconds.parse::<u64>().unwrap_or(3600))),
        },
        denom_to_pfm_map: osmosis_to_neutron_atom_pfm_map,
    };

    let generic_ibc_atom_transfer = builder.add_library(LibraryInfo::new(
        "generic_ibc_atom_transfer".to_string(),
        &osmosis_domain,
        LibraryConfig::ValenceGenericIbcTransferLibrary(generic_ibc_atom_config.clone()),
    ));

    builder.add_link(&generic_ibc_atom_transfer, vec![&account_o3], vec![&account_n2]);

    // Generic IBC Transfer Library for stATOM (O3 -> N2) via Stride
    let generic_ibc_statom_config = GenericIbcConfig {
        input_addr: account_o3.clone(),
        output_addr: account_n2.clone(),
        denom: UncheckedDenom::Native(osmosis_statom_denom.clone()),
        amount: valence_generic_ibc_transfer_library::msg::IbcTransferAmount::FullAmount,
        memo: "Transfer stATOM from Osmosis to Neutron via Stride".to_string(),
        remote_chain_info: valence_generic_ibc_transfer_library::msg::RemoteChainInfo {
            channel_id: osmosis_stride_channel.clone(),
            ibc_transfer_timeout: Some(Uint64::from(ibc_timeout_seconds.parse::<u64>().unwrap_or(3600))),
        },
        denom_to_pfm_map: osmosis_to_neutron_statom_pfm_map,
    };

    let generic_ibc_statom_transfer = builder.add_library(LibraryInfo::new(
        "generic_ibc_statom_transfer".to_string(),
        &osmosis_domain,
        LibraryConfig::ValenceGenericIbcTransferLibrary(generic_ibc_statom_config.clone()),
    ));

    builder.add_link(&generic_ibc_statom_transfer, vec![&account_o3], vec![&account_n2]);

    // Forwarder Library (N2 -> N1) for demo purposes
    let forwarder_config = valence_forwarder_library::msg::LibraryConfig {
        input_addr: account_n2.clone(),
        output_addr: account_n1.clone(),
        forwarding_configs: vec![
            UncheckedForwardingConfig {
                denom: UncheckedDenom::Native(neutron_atom_denom.clone()),
                max_amount: Uint128::from_str(&max_forward_amount).unwrap_or_default(),
            },
            UncheckedForwardingConfig {
                denom: UncheckedDenom::Native(neutron_statom_denom.clone()),
                max_amount: Uint128::from_str(&max_forward_amount).unwrap_or_default(),
            }
        ],
        forwarding_constraints: valence_forwarder_library::msg::ForwardingConstraints::default(),
    };

    let forwarder = builder.add_library(LibraryInfo::new(
        "forwarder".to_string(),
        &neutron_domain,
        LibraryConfig::ValenceForwarderLibrary(forwarder_config.clone()),
    ));

    builder.add_link(&forwarder, vec![&account_n2], vec![&account_n1]);

    // Authorizations

    // 1. IBC Transfer from Neutron to Osmosis via Cosmos Hub (ATOM)
    let neutron_to_osmosis_atom_auth = AtomicFunctionBuilder::new()
        .with_contract_address(neutron_ibc_atom_transfer.clone())
        .with_message_details(MessageDetails {
            message_type: MessageType::CosmwasmExecuteMsg,
            message: Message {
                name: "process_function".to_string(),
                params_restrictions: Some(vec![ParamRestriction::MustBeIncluded(vec![
                    "process_function".to_string(),
                    "ibc_transfer".to_string(),
                ])]),
            },
        })
        .build();

    let neutron_to_osmosis_atom_subroutine = AtomicSubroutineBuilder::new()
        .with_function(neutron_to_osmosis_atom_auth)
        .build();
    let neutron_to_osmosis_atom_authorization = AuthorizationBuilder::new()
        .with_label("IBC_Transfer_Neutron_to_Osmosis_ATOM")
        .with_subroutine(neutron_to_osmosis_atom_subroutine)
        .build();

    builder.add_authorization(neutron_to_osmosis_atom_authorization);

    // 1b. IBC Transfer from Neutron to Osmosis via Stride (stATOM)
    let neutron_to_osmosis_statom_auth = AtomicFunctionBuilder::new()
        .with_contract_address(neutron_ibc_statom_transfer.clone())
        .with_message_details(MessageDetails {
            message_type: MessageType::CosmwasmExecuteMsg,
            message: Message {
                name: "process_function".to_string(),
                params_restrictions: Some(vec![ParamRestriction::MustBeIncluded(vec![
                    "process_function".to_string(),
                    "ibc_transfer".to_string(),
                ])]),
            },
        })
        .build();

    let neutron_to_osmosis_statom_subroutine = AtomicSubroutineBuilder::new()
        .with_function(neutron_to_osmosis_statom_auth)
        .build();
    let neutron_to_osmosis_statom_authorization = AuthorizationBuilder::new()
        .with_label("IBC_Transfer_Neutron_to_Osmosis_STATOM")
        .with_subroutine(neutron_to_osmosis_statom_subroutine)
        .build();

    builder.add_authorization(neutron_to_osmosis_statom_authorization);

    // 2. Provide Liquidity
    let provide_liquidity_auth = AtomicFunctionBuilder::new()
        .with_contract_address(osmosis_cl_lper.clone())
        .with_domain(Domain::External("osmosis".to_string()))
        .with_message_details(MessageDetails {
            message_type: MessageType::CosmwasmExecuteMsg,
            message: Message {
                name: "process_function".to_string(),
                params_restrictions: Some(vec![ParamRestriction::MustBeIncluded(vec![
                    "process_function".to_string(),
                    "provide_liquidity_default".to_string(),
                ])]),
            },
        })
        .build();

    let provide_liquidity_subroutine = AtomicSubroutineBuilder::new()
        .with_function(provide_liquidity_auth)
        .build();
    let provide_liquidity_authorization = AuthorizationBuilder::new()
        .with_label("Provide_Liquidity")
        .with_subroutine(provide_liquidity_subroutine)
        .build();

    builder.add_authorization(provide_liquidity_authorization);

    // 3. Withdraw Liquidity
    let withdraw_liquidity_auth = AtomicFunctionBuilder::new()
        .with_contract_address(osmosis_cl_withdrawer.clone())
        .with_domain(Domain::External("osmosis".to_string()))
        .with_message_details(MessageDetails {
            message_type: MessageType::CosmwasmExecuteMsg,
            message: Message {
                name: "process_function".to_string(),
                params_restrictions: Some(vec![ParamRestriction::MustBeIncluded(vec![
                    "process_function".to_string(),
                    "withdraw_liquidity".to_string(),
                ])]),
            },
        })
        .build();

    let withdraw_liquidity_subroutine = AtomicSubroutineBuilder::new()
        .with_function(withdraw_liquidity_auth)
        .build();
    let withdraw_liquidity_authorization = AuthorizationBuilder::new()
        .with_label("Withdraw_Liquidity")
        .with_subroutine(withdraw_liquidity_subroutine)
        .build();

    builder.add_authorization(withdraw_liquidity_authorization);

    // 4. IBC Transfer from Osmosis to Neutron via Cosmos Hub (ATOM)
    let osmosis_to_neutron_atom_auth = AtomicFunctionBuilder::new()
        .with_contract_address(generic_ibc_atom_transfer.clone())
        .with_domain(Domain::External("osmosis".to_string()))
        .with_message_details(MessageDetails {
            message_type: MessageType::CosmwasmExecuteMsg,
            message: Message {
                name: "process_function".to_string(),
                params_restrictions: Some(vec![ParamRestriction::MustBeIncluded(vec![
                    "process_function".to_string(),
                    "ibc_transfer".to_string(),
                ])]),
            },
        })
        .build();

    let osmosis_to_neutron_atom_subroutine = AtomicSubroutineBuilder::new()
        .with_function(osmosis_to_neutron_atom_auth)
        .build();
    let osmosis_to_neutron_atom_authorization = AuthorizationBuilder::new()
        .with_label("IBC_Transfer_Osmosis_to_Neutron_ATOM")
        .with_subroutine(osmosis_to_neutron_atom_subroutine)
        .build();

    builder.add_authorization(osmosis_to_neutron_atom_authorization);

    // 5. Forward Tokens (N2 -> N1)
    let forward_auth = AtomicFunctionBuilder::new()
        .with_contract_address(forwarder.clone())
        .with_message_details(MessageDetails {
            message_type: MessageType::CosmwasmExecuteMsg,
            message: Message {
                name: "process_function".to_string(),
                params_restrictions: Some(vec![ParamRestriction::MustBeIncluded(vec![
                    "process_function".to_string(),
                    "forward".to_string(),
                ])]),
            },
        })
        .build();

    let forward_subroutine = AtomicSubroutineBuilder::new()
        .with_function(forward_auth)
        .build();
    let forward_authorization = AuthorizationBuilder::new()
        .with_label("Forward_Tokens")
        .with_subroutine(forward_subroutine)
        .build();

    builder.add_authorization(forward_authorization);

    // 6. IBC Transfer from Osmosis to Neutron via Stride (stATOM)
    let osmosis_to_neutron_statom_auth = AtomicFunctionBuilder::new()
        .with_contract_address(generic_ibc_statom_transfer.clone())
        .with_domain(Domain::External("osmosis".to_string()))
        .with_message_details(MessageDetails {
            message_type: MessageType::CosmwasmExecuteMsg,
            message: Message {
                name: "process_function".to_string(),
                params_restrictions: Some(vec![ParamRestriction::MustBeIncluded(vec![
                    "process_function".to_string(),
                    "ibc_transfer".to_string(),
                ])]),
            },
        })
        .build();

    let osmosis_to_neutron_statom_subroutine = AtomicSubroutineBuilder::new()
        .with_function(osmosis_to_neutron_statom_auth)
        .build();
    let osmosis_to_neutron_statom_authorization = AuthorizationBuilder::new()
        .with_label("IBC_Transfer_Osmosis_to_Neutron_STATOM")
        .with_subroutine(osmosis_to_neutron_statom_subroutine)
        .build();

    builder.add_authorization(osmosis_to_neutron_statom_authorization);

    builder.build()
}
