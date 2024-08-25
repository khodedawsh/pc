use std::fmt::{self, Display, Formatter};

use clap::Parser;
use reqwest::blocking::{multipart::Form, Client};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::error::PasteResult;
use crate::types::PasteClient;
use crate::utils::{override_if_present, serde_url};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub struct Backend {
    #[serde(with = "serde_url")]
    pub url: Url,
}
#[derive(Parser)]
#[command(version, about = "pipfi backend", long_about = None)]
pub struct Opt {
    /// Overrides url set in config
    #[arg(short = 'u', long = "url")]
    url: Option<Url>,
}

pub const NAME: &str = "pipfi";

pub const INFO: &str = r#"Pipfi backend.
Supports <http://p.ip.fi/>.

Example config block:

    [servers.pip]
    backend = "pipfi"
    url = "http://p.ip.fi/"
"#;

impl PasteClient for Backend {
    fn apply_args(&mut self, args: Vec<String>) -> Result<(), clap::Error> {
        let opt = Opt::try_parse_from(args)?;
        override_if_present(&mut self.url, opt.url);
        Ok(())
    }

    fn paste(&self, data: String) -> PasteResult<Url> {
        let form = Form::new().text("paste", data);
        let text = Client::new()
            .post::<reqwest::Url>(reqwest::Url::from(self.url.clone()))
            .multipart(form)
            .send()?
            .text()?;
        let url = Url::parse(&text)?;
        Ok(url)
    }
}

impl Display for Backend {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "pipfi | {}", self.url)
    }
}
