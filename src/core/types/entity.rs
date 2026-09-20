//! Representations of OpenLibrary entity types (Work, Author, etc) returned from searches

use serde::{Deserialize, Serialize};

/// Entity type
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EntityType {
    Work,
    Author,
    Subject,
}

macro_rules! EntityType {
    ($name:ident ($desc:literal) {$($field_name:ident: $field_type:ty),*}) => {
        #[doc = $desc]
        #[allow(missing_docs)]
        #[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
        pub struct $name {
            /// Unique key for this document
            pub key: String,

            $(#[serde(default)] pub $field_name: Option<$field_type>),*,

            /// All returned fields not captured by the strong type
            #[serde(flatten)]
            pub extras: std::collections::HashMap<String, serde_json::Value>
        }
    };
}

macro_rules! SubEntityType {
    ($name:ident ($desc:literal) {$($field_name:ident: $field_type:ty),*}) => {
        #[doc = $desc]
        #[allow(missing_docs)]
        #[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
        pub struct $name {
            $(#[serde(default)] pub $field_name: Option<$field_type>),*,

            /// All returned fields not captured by the strong type
            #[serde(flatten)]
            pub extras: std::collections::HashMap<String, serde_json::Value>
        }
    };
}

SubEntityType!(WorkAvailability ("Availability data for Works") {
    status: String,
    available_to_browse: bool,
    available_to_borrow: bool,
    available_to_waitlist: bool,
    is_printdisabled: bool,
    is_readable: bool,
    is_lendable: bool,
    is_previewable: bool,
    identifier: String,
    isbn: String,
    oclc: String,
    openlibrary_work: String,
    openlibrary_edition: String,
    last_loan_date: String,
    num_waitlist: String,
    last_waitlist_date: String,
    is_restricted: bool,
    is_browseable: bool
});

EntityType!(WorkEntity ("Representation of an OpenLibrary Work") {
    author_alternative_name: Vec<String>,
    author_key: Vec<String>,
    author_name: Vec<String>,
    chapter: Vec<String>,
    contributor: Vec<String>,
    cover_edition_key: String,
    cover_height: u64,
    cover_i: i64,
    cover_width: u64,
    ddc: Vec<String>,
    ebook_access: String,
    ebook_count_i: i64,
    ebook_provider: Vec<String>,
    edition_count: u64,
    edition_key: Vec<String>,
    first_publish_year: i64,
    first_sentence: Vec<String>,
    format: Vec<String>,
    has_fulltext: bool,
    ia: Vec<String>,
    ia_collection: Vec<String>,
    isbn: Vec<String>,
    language: Vec<String>,
    last_modified_i: i64,
    lcc: Vec<String>,
    lccn: Vec<String>,
    lending_edition_s: String,
    lending_identifier_s: String,
    lexile: Vec<i64>,
    number_of_pages_median: u64,
    oclc: Vec<String>,
    osp_count: u64,
    printdisabled_s: String,
    public_scan_b: bool,
    publish_date: Vec<String>,
    publish_place: Vec<String>,
    publish_year: Vec<i64>,
    publisher: Vec<String>,
    series_key: Vec<String>,
    series_name: Vec<String>,
    series_position: Vec<String>,
    title: String,
    title_suggest: String,
    id_amazon: Vec<String>,
    id_goodreads: Vec<String>,
    id_better_world_books: Vec<String>,
    id_storygraph: Vec<String>,
    id_librarything: Vec<String>,
    id_bookbrainz: Vec<String>,
    id_wikidata: Vec<String>,
    id_yakaboo: Vec<String>,
    id_google: Vec<String>,
    id_alibris_id: Vec<String>,
    id_hathi_trust: Vec<String>,
    id_paperback_swap: Vec<String>,
    subject: Vec<String>,
    place: Vec<String>,
    time: Vec<String>,
    person: Vec<String>,
    ia_box_id: Vec<String>,
    ratings_average: f64,
    ratings_sortable: f64,
    ratings_count: u64,
    ratings_count_1: u64,
    ratings_count_2: u64,
    ratings_count_3: u64,
    ratings_count_4: u64,
    ratings_count_5: u64,
    readinglog_count: u64,
    want_to_read_count: u64,
    currently_reading_count: u64,
    already_read_count: u64,
    stopped_reading_count: u64,
    trending_score_hourly_0: u64,
    trending_score_hourly_1: u64,
    trending_score_hourly_2: u64,
    trending_score_hourly_3: u64,
    trending_score_hourly_4: u64,
    trending_score_hourly_5: u64,
    trending_score_hourly_6: u64,
    trending_score_hourly_7: u64,
    trending_score_hourly_8: u64,
    trending_score_hourly_9: u64,
    trending_score_hourly_10: u64,
    trending_score_hourly_11: u64,
    trending_score_hourly_12: u64,
    trending_score_hourly_13: u64,
    trending_score_hourly_14: u64,
    trending_score_hourly_15: u64,
    trending_score_hourly_16: u64,
    trending_score_hourly_17: u64,
    trending_score_hourly_18: u64,
    trending_score_hourly_19: u64,
    trending_score_hourly_20: u64,
    trending_score_hourly_21: u64,
    trending_score_hourly_22: u64,
    trending_score_hourly_23: u64,
    trending_score_daily_0: u64,
    trending_score_daily_1: u64,
    trending_score_daily_2: u64,
    trending_score_daily_3: u64,
    trending_score_daily_4: u64,
    trending_score_daily_5: u64,
    trending_score_daily_6: u64,
    person_key: Vec<String>,
    person_facet: Vec<String>,
    time_key: Vec<String>,
    subject_key: Vec<String>,
    lcc_sort: String,
    publisher_facet: Vec<String>,
    place_key: Vec<String>,
    place_facet: Vec<String>,
    subject_facet: Vec<String>,
    author_facet: Vec<String>,
    trending_z_score: f64,
    trending_score_hourly_sum: u64,
    ddc_sort: String,
    availability: WorkAvailability
});

EntityType!(AuthorEntity ("Representation of an OpenLibrary Author") {
    name: String,
    top_subjects: Vec<String>,
    top_work: String,
    work_count: u64,
    ratings_average: f64,
    ratings_sortable: f64,
    ratings_count: u64,
    ratings_count_1: u64,
    ratings_count_2: u64,
    ratings_count_3: u64,
    ratings_count_4: u64,
    ratings_count_5: u64,
    readinglog_count: u64,
    want_to_read_count: u64,
    currently_reading_count: u64,
    already_read_count: u64,
    stopped_reading_count: u64
});

EntityType!(SubjectEntity ("Representation of an OpenLibrary Subject") {
    name: String,
    subject_type: String,
    work_count: u64,
    count: u64
});

/// Combinator enum for search results
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Entity {
    Work(WorkEntity),
    Author(AuthorEntity),
    Subject(SubjectEntity),
}

impl Entity {
    /// Get the [`EntityType`] of this entity
    pub fn entity_type(&self) -> EntityType {
        match self.clone() {
            Entity::Work(_) => EntityType::Work,
            Entity::Author(_) => EntityType::Author,
            Entity::Subject(_) => EntityType::Subject,
        }
    }

    /// Get the normalized OLID of this entity
    pub fn olid(&self) -> String {
        let key = match self.clone() {
            Entity::Work(work_entity) => work_entity.key,
            Entity::Author(author_entity) => author_entity.key,
            Entity::Subject(subject_entity) => subject_entity.key,
        };

        key.split("/").last().unwrap().to_string()
    }
}
