pub mod ecommerce {
    tonic::include_proto!("ecommerce");
}

use ecommerce::order_management_client::OrderManagementClient;
use ecommerce::Order;
use futures::stream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting client");
    let mut client = OrderManagementClient::connect("http://[::1]:50051").await?;

    let mut orders = vec![];
    for i in 0..10 {
        orders.push(Order {
            id: i.to_string(),
            items: vec!["Item".to_string()],
            description: "New item from stream".to_string(),
            price: 1.0,
            destination: "Your house".to_string(),
        })
    }

    println!("Updating {} orders", orders.len());
    let request = tonic::Request::new(stream::iter(orders));

    match client.update_orders(request).await {
        Ok(response) => println!("SUMMARY: {:?}", response.into_inner()),
        Err(e) => println!("something went wrong: {:?}", e),
    }

    Ok(())
}
