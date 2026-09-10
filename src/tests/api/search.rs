use crate::{tests::unauthenticated_client, types::search::SearchType};

#[tokio::test]
pub async fn api_search_books() -> crate::Result<()> {
    let client = unauthenticated_client();
    let search = client
        .search(SearchType::Work, "lord of the rings")
        .field("*")
        .offset(0, 3)
        .build();
    let result = search.search().await?;
    assert!(
        result.results.len() > 0,
        "Should return at least one result"
    );
    assert_eq!(result.results[0].entity_type(), SearchType::Work, "Should return a Work");
    println!("{}", result.results[0].reference());
    Ok(())
}

#[tokio::test]
pub async fn api_search_authors() -> crate::Result<()> {
    let client = unauthenticated_client();
    let search = client
        .search(SearchType::Author, "mark twain")
        .field("*")
        .offset(0, 3)
        .build();
    let result = search.search().await?;
    assert!(
        result.results.len() > 0,
        "Should return at least one result"
    );
    assert_eq!(result.results[0].entity_type(), SearchType::Author, "Should return an Author");
    println!("{}", result.results[0].reference());
    Ok(())
}

#[tokio::test]
pub async fn api_search_subjects() -> crate::Result<()> {
    let client = unauthenticated_client();
    let search = client.search(SearchType::Subject, "science")
        .field("*")
        .offset(0, 3)
        .build();
    let result = search.search().await?;
    assert!(
        result.results.len() > 0,
        "Should return at least one result"
    );
    assert_eq!(result.results[0].entity_type(), SearchType::Subject, "Should return a Subject");
    println!("{}", result.results[0].reference());
    Ok(())
}
