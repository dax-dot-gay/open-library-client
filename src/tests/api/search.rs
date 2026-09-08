use crate::tests::unauthenticated_client;

#[tokio::test]
pub async fn api_search_books() -> crate::Result<()> {
    let client = unauthenticated_client();
    let search = client
        .search()
        .books("lord of the rings")
        .field("*")
        .offset(0, 3)
        .build();
    let result = search.search().await?;
    assert!(
        result.results.len() > 0,
        "Should return at least one result"
    );
    println!("{:?}", result.results[0]);
    Ok(())
}

#[tokio::test]
pub async fn api_search_authors() -> crate::Result<()> {
    let client = unauthenticated_client();
    let search = client
        .search()
        .authors("mark twain")
        .field("*")
        .offset(0, 3)
        .build();
    let result = search.search().await?;
    assert!(
        result.results.len() > 0,
        "Should return at least one result"
    );
    println!("{:?}", result.results[0]);
    Ok(())
}

#[tokio::test]
pub async fn api_search_subjects() -> crate::Result<()> {
    let client = unauthenticated_client();
    let search = client
        .search()
        .subjects("science")
        .field("*")
        .offset(0, 3)
        .build();
    let result = search.search().await?;
    assert!(
        result.results.len() > 0,
        "Should return at least one result"
    );
    println!("{:?}", result.results[0]);
    Ok(())
}
