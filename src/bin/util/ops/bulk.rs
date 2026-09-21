use clap::Args;
use open_library_client::{Client, CoreApi, core::types::SearchWork};
use std::{fs::File, io::Write, time::Duration};

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
