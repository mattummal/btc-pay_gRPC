use payment::btc_client::BtcClient;
use payment::PayBtcRequest;

pub mod payment {
    tonic::include_proto!("payment");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = BtcClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(
        PayBtcRequest {
            donor: "Rab".to_owned(),
            beneficiary: "Abhi".to_owned(),
            amount: 47
        }
    );

    let response = client.pay_btc(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}