use mini_redis::{client, Result};

#[tokio::main]
pub async fn main() -> Result<()> {
    // open a connection to mini-redis
    let mut client = client::connect("127.0.0.1:6379").await?;

    // set value
    client.set("hello", "world".into()).await?;

    // get value
    let result = client.get("hello").await?;

    print!("got value '{:?}' for key 'hello'", result);
    Ok(())
}
