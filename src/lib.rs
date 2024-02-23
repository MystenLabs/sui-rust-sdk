#![cfg_attr(doc_cfg, feature(doc_cfg))]

pub mod types;

#[cfg(feature = "serde")]
mod _serde {
    pub(crate) type ReadableDisplay =
        ::serde_with::As<::serde_with::IfIsHumanReadable<::serde_with::DisplayFromStr>>;
}
