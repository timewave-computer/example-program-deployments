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

/// Write your program using the program builder
pub fn program_builder(params: deployer_lib::ProgramParams) -> ProgramConfig {
    // program params
    let owner = params.get("owner");

    // Domains
    let neutron_domain = valence_program_manager::domain::Domain::CosmosCosmwasm("neutron".to_string());

    // Write your program
    let swap_amount: u128 = 1_000_000_000;

    let mut builder = ProgramConfigBuilder::new("test_program", &owner);

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
        .with_message_details(MessageDetails {
            message_type: MessageType::CosmwasmExecuteMsg,
            message: Message {
                name: "process_function".to_string(),
                params_restrictions: Some(vec![ParamRestriction::MustBeIncluded(vec![
                    "process_function".to_string(),
                    "split".to_string(),
                ])]),
            },
        })
        .build();

    let subroutine = AtomicSubroutineBuilder::new()
        .with_function(function)
        .build();
    let authorization = AuthorizationBuilder::new()
        .with_label(action_label)
        .with_subroutine(subroutine)
        .build();

    builder.add_authorization(authorization);

    builder.build()
}
