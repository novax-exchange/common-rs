use novax_services::tokio as tokio;
use novax_services::error::{Result};
#[tokio::main]
async fn main() -> Result<()>{
    
    // std::env::set_var("HTTP_ADDR", "127.0.0.1:8086");
    // println!(" {:? }", std::env::var("HTTP_ADDR").unwrap());
    let addr = std::env::var("HTTP_ADDR").unwrap_or("127.0.0.1:8086".to_string());
    println!(" the address is {}", addr);
    match novax_services::service(addr).await {
        Ok(_) => println!("service is running"),
        Err(e) => eprintln!("error executing service {:?}", e)
    };
    Ok(())
}
