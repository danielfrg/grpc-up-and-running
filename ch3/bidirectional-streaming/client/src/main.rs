pub mod ecommerce {
    tonic::include_proto!("ecommerce");
}

use ecommerce::order_management_client::OrderManagementClient;
use ecommerce::Order;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting client");
    let mut client = OrderManagementClient::connect("http://[::1]:50051").await?;

    let outbound = async_stream::stream! {
        loop {
            let note = String::from("102");
            yield note;

            let note = String::from("103");
            yield note;

            let note = String::from("104");
            yield note;

            sleep(Duration::from_millis(1000)).await;
        }
    };

    let response = client.process_orders(tonic::Request::new(outbound)).await?;
    let mut inbound = response.into_inner();

    while let Some(note) = inbound.message().await? {
        println!("NOTE = {:?}", note);
    }

    Ok(())
}
