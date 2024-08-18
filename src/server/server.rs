use tonic::{transport::Server, Request, Response, Status};

use payment::btc_server::{Btc, BtcServer};
use payment::{PayBtcResponse, PayBtcRequest};

pub mod payment {
    tonic::include_proto!("payment");
}

#[derive(Debug, Default)]
pub struct BTCService {}

#[tonic::async_trait]
impl Btc for BTCService {
    async fn pay_btc(
        &self,
        request: Request<PayBtcRequest>,
    ) -> Result<Response<PayBtcResponse>, Status> {
        println!("Request received: {:?}", request);

        let req = request.into_inner();

        let res = PayBtcResponse {
            successful: true,
            message: format!("{} has sent {}BTC to {}.", req.donor, req.amount, req.beneficiary).into(),
        };

        Ok(Response::new(res))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address: std::net::SocketAddr = "[::1]:50051".parse()?;
    let btc_service = BTCService::default();

    Server::builder()
        .add_service(BtcServer::new(btc_service))
        .serve(address)
        .await?;

    Ok(())
}