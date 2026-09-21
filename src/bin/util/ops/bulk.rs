use clap::Args;
use open_library_client::{Client, CoreApi, core::types::SearchWork};
use std::{cmp::min, fs::File, io::Write, time::Duration};

#[derive(Clone, Debug, Args)]
pub struct BulkArgs {
    /// List of subject searches to start with
    subjects: Vec<String>,

    /// Max results to return from any bulk request
    #[arg(short, long, default_value_t = 10)]
    limit: u8,

    /// Output path
    #[arg(short, long, default_value_t = String::from("bulk.log"))]
    output: String,
}

impl BulkArgs {
    async fn process_edition(&self, client: &Client, output: &mut File, edition: String) -> crate::Result<()> {
        let edition_details = client.core().get_edition(edition).await?;
        if let Some(found_ed) = edition_details {
            writeln!(output, "          DETAILS: {{")?;
            let mut trimmed_rest = found_ed.rest.clone();
            let _ = trimmed_rest.remove(&"type".to_string());
            writeln!(output, "            REST = {trimmed_rest:?}")?;
            writeln!(output, "          }}")?;
        } else {
            writeln!(output, "          DETAILS: {{NONE}}")?;
        }
        Ok(())
    }

    async fn process_author(&self, client: &Client, output: &mut File, author: String) -> crate::Result<()> {
        let author_details = client.core().get_author(author).await?;
        if let Some(found_au) = author_details {
            writeln!(output, "          DETAILS: {{")?;
            let mut trimmed_rest = found_au.rest.clone();
            let _ = trimmed_rest.remove(&"type".to_string());
            writeln!(output, "            REST = {trimmed_rest:?}")?;
            writeln!(output, "          }}")?;
        } else {
            writeln!(output, "          DETAILS: {{NONE}}")?;
        }
        Ok(())
    }

    async fn process_work(
        &self,
        client: &Client,
        output: &mut File,
        work: SearchWork,
    ) -> crate::Result<()> {
        let work_details = client.core().get_work(work.key.clone()).await?;

        if let Some(found_work) = work_details {
            writeln!(output, "      DETAILS: {{")?;
            let mut trimmed_rest = found_work.rest.clone();
            let _ = trimmed_rest.remove(&"type".to_string());
            writeln!(output, "        REST = {trimmed_rest:?}")?;
            writeln!(output, "      }}")?;
        } else {
            writeln!(output, "      DETAILS: {{NONE}}")?;
        }

        if let Some(editions) = work.edition_key.clone() {
            if editions.len() > 0 {
                writeln!(output, "      EDITIONS ({}/{}): {{", min(self.limit.into(), editions.len()), editions.len())?;
                let mut trunc_editions = editions.clone();
                trunc_editions.truncate(self.limit.into());
                for edition in trunc_editions {
                    writeln!(output, "        {edition}: {{")?;
                    if let Err(failure) = self.process_edition(&client, output, edition).await {
                        writeln!(output, "          !!ERROR: {failure:?}")?;
                    }
                    writeln!(output, "        }}")?;
                }
            }
        }

        if let Some(authors) = work.author_key.clone() {
            if authors.len() > 0 {
                writeln!(output, "      AUTHORS ({}/{}): {{", min(self.limit.into(), authors.len()), authors.len())?;
                let mut trunc_authors = authors.clone();
                trunc_authors.truncate(self.limit.into());
                for author in trunc_authors {
                    writeln!(output, "        {author}: {{")?;
                    if let Err(failure) = self.process_author(&client, output, author).await {
                        writeln!(output, "          !!ERROR: {failure:?}")?;
                    }
                    writeln!(output, "        }}")?;
                }
            }
        }

        Ok(())
    }

    async fn process_subj(
        &self,
        client: &Client,
        output: &mut File,
        subj: String,
    ) -> crate::Result<()> {
        let works = client
            .core()
            .search_books(format!("subject:{subj}"))
            .limit(self.limit)
            .fields("key,edition_key,author_key,title")
            .search()
            .await?
            .results;

        for work in works.clone() {
            writeln!(
                output,
                "    WORK: {} ({}) {{",
                work.title.clone().unwrap_or_default(),
                work.key
            )?;
            if let Err(failure) = self.process_work(&client, output, work).await {
                writeln!(output, "      !!ERROR: {failure:?}")?;
            }
            writeln!(output, "    }}\n")?;
        }

        Ok(())
    }

    async fn process_root(
        &self,
        client: &Client,
        output: &mut File,
        root: String,
    ) -> crate::Result<()> {
        let subjects = client
            .core()
            .search_subjects(root.clone())
            .limit(self.limit)
            .search()
            .await?
            .results;

        for subj in subjects.clone() {
            writeln!(output, "  SUBJ: {} {{", subj.key)?;
            if let Err(failure) = self
                .process_subj(&client, output, subj.key.to_string())
                .await
            {
                writeln!(output, "    !!ERROR: {failure:?}")?;
            }
            writeln!(output, "  }}\n")?;
        }

        Ok(())
    }

    pub async fn run(&self) -> crate::Result<()> {
        let client = Client::builder()
            .client(reqwest::ClientBuilder::new().timeout(Duration::from_secs(120)))
            .user_agent("open_library_client/bin/util.bulk (git@dax.gay)")
            .build()
            .await?;
        let mut output = std::fs::File::create(self.output.clone())?;

        for root in self.subjects.clone() {
            writeln!(output, "ROOT: {root} {{")?;
            if let Err(failure) = self.process_root(&client, &mut output, root).await {
                writeln!(output, "  !!ERROR: {failure:?}")?;
            }
            writeln!(output, "}}\n\n===============\n")?;
        }

        output.flush()?;

        Ok(())
    }
}
