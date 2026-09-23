use super::*;

#[tokio::test]
pub async fn search_books() -> crate::Result<()> {
    let api = client().await;
    let results = api.search_books("rust").limit(10).search().await?;
    assert!(
        results.results.len() <= 10,
        "More than 10 results were returned"
    );
    println!("{}", serde_json::to_string_pretty(&results.results[0]).unwrap());
    Ok(())
}

#[tokio::test]
pub async fn search_authors() -> crate::Result<()> {
    let api = client().await;
    let results = api
        .search_authors("twain")
        .limit(10)
        .search()
        .await?;
    assert!(
        results.results.len() <= 10,
        "More than 10 results were returned"
    );
    println!("{}", serde_json::to_string_pretty(&results.results[0]).unwrap());
    Ok(())
}

#[tokio::test]
pub async fn search_subjects() -> crate::Result<()> {
    let api = client().await;
    let results = api
        .search_subjects("science")
        .limit(10)
        .search()
        .await?;
    assert!(
        results.results.len() <= 10,
        "More than 10 results were returned"
    );
    println!("{}", serde_json::to_string_pretty(&results.results[0]).unwrap());
    Ok(())
}

#[tokio::test]
pub async fn get_author() -> crate::Result<()> {
    let api = client().await;
    let result = api.get_author("OL1394244A").await?;
    assert!(result.is_some(), "Should return a result");
    println!("{result:#?}");
    Ok(())
}

#[tokio::test]
pub async fn get_author_works() -> crate::Result<()> {
    let api = client().await;
    let result = api.get_author_works("OL1394244A").limit(50).get().await?;
    assert!(result.is_some(), "Should return a result");
    assert!(result.clone().unwrap().entries.len() <= 50, "Should return at most 50 results");
    println!("{result:#?}");
    Ok(())
}

#[tokio::test]
pub async fn get_work() -> crate::Result<()> {
    let api = client().await;
    let result = api.get_work("OL8193416W").await?;
    assert!(result.is_some(), "Should return a result");
    println!("{result:#?}");
    Ok(())
}

#[tokio::test]
pub async fn get_edition() -> crate::Result<()> {
    let api = client().await;
    let result = api.get_edition("OL61135107M").await?;
    assert!(result.is_some(), "Should return a result");
    println!("{result:#?}");
    Ok(())
}

#[tokio::test]
pub async fn get_subject() -> crate::Result<()> {
    let api = client().await;
    let result = api.get_subject("love").details(true).get().await?;
    assert!(result.works.len() > 0, "Should return at least one work");
    println!("{result:#?}");
    Ok(())
}

#[tokio::test]
pub async fn get_image() -> crate::Result<()> {
    let api = client().await;
    let metadata = api.get_cover_image_data("OL35615701M").get().await?;
    assert_eq!(metadata.olid, "OL35615701M".to_string().into(), "Should return the same OLID");
    let data = api.get_cover_image("OL35615701M").size(crate::core::types::covers::CoverImageSize::Large).get().await?;
    assert!(data.len() > 0, "Some byte data should be returned");
    println!("{metadata:#?}");
    Ok(())
}

