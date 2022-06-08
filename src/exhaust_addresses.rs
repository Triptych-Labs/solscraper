pub async fn get_genesis_transaction(msg: String) -> String {
    let ret = format!("hello {}", msg).to_string();
    ret
}
