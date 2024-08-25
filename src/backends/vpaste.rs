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
#[command(version, about = "vpaste backend", long_about = None)]
pub struct Opt {
    /// Overrides url set in config
    #[arg(short = 'u', long = "url")]
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
    fn apply_args(&mut self, args: Vec<String>) -> Result<(), clap::Error> {
        let opt = Opt::try_parse_from(args.iter())?;
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
