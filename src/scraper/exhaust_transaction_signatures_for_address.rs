use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_client::GetConfirmedSignaturesForAddress2Config;
use solana_sdk::program_utils::limited_deserialize;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_sdk::system_instruction::SystemInstruction;
use solana_transaction_status::{EncodableWithMeta, UiCompiledInstruction, UiTransactionEncoding};
use std::io;
use std::str::FromStr;

use serde_json::to_string_pretty;

use crate::scraper::types::GenesisIntelligence;

async fn exhaust_transaction_signatures_for_genesis_signature(
    network: String,
    address: String,
) -> Result<Signature, io::Error> {
    let rpc_client = RpcClient::new(network);

    let mut earliest_signature: Option<Signature> = None;

    loop {
        let config = GetConfirmedSignaturesForAddress2Config {
            before: earliest_signature,
            until: None,
            limit: None,
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
    let address_clone = &address;
    let genesis_signature = exhaust_transaction_signatures_for_genesis_signature(
        network.clone(),
        address_clone.to_string(),
    )
    .await
    .unwrap();

    let rpc_client = RpcClient::new(network.clone());
    let transaction_meta = rpc_client
        .get_transaction(&genesis_signature, UiTransactionEncoding::JsonParsed)
        .await
        .unwrap();

    let transaction_meta_decode = transaction_meta.transaction.meta.unwrap();

    let inner_instructions = transaction_meta_decode.inner_instructions.unwrap();
    let mut mint_price: Option<u64> = None;
    let mut minter: Option<String> = None;
    if inner_instructions.len() > 1 {
        let tender_inner = inner_instructions[1].instructions[0].clone();
        if let solana_transaction_status::UiInstruction::Parsed(tender_inner_parsed) = tender_inner
        {
            if let solana_transaction_status::UiParsedInstruction::Parsed(
                tender_inner_parsed_parsed,
            ) = tender_inner_parsed
            {
                if tender_inner_parsed_parsed.parsed["type"] == "transfer" {
                    minter = Some(
                        tender_inner_parsed_parsed.parsed["info"]["source"]
                            .as_str()
                            .unwrap()
                            .to_string(),
                    );
                    mint_price = Some(
                        tender_inner_parsed_parsed.parsed["info"]["lamports"]
                            .as_u64()
                            .unwrap(),
                    );
                } else {
                    println!("{}", genesis_signature.to_string());
                }
            }
        }

        if mint_price == None {
            let inner = inner_instructions[1].instructions[1].clone();

            if let solana_transaction_status::UiInstruction::Parsed(tender_inner_parsed) = inner {
                if let solana_transaction_status::UiParsedInstruction::Parsed(
                    tender_inner_parsed_parsed,
                ) = tender_inner_parsed
                {
                    if tender_inner_parsed_parsed.parsed["type"] == "transfer" {
                        minter = Some(
                            tender_inner_parsed_parsed.parsed["info"]["source"]
                                .as_str()
                                .unwrap()
                                .to_string(),
                        );
                        mint_price = Some(
                            tender_inner_parsed_parsed.parsed["info"]["lamports"]
                                .as_u64()
                                .unwrap(),
                        );
                    }
                }
            }
        }
    }

    let genesis = GenesisIntelligence {
        block_time: transaction_meta.block_time.unwrap(),
        mint_price,
        genesis_transaction_signature: genesis_signature.to_string(),
        genesis_minter: minter,
        mint: address_clone.to_string(),
    };

    println!(
        "{}",
        to_string_pretty(&genesis).unwrap(), // [transaction_decoded.instructions()[6].accounts[2] as usize]
    );

    let file_path = format!("./db/{}.json", address_clone);
    std::fs::write(
        file_path,
        to_string_pretty(&genesis).unwrap(), // [transaction_decoded.instructions()[6].accounts[2] as usize]
    )
    .unwrap();

    Ok("".to_string())
}

pub async fn decode_transfer_instruction(network: String) -> Result<String, io::Error> {
    let rpc_client = RpcClient::new(network.clone());
    let transaction_meta = rpc_client
        .get_transaction(&Signature::from_str("FPuB67Rz97LyqmfjjAdmTE7NdgU4d7RsKQZFAMiZJfxagcRyMviFiumE8gNLCvnW5g3ECSfVAfKerGYx7Bn14CB").unwrap(), UiTransactionEncoding::Base64)
        .await
        .unwrap();

    let transaction_meta_decode = transaction_meta.transaction.transaction.decode();

    let transaction_meta_decoded = transaction_meta_decode.unwrap();
    let transaction_decoded = transaction_meta_decoded.clone().message;
    let _x = limited_deserialize::<SystemInstruction>(
        transaction_decoded.instructions()[0].data.as_slice(),
    );

    Ok("".to_string())
}
