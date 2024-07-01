use en_ressources::keyboard_en;
use fr_ressources::keyboard_fr;
use std::collections::HashMap;

#[cfg(feature = "fr")]
pub(crate) mod fr_ressources;

#[cfg(feature = "en")]
pub(crate) mod en_ressources;

#[derive(Debug, Clone)]
pub enum Lang {
    FR,
    EN,
    // ES, someday!
}

impl Lang {
    pub(crate) fn keyboard_map(&self) -> KBAugmentationMap {
        match self {
            Lang::FR => keyboard_fr(),
            Lang::EN => keyboard_en(),
        }
    }
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
