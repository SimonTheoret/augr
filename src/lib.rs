mod augmenters;
mod error_strategy;
mod langs;
use crate::augmenters::keyboard::keyboard_augmenter::KeyboardAugmenter;
use crate::augmenters::ocr::ocr_augmenter::OCRAugmenter;
use crate::augmenters::Augmenter;
use crate::augmenters::CharAugmenter;
use crate::error_strategy::ErrorStrategy;
use crate::langs::Lang;
use rayon::iter::IntoParallelIterator;
use std::error::Error;
use std::result::Result;

pub fn augment_kb<'a, I, PI>(
    content: I,
    lang: Lang,
    proba: f32,
    err_strategy: ErrorStrategy,
) -> Vec<Result<String, Box<dyn Error + Send>>>
where
    I: IntoIterator<Item = &'a str>,
    PI: IntoParallelIterator<Item = &'a str>,
{
    let aug_map = lang.keyboard_map();
    let char_augmenter: CharAugmenter = CharAugmenter::new(aug_map, proba, err_strategy);
    let kb_augmenter: KeyboardAugmenter = KeyboardAugmenter::new(char_augmenter);
    <KeyboardAugmenter as Augmenter<'_, PI, I>>::augment(&kb_augmenter, content)
}

#[cfg(feature = "parallel")]
pub fn augment_kb_parallel<'a, I, PI>(
    content: PI,
    lang: Lang,
    proba: f32,
    err_strategy: ErrorStrategy,
) -> Vec<Result<String, Box<dyn Error + Send>>>
where
    PI: IntoParallelIterator<Item = &'a str>,
    I: IntoIterator<Item = &'a str>,
{
    let aug_map = lang.keyboard_map();
    let char_augmenter: CharAugmenter = CharAugmenter::new(aug_map, proba, err_strategy);
    let kb_augmenter: KeyboardAugmenter = KeyboardAugmenter::new(char_augmenter);
    <KeyboardAugmenter as Augmenter<'_, PI, I>>::parallel_augment(&kb_augmenter, content)
}

pub fn augment_ocr<'a, I, PI>(
    content: I,
    lang: Lang,
    proba: f32,
    err_strategy: ErrorStrategy,
) -> Vec<Result<String, Box<dyn Error + Send>>>
where
    I: IntoIterator<Item = &'a str>,
    PI: IntoParallelIterator<Item = &'a str>,
{
    let aug_map = lang.keyboard_map();
    let char_augmenter: CharAugmenter = CharAugmenter::new(aug_map, proba, err_strategy);
    let ocr_augmenter: OCRAugmenter = OCRAugmenter::new(char_augmenter);
    <OCRAugmenter as Augmenter<'_, PI, I>>::augment(&ocr_augmenter, content)
}

#[cfg(feature = "parallel")]
pub fn augment_ocr_parallel<'a, I, PI>(
    content: PI,
    lang: Lang,
    proba: f32,
    err_strategy: ErrorStrategy,
) -> Vec<Result<String, Box<dyn Error + Send>>>
where
    PI: IntoParallelIterator<Item = &'a str>,
    I: IntoIterator<Item = &'a str>,
{
    let aug_map = lang.keyboard_map();
    let char_augmenter: CharAugmenter = CharAugmenter::new(aug_map, proba, err_strategy);
    let ocr_augmenter: OCRAugmenter = OCRAugmenter::new(char_augmenter);
    <OCRAugmenter as Augmenter<'_, PI, I>>::parallel_augment(&ocr_augmenter, content)
}

#[cfg(test)]
mod tests {}
