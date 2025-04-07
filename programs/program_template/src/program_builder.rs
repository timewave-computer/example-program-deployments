<<<<<<< HEAD
=======
use std::str::FromStr;

use cosmwasm_std::Uint128;
>>>>>>> template/main
use valence_authorization_utils::{
    authorization_message::{Message, MessageDetails, MessageType, ParamRestriction},
    builders::{AtomicFunctionBuilder, AtomicSubroutineBuilder, AuthorizationBuilder},
};
<<<<<<< HEAD
=======
use valence_forwarder_library::msg::ForwardingConstraints;
>>>>>>> template/main
use valence_library_utils::denoms::UncheckedDenom;
use valence_program_manager::{
    account::{AccountInfo, AccountType},
    library::{LibraryConfig, LibraryInfo},
    program_config::ProgramConfig,
    program_config_builder::ProgramConfigBuilder,
};
<<<<<<< HEAD
use valence_splitter_library::msg::{UncheckedSplitAmount, UncheckedSplitConfig};

/// Write your program using the program builder
pub fn program_builder(params: deployer_lib::ProgramParams) -> ProgramConfig {
    // program params
    let owner = params.get("owner");

    // Domains
    let neutron_domain = valence_program_manager::domain::Domain::CosmosCosmwasm("neutron".to_string());

    // Write your program
    let swap_amount: u128 = 1_000_000_000;

    let mut builder = ProgramConfigBuilder::new(owner);

    let account_1 = builder.add_account(AccountInfo::new(
        "test_1".to_string(),
        &neutron_domain,
        AccountType::default(),
    ));
    let account_2 = builder.add_account(AccountInfo::new(
        "test_2".to_string(),
        &neutron_domain,
        AccountType::default(),
    ));
    
    let library_config = valence_splitter_library::msg::LibraryConfig {
        input_addr: account_1.clone(),
        splits: vec![UncheckedSplitConfig {
            denom: UncheckedDenom::Native("untrn".to_string()),
            account: account_2.clone(),
            amount: UncheckedSplitAmount::FixedAmount(swap_amount.into()),
        }],
    };

    let library_1 = builder.add_library(LibraryInfo::new(
        "test_splitter".to_string(),
        &neutron_domain,
        LibraryConfig::ValenceSplitterLibrary(library_config.clone()),
    ));

    builder.add_link(&library_1, vec![&account_1], vec![&account_2]);

    let action_label = "swap";

    let function = AtomicFunctionBuilder::new()
        .with_contract_address(library_1.clone())
=======

/// Write your program using the program builder
pub fn program_builder(params: deployer_lib::ProgramParams) -> ProgramConfig {
    //---- program params ----//
    // Owner of the program
    let owner = params.get("owner");
    // Denom to use for forwarding
    let denom = params.get("denom");
    // Max amount to forward from first account to second account
    let max_first_forward_amount = params.get("max_first_forward_amount");
    // Max amount to forward from second account to first account
    let max_second_forward_amount = params.get("max_second_forward_amount");
    // Authorized address that can change the forwarders config
    let authorized_addr = params.get("authorized_addr");

    //---- Set builder ----//
    let mut builder = ProgramConfigBuilder::new("example-program", owner.as_str());

    //---- Domains ----//
    // Neutron domain
    let neutron_domain =
        valence_program_manager::domain::Domain::CosmosCosmwasm("neutron".to_string());

    //---- Accounts ----//
    // First account
    let acc_first = builder.add_account(AccountInfo::new(
        "first_account".to_string(),
        &neutron_domain,
        AccountType::default(),
    ));
    // Second account
    let acc_second = builder.add_account(AccountInfo::new(
        "second_account".to_string(),
        &neutron_domain,
        AccountType::default(),
    ));

    //---- Libraries ----//
    // Forward funds from first account to second account
    let first_forwarder_config = valence_forwarder_library::msg::LibraryConfig {
        input_addr: acc_first.clone(),
        output_addr: acc_second.clone(),
        forwarding_configs: vec![valence_forwarder_library::msg::UncheckedForwardingConfig {
            denom: UncheckedDenom::Native(denom.clone()),
            max_amount: Uint128::from_str(max_first_forward_amount.as_str()).unwrap(),
        }],
        forwarding_constraints: ForwardingConstraints::new(None),
    };

    let lib_first_forwarder = builder.add_library(LibraryInfo::new(
        "first_forwarder".to_string(),
        &neutron_domain,
        LibraryConfig::ValenceForwarderLibrary(first_forwarder_config),
    ));

    builder.add_link(&lib_first_forwarder, vec![&acc_first], vec![&acc_second]);

    // Forward funds from second account to first account
    let second_forwarder_config = valence_forwarder_library::msg::LibraryConfig {
        input_addr: acc_second.clone(),
        output_addr: acc_first.clone(),
        forwarding_configs: vec![valence_forwarder_library::msg::UncheckedForwardingConfig {
            denom: UncheckedDenom::Native(denom),
            max_amount: Uint128::from_str(max_second_forward_amount.as_str()).unwrap(),
        }],
        forwarding_constraints: ForwardingConstraints::new(None),
    };

    let lib_second_forwarder = builder.add_library(LibraryInfo::new(
        "second_forwarder".to_string(),
        &neutron_domain,
        LibraryConfig::ValenceForwarderLibrary(second_forwarder_config),
    ));

    builder.add_link(&lib_second_forwarder, vec![&acc_second], vec![&acc_first]);

    //---- Authorizations ----//
    // First authorization to forward funds from the first account to the second account
    let function = AtomicFunctionBuilder::new()
        .with_contract_address(lib_first_forwarder.clone())
>>>>>>> template/main
        .with_message_details(MessageDetails {
            message_type: MessageType::CosmwasmExecuteMsg,
            message: Message {
                name: "process_function".to_string(),
                params_restrictions: Some(vec![ParamRestriction::MustBeIncluded(vec![
                    "process_function".to_string(),
<<<<<<< HEAD
                    "split".to_string(),
=======
                    "forward".to_string(),
>>>>>>> template/main
                ])]),
            },
        })
        .build();

    let subroutine = AtomicSubroutineBuilder::new()
        .with_function(function)
        .build();
    let authorization = AuthorizationBuilder::new()
<<<<<<< HEAD
        .with_label(action_label)
=======
        .with_label("Forward_from_first_to_second")
>>>>>>> template/main
        .with_subroutine(subroutine)
        .build();

    builder.add_authorization(authorization);

<<<<<<< HEAD
=======
    // Second authorization to forward funds from the second account to the first account
    let function = AtomicFunctionBuilder::new()
        .with_contract_address(lib_second_forwarder.clone())
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

    let subroutine = AtomicSubroutineBuilder::new()
        .with_function(function)
        .build();
    let authorization = AuthorizationBuilder::new()
        .with_label("Forward_from second_to_first")
        .with_subroutine(subroutine)
        .build();

    builder.add_authorization(authorization);

    // Update first forwarder to change the amount that can be sent
    let update_first_forward_config_function = AtomicFunctionBuilder::new()
        .with_contract_address(lib_first_forwarder.clone())
        .with_message_details(MessageDetails {
            message_type: MessageType::CosmwasmExecuteMsg,
            message: Message {
                name: "update_config".to_string(),
                params_restrictions: Some(vec![ParamRestriction::MustBeIncluded(vec![
                    "update_config".to_string(),
                    "new_config".to_string(),
                ])]),
            },
        })
        .build();

    let subroutine = AtomicSubroutineBuilder::new()
        .with_function(update_first_forward_config_function)
        .build();
    let authorization = AuthorizationBuilder::new()
        .with_label("Secure_update_first_forwarder_config")
        .with_mode(
            valence_authorization_utils::authorization::AuthorizationModeInfo::Permissioned(
                valence_authorization_utils::authorization::PermissionTypeInfo::WithoutCallLimit(
                    vec![authorized_addr.clone()],
                ),
            ),
        )
        .with_subroutine(subroutine)
        .build();

    builder.add_authorization(authorization);

    // Update second forwarder to change the amount that can be sent
    let update_second_forward_config_function = AtomicFunctionBuilder::new()
        .with_contract_address(lib_second_forwarder.clone())
        .with_message_details(MessageDetails {
            message_type: MessageType::CosmwasmExecuteMsg,
            message: Message {
                name: "update_config".to_string(),
                params_restrictions: Some(vec![ParamRestriction::MustBeIncluded(vec![
                    "update_config".to_string(),
                    "new_config".to_string(),
                ])]),
            },
        })
        .build();

    let subroutine = AtomicSubroutineBuilder::new()
        .with_function(update_second_forward_config_function)
        .build();
    let authorization = AuthorizationBuilder::new()
        .with_label("Secure_update_second_forwarder_config")
        .with_mode(
            valence_authorization_utils::authorization::AuthorizationModeInfo::Permissioned(
                valence_authorization_utils::authorization::PermissionTypeInfo::WithoutCallLimit(
                    vec![authorized_addr.clone()],
                ),
            ),
        )
        .with_subroutine(subroutine)
        .build();

    builder.add_authorization(authorization);
    
    // Build program config
>>>>>>> template/main
    builder.build()
}
