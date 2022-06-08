use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_client::GetConfirmedSignaturesForAddress2Config;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_transaction_status::UiTransactionEncoding;
use std::io;
use std::str::FromStr;

async fn exhaust_transaction_signatures_for_genesis_signature(
    network: String,
    address: String,
) -> Result<Signature, io::Error> {
    println!("{}", network);
    let rpc_client = RpcClient::new(network);
    println!("{:?}", rpc_client.get_recent_blockhash().await.unwrap());

    let mut earliest_signature: Option<Signature> = None;

    loop {
        let config = GetConfirmedSignaturesForAddress2Config {
            before: None,
            until: earliest_signature,
            limit: Some(1000),
            commitment: None,
        };

        let signatures = rpc_client
            .get_signatures_for_address_with_config(
                &Pubkey::from_str(address.as_str()).unwrap(),
                config,
            )
            .await
            .unwrap();
        if signatures.len() == 0 {
            break;
        }
        earliest_signature = Some(
            Signature::from_str(
                &signatures[signatures.len() - 1]
                    .signature
                    .clone()
                    .to_string(),
            )
            .unwrap(),
        );
    }

    Ok(earliest_signature.unwrap())
}

pub async fn get_genesis_transaction(
    network: String,
    address: String,
) -> Result<String, io::Error> {
    let genesis_signature =
        exhaust_transaction_signatures_for_genesis_signature(network.clone(), address)
            .await
            .unwrap();

    let rpc_client = RpcClient::new(network.clone());
    let transaction_meta = rpc_client
        .get_transaction(&genesis_signature, UiTransactionEncoding::Base58)
        .await
        .unwrap();

    let transaction_meta_decode = transaction_meta.transaction.transaction.decode();

    let transaction_meta_decoded = transaction_meta_decode.unwrap();
    let transaction_decoded = transaction_meta_decoded.message;

    println!(
        "{:?}",
        transaction_decoded.static_account_keys()
            [transaction_decoded.instructions()[5].accounts[2] as usize]
    );

    Ok("".to_string())
}
