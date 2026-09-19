use super::*;
use crate::CoreApi;

#[tokio::test]
pub async fn search_books() -> crate::Result<()> {
    let api = client().await;
    let results = api.core().search_books("rust").limit(10).search().await?;
    assert!(
        results.results.len() <= 10,
        "More than 10 results were returned"
    );
    Ok(())
}

#[tokio::test]
pub async fn search_authors() -> crate::Result<()> {
    let api = client().await;
    let results = api
        .core()
        .search_authors("twain")
        .limit(10)
        .search()
        .await?;
    assert!(
        results.results.len() <= 10,
        "More than 10 results were returned"
    );
    Ok(())
}

#[tokio::test]
pub async fn search_subjects() -> crate::Result<()> {
    let api = client().await;
    let results = api
        .core()
        .search_subjects("science")
        .limit(10)
        .search()
        .await?;
    assert!(
        results.results.len() <= 10,
        "More than 10 results were returned"
    );
    Ok(())
}
