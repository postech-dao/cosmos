use crate::request;

pub async fn send_query(
    rest_api_endpoint: &str,
    contract_address: &str,
    encode_msg: &str
){
    let client = reqwest::Client::new();
    let response = request(
        &client,
        &format!(
            "https://{}/juno/wasm/v1beta1/contract/{}/store?query_msg={}",
            rest_api_endpoint, contract_address, encode_msg
        ),
        None,
    )
    .await
    .unwrap();
    
    println!("{}", response);
    //TODO: check response and test
    //assert_eq!(response["count"], "0");
}