pub mod ecommerce {
    tonic::include_proto!("ecommerce");
}

use ecommerce::order_management_client::OrderManagementClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting client");
    let mut client = OrderManagementClient::connect("http://[::1]:50051").await?;

    let order = tonic::Request::new(String::from("Google"));
    let mut stream = client.search_orders(order).await.unwrap().into_inner();
    println!("RESPONSE={:?}", stream);

    while let Some(feature) = stream.message().await? {
        println!("ORDER = {:?}", feature);
    }

    Ok(())
}
