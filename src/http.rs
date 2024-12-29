
#[tokio::test]
async fn get() -> Result<(), Error> {
    let client = Client::new();
    let response = client.get("https://www.baidu.com").send().await?;
    assert!(response.status().is_success());
    Ok(())
}