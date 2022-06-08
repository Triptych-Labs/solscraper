use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_client::GetConfirmedSignaturesForAddress2Config;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use std::io;
use std::str::FromStr;

async fn exhaust_transaction_signatures_for_address(
    network: String,
    address: String,
) -> Result<String, io::Error> {
    println!("{}", network);
    let rpc_client = RpcClient::new(network);
    println!("{:?}", rpc_client.get_recent_blockhash().await.unwrap());

    let mut exhausted = false;

    let mut latest_signature: Option<Signature> = None;
    let mut earliest_signature: Option<Signature> = None;

    let mut depth = 0;
    while !exhausted {
        let config = GetConfirmedSignaturesForAddress2Config {
            before: None,
            until: earliest_signature,
            limit: Some(1000),
            commitment: None,
        };

        let signatures = rpc_client
            .get_confirmed_signatures_for_address2_with_config(
                &Pubkey::from_str(address.as_str()).unwrap(),
                config,
            )
            .await
            .unwrap();
        if signatures.len() == 0 {
            break;
        }
        if depth == 0 {
            latest_signature =
                Some(Signature::from_str(&signatures[0].signature.clone().to_string()).unwrap());
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
        println!(
            "{:?} {:?} {:?} {}",
            signatures.len(),
            latest_signature,
            earliest_signature,
            &Pubkey::from_str(address.as_str()).unwrap()
        );

        depth += 1;
    }
    println!("{:?} {:?}", latest_signature, earliest_signature);

    Ok("".to_string())
}

pub async fn get_genesis_transaction(
    network: String,
    address: String,
) -> Result<String, io::Error> {
    exhaust_transaction_signatures_for_address(network, address).await
}
