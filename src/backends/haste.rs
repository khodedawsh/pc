use std::fmt::{self, Display, Formatter};

use crate::error::PasteResult;
use crate::types::PasteClient;
use crate::utils::{override_if_present, serde_url};
use clap::Parser;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub struct Backend {
    #[serde(with = "serde_url")]
    pub url: Url,
}

#[derive(Parser)]
#[command(about = "haste backend")]
pub struct Opt {
    /// Overrides url set in config
    #[arg(short = 'u', long = "url")]
    url: Option<Url>,
}

pub const NAME: &str = "haste";

pub const INFO: &str = r#"Haste backend.
Supports any servers running Haste <https://github.com/seejohnrun/haste-server>.
Official publicly available server for this is <https://hastebin.com/>.

Example config block:

    [servers.hastebin]
    backend = "haste"
    url = "https://hastebin.com/"
"#;

impl PasteClient for Backend {
    fn apply_args(&mut self, args: Vec<String>) -> Result<(), clap::Error> {
        let opt = Opt::try_parse_from(args)?;
        override_if_present(&mut self.url, opt.url);
        Ok(())
    }

    fn paste(&self, data: String) -> PasteResult<Url> {
        let client = Client::new();

        let mut base_url = self.url.clone();

        base_url.set_path("documents");
        let info: HastePasteResponse = client.post(base_url.clone()).body(data).send()?.json()?;

        base_url.set_path(&info.key);
        Ok(base_url)
    }
}

#[derive(Deserialize)]
struct HastePasteResponse {
    key: String,
}

impl Display for Backend {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "haste | {}", self.url)
    }
}
