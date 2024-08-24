use std::fmt::{self, Display, Formatter};

use reqwest::blocking::{Client, multipart::Form};

use serde::{Deserialize, Serialize};
use structopt::StructOpt;
use url::Url;

use crate::error::PasteResult;
use crate::types::PasteClient;
use crate::utils::{override_if_present, override_option_with_option_none, serde_url};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub struct Backend {
    #[serde(with = "serde_url")]
    pub url: Url,
    pub syntax: Option<String>,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "sprunge backend")]
#[structopt(template = "{about}\n\nUSAGE:\n    {usage}\n\n{all-args}")]
pub struct Opt {
    /// Overrides url set in config
    #[structopt(short = "u", long = "url")]
    url: Option<Url>,

    /// Filetype for syntax highlighting
    #[structopt(short = "s", long = "syntax", value_name = "filetype|NONE")]
    syntax: Option<String>,
}

pub const NAME: &str = "sprunge";

pub const INFO: &str = r#"Sprunge backend.
Supports any servers running sprunge <https://github.com/rupa/sprunge>.

Example config block:

    [servers.sprunge]
    backend = "sprunge"
    url = "http://sprunge.us/"

    # Optional values

    # Filetype for syntax highlighting. Default is plain text. A custom syntax set also marks up
    # the content with html - not suitable for curl'ing as raw text.
    syntax = "py"
"#;

impl PasteClient for Backend {
    fn apply_args(&mut self, args: Vec<String>) -> clap::Result<()> {
        let opt = Opt::from_iter_safe(args)?;
        override_if_present(&mut self.url, opt.url);
        override_option_with_option_none(&mut self.syntax, opt.syntax);
        Ok(())
    }

    fn paste(&self, data: String) -> PasteResult<Url> {
        let form = Form::new().text("sprunge", data);
        let text = Client::new()
            .post(self.url.clone())
            .multipart(form)
            .send()?
            .text()?;
        let mut url = Url::parse(&text)?;
        if let Some(ref lang) = self.syntax {
            url.set_query(Some(lang));
        }
        Ok(url)
    }
}

impl Display for Backend {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "sprunge | {}", self.url)
    }
}
