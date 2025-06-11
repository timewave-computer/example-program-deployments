use valence_authorization_utils::{
    authorization_message::{Message, MessageDetails, MessageType, ParamRestriction},
    builders::{AtomicFunctionBuilder, AtomicSubroutineBuilder, AuthorizationBuilder},
};
use valence_library_utils::denoms::UncheckedDenom;
use valence_program_manager::{
    account::{AccountInfo, AccountType},
    library::{LibraryConfig, LibraryInfo},
    program_config::ProgramConfig,
    program_config_builder::ProgramConfigBuilder,
};
use valence_splitter_library::msg::{UncheckedSplitAmount, UncheckedSplitConfig};
use valence_forwarder_library::msg::UncheckedForwardingConfig;
use cosmwasm_std::Uint128;

/// Write your program using the program builder
pub fn program_builder(params: deployer_lib::ProgramParams) -> ProgramConfig {
    // program params
    let owner = params.get("owner");

    // Domains
    let neutron_domain = valence_program_manager::domain::Domain::CosmosCosmwasm("neutron".to_string());
    let osmosis_domain = valence_program_manager::domain::Domain::CosmosCosmwasm("osmosis".to_string());

    // Write your program
    let mut builder = ProgramConfigBuilder::new("osmosis_token_forwarder", &owner);

    let account_a = builder.add_account(AccountInfo::new(
        "account_A".to_string(),
        &osmosis_domain,
        AccountType::default(),
    ));
    let account_b = builder.add_account(AccountInfo::new(
        "account_B".to_string(),
        &osmosis_domain,
        AccountType::default(),
    ));
    
    let library_forwarder_a_to_b_config = valence_forwarder_library::msg::LibraryConfig {
        input_addr: account_a.clone(),
        output_addr: account_b.clone(),
        forwarding_configs: vec![
            UncheckedForwardingConfig {
                denom: UncheckedDenom::Native("uosmo".to_string()),
                max_amount: Uint128::new(1000000000),
            }
        ],
        forwarding_constraints: valence_forwarder_library::msg::ForwardingConstraints::default(),
    };

    let library_forwarder_a_to_b = builder.add_library(LibraryInfo::new(
        "forwader_A_to_B".to_string(),
        &osmosis_domain,
        LibraryConfig::ValenceForwarderLibrary(library_forwarder_a_to_b_config.clone()),
    ));

    builder.add_link(&library_forwarder_a_to_b, vec![&account_a], vec![&account_b]);

    // B to A forwarder
    let library_forwarder_b_to_a_config = valence_forwarder_library::msg::LibraryConfig {
        input_addr: account_b.clone(),
        output_addr: account_a.clone(),
        forwarding_configs: vec![
            UncheckedForwardingConfig {
                denom: UncheckedDenom::Native("uosmo".to_string()),
                max_amount: Uint128::new(1000000000),
            }
        ],
        forwarding_constraints: valence_forwarder_library::msg::ForwardingConstraints::default(),
    };

    let library_forwarder_b_to_a = builder.add_library(LibraryInfo::new(
        "forwarder_B_to_A".to_string(),
        &osmosis_domain,
        LibraryConfig::ValenceForwarderLibrary(library_forwarder_b_to_a_config.clone()),
    ));

    builder.add_link(&library_forwarder_b_to_a, vec![&account_b], vec![&account_a]);

    // A to B authorization
    let action_label_a_to_b = "A_to_B";
    let function_a_to_b = AtomicFunctionBuilder::new()
        .with_contract_address(library_forwarder_a_to_b.clone())
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

    let subroutine_a_to_b = AtomicSubroutineBuilder::new()
        .with_function(function_a_to_b)
        .build();
    let authorization_a_to_b = AuthorizationBuilder::new()
        .with_label(action_label_a_to_b)
        .with_subroutine(subroutine_a_to_b)
        .build();

    builder.add_authorization(authorization_a_to_b);

    // B to A authorization
    let action_label_b_to_a = "B_to_A";
    let function_b_to_a = AtomicFunctionBuilder::new()
        .with_contract_address(library_forwarder_b_to_a.clone())
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

    let subroutine_b_to_a = AtomicSubroutineBuilder::new()
        .with_function(function_b_to_a)
        .build();
    let authorization_b_to_a = AuthorizationBuilder::new()
        .with_label(action_label_b_to_a)
        .with_subroutine(subroutine_b_to_a)
        .build();

    builder.add_authorization(authorization_b_to_a);

    builder.build()
}
