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

impl Lang {
    fn to_str(&self) -> &'static str {
        std::stringify!(&self)
    }
}
impl std::fmt::Display for Lang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, std::stringify!(&self))
    }
}

pub type KBAugmentationMap = HashMap<&'static str, Vec<&'static str>>;
pub type OCRAugmentationMap = HashMap<&'static str, Vec<&'static str>>;
