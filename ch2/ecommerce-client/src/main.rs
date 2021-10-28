use ecommerce::product_info_client::ProductInfoClient;
use ecommerce::Product;

pub mod ecommerce {
    tonic::include_proto!("ecommerce");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting client");
    let mut client = ProductInfoClient::connect("http://[::1]:50051").await?;

    let product = tonic::Request::new(Product {
        id: "1".to_string(),
        name: "P1".to_string(),
        description: "P1 desc".to_string(),
        price: 1.0,
    });

    let response = client.add_product(product).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
