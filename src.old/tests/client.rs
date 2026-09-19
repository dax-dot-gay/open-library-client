use super::*;

#[test]
pub fn client_unauthenticated() {
    let client = unauthenticated_client();
    assert_eq!(
        client.user_agent(),
        format!(
            "open-library-client/tests (https://crates.io/crates/open-library-client) open-library-client/{} (https://crates.io/crates/open-library-client)",
            clap::crate_version!()
        ),
        "Ensuring that generated user agents match the desired format"
    );
}

#[tokio::test]
pub async fn client_authenticated() -> crate::Result<()> {
    let client = unauthenticated_client();
    client
        .login(
            dotenv::var("OLC_ACCESS_KEY").unwrap(),
            dotenv::var("OLC_SECRET_KEY").unwrap(),
        )
        .await?;
    Ok(())
}
