use crate::langs::OCRAugmentationMap;
use derive_builder::Builder;

#[derive(Builder, Debug)]
struct OCRAugmenter {
    content: Vec<String>,
    modified_content: Vec<String>,
    // List of augmentation to apply
    ocr_augmentation_map: OCRAugmentationMap,
    // Probability of applying an augmentation on a word
    augmentation_proba: f32,
}

impl Augment for OCRAugmenter{
    fn augment()
}
