use derive_builder::Builder;

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
