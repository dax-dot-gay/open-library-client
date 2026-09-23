macro_rules! solr {
    ($name:ident($($tokens:tt)*) => $expect:literal) => {
        #[test]
        pub fn $name() -> () {
            let tree = crate::solr!(crate = crate, $($tokens)*);
            assert_eq!(tree.to_string(), $expect.to_string(), "Generated output should match expected");
        }
    };
}

solr!(literal("foo") => "foo");
solr!(literal_phrase("'foo'") => "\"foo\"");
solr!(literal_proximity_phrase("'foo'"~2) => "\"foo\"~2");
solr!(literal_proximity_bare("foo"~2) => "\"foo\"~2");
solr!(require(+"foo") => "+foo");
solr!(exclude(-"foo") => "-foo");
solr!(field_literal(foo:"bar") => "foo:bar");
solr!(field_literal_phrase(foo:"'bar'") => "foo:\"bar\"");
solr!(field_literal_proximity(foo:"bar"~2) => "foo:\"bar\"~2");
solr!(field_wildcard(foo:*) => "foo:*");
solr!(multi_clause(foo:"bar", baz:10) => "foo:bar baz:10");
solr!(top_seq((foo:"bar" & baz:10)) => "(foo:bar AND baz:10)");
solr!(field_seq(foo:("bar" | 10)) => "foo:(bar OR 10)");
solr!(double_range(foo:[10..20]) => "foo:[10 TO 20]");
solr!(lower_range(foo:[10..]) => "foo:[10 TO *]");
solr!(higher_range(foo:[..20]) => "foo:[* TO 20]");
solr!(combo_1(
    foo:"bar", red:true, ((yellow:5 | green:10) & baz:[10..20]), user:*, exact:"'exact'", prox:"10"~5
) => "foo:bar red:true ((yellow:5 OR green:10) AND baz:[10 TO 20]) user:* exact:\"exact\" prox:\"10\"~5");
