use std::collections::HashMap;

#[cfg(feature = "fr")]
pub(crate) mod fr_ressources;

#[cfg(feature = "en")]
pub(crate) mod en_ressources;

#[derive(Debug, Clone)]
pub(crate) enum Lang {
    FR,
    EN,
    // ES, someday!
}

pub type KBAugmentationMap = HashMap<&'static str, Vec<&'static str>>;
