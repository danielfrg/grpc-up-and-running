use ecommerce::order_management_client::OrderManagementClient;

pub mod ecommerce {
    tonic::include_proto!("ecommerce");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting client");
    let mut client = OrderManagementClient::connect("http://[::1]:50051").await?;

    let order = tonic::Request::new(String::from("102"));
    let response = client.get_order(order).await?;
    println!("RESPONSE={:?}", response);

    Ok(())
}
