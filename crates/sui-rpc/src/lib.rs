pub mod client;
pub mod field;
pub mod headers;
pub mod merge;
pub mod proto;

pub use client::Client;

#[doc(hidden)]
mod _serde;

#[cfg(test)]
mod test {
    use prost_types::FieldMask;

    use crate::{
        field::FieldMaskUtil,
        proto::sui::rpc::v2beta2::{
            simulate_transaction_request::TransactionChecks, Argument, Command, Input, MoveCall,
            ProgrammableTransaction, SimulateTransactionRequest,
        },
    };

    use super::*;

    #[tokio::test]
    async fn bar() {
        //
        let mut client = Client::new("http://localhost:9000").unwrap();
        let resp = client
            .live_data_client()
            .simulate_transaction(SimulateTransactionRequest {
                transaction: Some(proto::sui::rpc::v2beta2::Transaction {
                    kind: Some(
                        ProgrammableTransaction {
                            inputs: vec![
                                Input {
                                object_id: Some("0x5".into()),
                                ..Default::default()
                            },
                                Input {
                                object_id: Some("0xc1abdfc817ae1b43cd281df7e3515b8fe0687a32b90f33ef4a449879526e0a54".into()),
                                ..Default::default()
                            }
                            ],
                            commands: vec![Command::from(MoveCall {
                                package: Some("0x3".to_owned()),
                                module: Some("sui_system".to_owned()),
                                function: Some("calculate_rewards".to_owned()),
                                type_arguments: vec![],
                                arguments: vec![Argument::new_input(0), Argument::new_input(1)],
                            })],
                        }
                        .into(),
                    ),
                    sender: Some(
                        "0x8a7ed918db7b9a4407e365125788ef24ead56261e0a4e915ff16c1155c382302".into(),
                    ),
                    ..Default::default()
                }),
                read_mask: Some(FieldMask::from_str("*")),
                checks: Some(TransactionChecks::Disabled as _),
                do_gas_selection: None,
            })
            .await;

        println!("{:#?}", resp);
    }
}
