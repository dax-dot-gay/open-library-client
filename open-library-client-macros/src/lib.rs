use manyhow::manyhow;

mod solr;

#[manyhow(proc_macro)]
pub use solr::solr;