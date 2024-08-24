use std::fmt::{self, Display, Formatter};

use reqwest::blocking::{multipart::Form, Client};

use serde::{Deserialize, Serialize};
use structopt::StructOpt;
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

#[derive(Debug, StructOpt)]
#[structopt(about = "vpaste backend")]
#[structopt(template = "{about}\n\nUSAGE:\n    {usage}\n\n{all-args}")]
pub struct Opt {
    /// Overrides url set in config
    #[structopt(short = "u", long = "url")]
    url: Option<Url>,
}

pub const NAME: &str = "vpaste";

pub const INFO: &str = r#"Vpaste backend.
Supports any servers running vpaste <http://vpaste.net/>.

Example config block:

    [servers.vp]
    backend = "vpaste"
    url = "http://vpaste.net/"
"#;

impl PasteClient for Backend {
    fn apply_args(&mut self, args: Vec<String>) -> clap::Result<()> {
        let opt = Opt::from_iter_safe(args)?;
        override_if_present(&mut self.url, opt.url);
        Ok(())
    }

    fn paste(&self, data: String) -> PasteResult<Url> {
        let form = Form::new().text("text", data);
        let res = Client::new()
            .post(self.url.clone())
            .multipart(form)
            .send()?;
        Ok(res.url().to_owned())
    }
}

impl Display for Backend {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "vpaste | {}", self.url)
    }
}
