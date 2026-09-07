use std::path::Path;

pub fn unauthenticated_client() -> crate::OpenLibraryClient {
    crate::OpenLibraryClient::builder()
        .app(
            "open-library-client/tests",
            "https://crates.io/crates/open-library-client",
        )
        .build()
}

pub async fn authenticated_client() -> crate::OpenLibraryClient {
    dotenv::from_path(Path::new(env!("CARGO_MANIFEST_DIR")).join(".env"))
        .expect(".env should exist");
    let client = unauthenticated_client();
    client
        .login(
            dotenv::var("OLC_ACCESS_KEY").unwrap(),
            dotenv::var("OLC_SECRET_KEY").unwrap(),
        )
        .await
        .expect("login failed while not being tested for");
    client
}
