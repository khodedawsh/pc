use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};
use serde::{Deserializer, Serializer};
use url::Url;

use crate::backends::{dpaste_com, fiche, generic, haste, modern_paste, vpaste};
pub use crate::types::PasteClient;

pub fn deserialize_url<'de, D>(d: D) -> Result<Url, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(d)?;

    match Url::parse(&s) {
        Ok(u) => Ok(u),
        Err(_) => Err(serde::de::Error::custom(format!(
            "Could not parse {:?} as a url",
            s
        ))),
    }
}

pub fn serialize_url<S>(x: &Url, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(x.as_str())
}

/// Provides a paste client implementation given config
pub fn build_client(
    config: BackendConfig,
    override_args: Vec<String>,
) -> clap::Result<Box<dyn PasteClient>> {
    match config {
        BackendConfig::Generic(backend) => backend.apply_args(override_args),
        BackendConfig::Haste(backend) => backend.apply_args(override_args),
        BackendConfig::Vpaste(backend) => backend.apply_args(override_args),
        BackendConfig::Fiche(backend) => backend.apply_args(override_args),
        BackendConfig::ModernPaste(backend) => backend.apply_args(override_args),
        BackendConfig::DpasteCom(backend) => backend.apply_args(override_args),
    }
}

pub const BACKEND_NAMES: &'static [&'static str] = &[
    generic::NAME,
    haste::NAME,
    vpaste::NAME,
    fiche::NAME,
    modern_paste::NAME,
    dpaste_com::NAME,
];

pub fn info_from_str(name: &str) -> Result<&'static str, String> {
    match name {
        generic::NAME => Ok(generic::info()),
        haste::NAME => Ok(haste::info()),
        vpaste::NAME => Ok(vpaste::info()),
        fiche::NAME => Ok(fiche::info()),
        modern_paste::NAME => Ok(modern_paste::info()),
        dpaste_com::NAME => Ok(dpaste_com::info()),
        s => Err(format!("{} is not a valid backend", s)),
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "backend")]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub enum BackendConfig {
    Generic(generic::Backend),
    Haste(haste::Backend),
    Vpaste(vpaste::Backend),
    Fiche(fiche::Backend),
    ModernPaste(modern_paste::Backend),
    DpasteCom(dpaste_com::Backend),
}

impl Display for BackendConfig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            BackendConfig::Generic(generic::Backend { url }) => write!(f, "generic | {}", url),
            BackendConfig::Haste(haste::Backend { url }) => write!(f, "haste | {}", url),
            BackendConfig::Vpaste(vpaste::Backend { url }) => write!(f, "vpaste | {}", url),
            BackendConfig::DpasteCom(dpaste_com::Backend { url }) => {
                write!(f, "dpaste_com | {}", url)
            }
            BackendConfig::Fiche(fiche::Backend { domain, port }) => {
                write!(f, "fiche | {}:{}", domain, port)
            }
            BackendConfig::ModernPaste(modern_paste::Backend { url, .. }) => {
                write!(f, "modern_paste | {}", url)
            }
        }
    }
}
