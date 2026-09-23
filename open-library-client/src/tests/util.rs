pub async fn client() -> crate::OpenLibraryClientCore {
    let client = crate::OpenLibraryClientCore::new(Some("open-library-client/test (git@dax.gay)"));
    client
}
