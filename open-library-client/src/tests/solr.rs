#[test]
pub fn scratchpad() {
    let tree = crate::solr!(crate = crate, (subject:"horror" & -author_key:*), publish_year:[..1800], subject:"tennis rules", ddc:"200*");
    println!("{tree}");
    assert_eq!(
        format!("{tree}"),
        "(subject:horror AND -author_key:*) publish_year:[* TO 1800] subject:tennis rules ddc:200*"
            .to_string(),
        "Generated output does not match expected"
    );
}
