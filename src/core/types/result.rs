//! Standardized typed results

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use crate::core::types::ExplicitType;

use super::OLID;

macro_rules! ResultType {
    ($name:ident ($desc:literal) {
        $(required = {$($($(#[$req_field_attr:meta])+)? $req_field_name:ident: $req_field_type:ty),*};)?
        $(optional = {$($($(#[$opt_field_attr:meta])+)? $opt_field_name:ident: $opt_field_type:ty),*};)?
    }) => {
        #[doc = $desc]
        #[allow(missing_docs)]
        #[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
        pub struct $name {
            $($($($(#[$req_field_attr])*)? pub $req_field_name: $req_field_type),*,)?
            $($(#[serde(default, skip_serializing_if = "Option::is_none")] $($(#[$opt_field_attr])*)? pub $opt_field_name: Option<$opt_field_type>),*,)?

            /// Fields not otherwise covered by this type
            #[serde(flatten)]
            pub rest: std::collections::HashMap<String, serde_json::Value>
        }
    };
}

/// Enum describing a search result type
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "snake_case")]
#[allow(missing_docs)]
pub enum SearchResultKind {
    #[serde(alias = "type:work")]
    Work,

    #[serde(alias = "type:author")]
    Author,

    #[serde(alias = "type:subject")]
    Subject,
}

#[doc(hidden)]
#[allow(missing_docs)]
impl SearchResultKind {
    pub fn work() -> Self {
        Self::Work
    }

    pub fn author() -> Self {
        Self::Author
    }

    pub fn subject() -> Self {
        Self::Subject
    }
}

ResultType!(SearchWorkAvailability ("Availability data for Works") {
    optional = {
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
    };
});

ResultType!(SearchWork ("A single work returned from a book search") {
    required = {
        /// Unique ID
        #[serde(default)]
        key: OLID,
        /// Result type
        #[serde(skip_deserializing, default = "SearchResultKind::work")] kind: SearchResultKind
    };
    optional = {
        author_alternative_name: Vec<String>,
        author_key: Vec<String>,
        author_name: Vec<String>,
        chapter: Vec<String>,
        contributor: Vec<String>,
        cover_edition_key: String,
        cover_height: i64,
        cover_i: i64,
        cover_width: i64,
        ddc: Vec<String>,
        ebook_access: String,
        ebook_count_i: i64,
        ebook_provider: Vec<String>,
        edition_count: i64,
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
        number_of_pages_median: i64,
        oclc: Vec<String>,
        osp_count: i64,
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
        ratings_count: i64,
        ratings_count_1: i64,
        ratings_count_2: i64,
        ratings_count_3: i64,
        ratings_count_4: i64,
        ratings_count_5: i64,
        readinglog_count: i64,
        want_to_read_count: i64,
        currently_reading_count: i64,
        already_read_count: i64,
        stopped_reading_count: i64,
        trending_score_hourly_0: i64,
        trending_score_hourly_1: i64,
        trending_score_hourly_2: i64,
        trending_score_hourly_3: i64,
        trending_score_hourly_4: i64,
        trending_score_hourly_5: i64,
        trending_score_hourly_6: i64,
        trending_score_hourly_7: i64,
        trending_score_hourly_8: i64,
        trending_score_hourly_9: i64,
        trending_score_hourly_10: i64,
        trending_score_hourly_11: i64,
        trending_score_hourly_12: i64,
        trending_score_hourly_13: i64,
        trending_score_hourly_14: i64,
        trending_score_hourly_15: i64,
        trending_score_hourly_16: i64,
        trending_score_hourly_17: i64,
        trending_score_hourly_18: i64,
        trending_score_hourly_19: i64,
        trending_score_hourly_20: i64,
        trending_score_hourly_21: i64,
        trending_score_hourly_22: i64,
        trending_score_hourly_23: i64,
        trending_score_daily_0: i64,
        trending_score_daily_1: i64,
        trending_score_daily_2: i64,
        trending_score_daily_3: i64,
        trending_score_daily_4: i64,
        trending_score_daily_5: i64,
        trending_score_daily_6: i64,
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
        trending_score_hourly_sum: i64,
        ddc_sort: String,
        availability: SearchWorkAvailability,
        editions: ExplicitType
    };
});

ResultType!(SearchAuthor ("A single author returned from an author search") {
    required = {
        /// Unique ID
        #[serde(default)]
        key: OLID,
        /// Result type
        #[serde(skip_deserializing, default = "SearchResultKind::author")] kind: SearchResultKind
    };
    optional = {
        name: String,
        top_subjects: Vec<String>,
        top_work: String,
        work_count: i64,
        ratings_average: f64,
        ratings_sortable: f64,
        ratings_count: i64,
        ratings_count_1: i64,
        ratings_count_2: i64,
        ratings_count_3: i64,
        ratings_count_4: i64,
        ratings_count_5: i64,
        readinglog_count: i64,
        want_to_read_count: i64,
        currently_reading_count: i64,
        already_read_count: i64,
        stopped_reading_count: i64
    };
});

ResultType!(SearchSubject ("A single subject returned from a subject search") {
    required = {
        /// Unique ID
        #[serde(default)]
        key: OLID,
        /// Result type
        #[serde(skip_deserializing, default = "SearchResultKind::subject")] kind: SearchResultKind
    };
    optional = {
        name: String,
        subject_type: String,
        work_count: i64,
        count: i64
    };
});

ResultType!(SelectedAuthor ("An author selected with get_author") {
    required = {
        /// Unique ID
        #[serde(default)]
        key: OLID
    };
    optional = {
        name: String,
        eastern_order: bool,
        personal_name: String,
        enumeration: String,
        title: String,
        alternate_names: Vec<String>,
        uris: Vec<String>,
        bio: ExplicitType,
        location: String,
        birth_date: String,
        death_date: String,
        date: String,
        wikipedia: String,
        links: Vec<ExplicitType>,
        latest_revision: i64,
        revision: i64,
        created: ExplicitType,
        last_modified: ExplicitType,
        remote_ids: HashMap<String, String>,
        source_records: Vec<String>,
        photos: Vec<i64>,
        fuller_name: String,
        photograph: String
    };
});

ResultType!(SelectedWork ("A work selected with get_work or get_author_works") {
    required = {
        /// Unique ID
        #[serde(default)]
        key: OLID
    };
    optional = {
        title: String,
        subtitle: String,
        authors: Vec<ExplicitType>,
        translated_titles: Vec<ExplicitType>,
        subjects: Vec<String>,
        subject_places: Vec<String>,
        subject_times: Vec<String>,
        subject_people: Vec<String>,
        excerpts: Vec<ExplicitType>,
        series: Vec<ExplicitType>,
        description: ExplicitType,
        dewey_number: Vec<String>,
        lc_classifications: Vec<String>,
        first_sentence: ExplicitType,
        original_languages: Vec<ExplicitType>,
        other_titles: Vec<String>,
        first_publish_date: String,
        links: Vec<ExplicitType>,
        notes: ExplicitType,
        cover_edition: ExplicitType,
        covers: Vec<i64>,
        genres: Vec<ExplicitType>,
        subgenres: Vec<ExplicitType>,
        audiences: Vec<ExplicitType>,
        works: Vec<ExplicitType>,
        identifiers: HashMap<String, Vec<String>>,
        location: ExplicitType,
        latest_revision: i64,
        revision: i64,
        created: ExplicitType,
        last_modified: ExplicitType
    };
});

ResultType!(SelectedEdition ("An edition selected with get_edition") {
    required = {
        /// Unique ID
        #[serde(default)]
        key: OLID
    };
    optional = {
        title: String,
        title_prefix: String,
        subtitle: String,
        other_titles: Vec<String>,
        authors: Vec<ExplicitType>,
        by_statement: String,
        publish_date: String,
        copyright_date: String,
        edition_name: String,
        languages: Vec<ExplicitType>,
        description: ExplicitType,
        notes: ExplicitType,
        genres: Vec<String>,
        table_of_contents: Vec<ExplicitType>,
        work_titles: Vec<String>,
        series: Vec<String>,
        physical_dimensions: String,
        physical_format: String,
        number_of_pages: i64,
        subjects: Vec<String>,
        pagination: String,
        lccn: Vec<String>,
        ocaid: String,
        #[serde(alias = "oclc_number")]
        oclc_numbers: Vec<String>,
        isbn_10: Vec<String>,
        isbn_13: Vec<String>,
        dewey_decimal_class: Vec<String>,
        lc_classifications: Vec<String>,
        contributions: Vec<String>,
        publish_places: Vec<String>,
        publish_country: String,
        publishers: Vec<String>,
        distributors: Vec<String>,
        first_sentence: ExplicitType,
        weight: String,
        location: Vec<String>,
        scan_on_demand: bool,
        collections: Vec<ExplicitType>,
        uris: Vec<String>,
        uri_descriptions: Vec<String>,
        translation_of: String,
        works: Vec<ExplicitType>,
        source_records: Vec<String>,
        translated_from: Vec<ExplicitType>,
        scan_records: Vec<ExplicitType>,
        volumes: Vec<ExplicitType>,
        accompanying_material: String,
        covers: Vec<i64>,
        local_id: Vec<String>,
        full_title: String,
        identifiers: HashMap<String, Vec<String>>,
        contributors: Vec<ExplicitType>,
        #[serde(alias = "subject_place")]
        subject_places: Vec<String>,
        #[serde(alias = "subject_time")]
        subject_times: Vec<String>,
        subject_people: Vec<String>,
        classifications: HashMap<String, Vec<String>>,
        links: Vec<ExplicitType>,
        latest_revision: i64,
        revision: i64,
        created: ExplicitType,
        last_modified: ExplicitType
    };
});

/// A representation of an object linked to a [`SelectedSubject`]
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum SubjectRelation {
    /// The standard format
    Regular {
        key: String,
        name: String,
        count: u64
    },

    /// The format of the `languages` key
    Language {
        name: String,
        count: u64
    },

    /// The format of the `publishing_history` key
    PublishingHistory(
        i64,
        u64
    )
}

impl SubjectRelation {
    /// Get the unique identifier of this relation
    pub fn identifier(&self) -> String {
        match self.clone() {
            SubjectRelation::Regular { key, .. } => key,
            SubjectRelation::Language { name, .. } => name,
            SubjectRelation::PublishingHistory(year, _) => year.to_string(),
        }
    }

    /// Get the count of this relation
    pub fn count(&self) -> u64 {
        match self.clone() {
            SubjectRelation::Regular { count, .. } => count,
            SubjectRelation::Language { count, .. } => count,
            SubjectRelation::PublishingHistory(_, count) => count,
        }
    }
}

ResultType!(SubjectRelatedWork ("A work related to a [`SelectedSubject`]") {
    required = {
        /// Unique ID
        #[serde(default)]
        key: OLID
    };
    optional = {
        title: String,
        edition_count: i64,
        cover_id: i64,
        cover_edition_key: String,
        subject: Vec<String>,
        ia_collection: Vec<String>,
        printdisabled: bool,
        lending_edition: String,
        lending_identifier: String,
        authors: Vec<ExplicitType>,
        first_publish_year: i64,
        ia: String,
        public_scan: bool,
        has_fulltext: bool,
        availability: SearchWorkAvailability
    };
});

ResultType!(SelectedSubject ("Subject information selected with get_subject") {
    required = {
        /// Unique ID
        #[serde(default)]
        key: OLID,
        name: String,
        subject_type: String,
        solr_query: String,
        work_count: u64,
        ebook_count: u64,

        #[serde(default)]
        works: Vec<SubjectRelatedWork>
    };
    optional = {
        subjects: Vec<SubjectRelation>,
        places: Vec<SubjectRelation>,
        people: Vec<SubjectRelation>,
        times: Vec<SubjectRelation>,
        authors: Vec<SubjectRelation>,
        publishers: Vec<SubjectRelation>,
        languages: Vec<SubjectRelation>,
        publishing_history: Vec<SubjectRelation>
    };
});

ResultType!(CoverImage ("Metadata of a cover image from get_cover_image_data") {
    required = {
        id: u64,
        olid: OLID
    };
    optional = {
        category_id: u64,
        filename: String,
        author: String,
        ip: String,
        source_url: String,
        isbn: String,
        source: String,
        created: String,
        last_modified: String,
        archived: bool,
        failed: bool,
        width: u64,
        height: u64,
        filename_s: String,
        filename_m: String,
        filename_l: String,
        isbn13: String,
        uploaded: bool,
        deleted: bool,
        filename_old: String
    };
});
