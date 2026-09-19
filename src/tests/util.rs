pub async fn client() -> crate::Client {
    let client = crate::Client::builder()
        .user_agent("open-library-client/test (git@dax.gay)")
        .build()
        .await
        .unwrap();
    client
}
