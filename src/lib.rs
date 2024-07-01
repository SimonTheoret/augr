mod augmenters;
mod error_strategy;
mod langs;
use crate::augmenters::Augmenter;
use crate::augmenters::CharAugmenter;
use crate::error_strategy::ErrorStrategy;
use crate::langs::Lang;
use std::error::Error;
use std::result::Result;

pub fn augment_kb(
    content: Vec<String>,
    lang: Lang,
    proba: f32,
    err_strategy: ErrorStrategy,
) -> Result<Vec<String>, Box<dyn Error>> {
    let mut builder = CharAugmenterBuilder::default();
    let modif = vec![];
    let aug_map = lang.keyboard_map();
    let mut char_augmenter: CharAugmenter = builder
        .content(content)
        .modified_content(modif)
        .augmentation_map(aug_map)
        .augmentation_proba(proba)
        .error_strategy(err_strategy)
        .build()
        .unwrap();
    // let mut char_augmenter: CharAugmenter = CharAugmenter{
    //     content,
    //     modified_content: modif,augmentation_map: aug_map, augmentation_proba: proba, error_strategy: err_strategy
    //     }
    char_augmenter.augment()?;
    Ok(char_augmenter.modified_content)
}
// #[cfg(feature = "parallel")]
pub fn augment_kb_parallel(
    content: Vec<String>,
    lang: Lang,
    proba: f32,
    err_strategy: ErrorStrategy,
) -> Result<Vec<String>, Box<dyn Error>> {
    let mut builder = CharAugmenterBuilder::default();
    let modif = vec![];
    let aug_map = lang.keyboard_map();
    let mut char_augmenter: CharAugmenter = builder
        .content(content)
        .modified_content(modif)
        .augmentation_map(aug_map)
        .augmentation_proba(proba)
        .error_strategy(err_strategy)
        .build()
        .unwrap();
    char_augmenter.augment()?;
    Ok(char_augmenter.modified_content)
}

#[cfg(test)]
mod tests {}
