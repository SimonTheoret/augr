use crate::error_strategy::ErrorStrategy;
use anyhow::anyhow;
use itertools::Itertools;
use rand::Rng;
use rayon::iter::IntoParallelIterator;
use rayon::prelude::*;
use std::collections::HashMap;
use std::error::Error;
use unicode_segmentation::UnicodeSegmentation;

pub(crate) mod keyboard;
pub(crate) mod ocr;

pub(crate) trait Augmenter<'a, PI, I>
where
    PI: IntoParallelIterator<Item = &'a str>,
    I: IntoIterator<Item = &'a str>,
{
    fn augment(&self, content: I) -> Vec<Result<String, Box<dyn Error + Send>>>;
    fn parallel_augment(&self, content: PI) -> Vec<Result<String, Box<dyn Error + Send>>>;
}

/// Structure capable of augmenting `content` at the character level. To be more precise, this
/// struct acts at the (grapheme) cluster level.
#[derive(Debug, Clone)]
pub(crate) struct CharAugmenter {
    // content: Vec<String>,
    // pub modified_content: Vec<String>,

    // List of augmentation to apply
    augmentation_map: HashMap<&'static str, Vec<&'static str>>,
    // Probability of applying an augmentation on a word
    augmentation_proba: f32,
    error_strategy: ErrorStrategy,
}

impl<'a, PI, I> Augmenter<'a, PI, I> for CharAugmenter
where
    PI: IntoParallelIterator<Item = &'a str>,
    I: IntoIterator<Item = &'a str>,
{
    /// Overwrites the content of the augmenter and replace the modified_content attribute with the
    /// newly modified content.
    fn augment(&self, content: I) -> Vec<Result<String, Box<dyn Error + Send>>> {
        let modif = content
            .into_iter()
            .map(|v| self.apply_augmentation(v))
            .collect_vec();
        modif
    }

    /// Overwrites the content of the augmenter and replace the modified_content attribute with the
    /// newly modified content.
    fn parallel_augment(&self, content: PI) -> Vec<Result<String, Box<dyn Error + Send>>> {
        let iter = content.into_par_iter();
        let modif: Vec<Result<String, Box<dyn Error + Send>>> =
            iter.map(|v| self.apply_augmentation(&v)).collect();
        modif
    }
}

impl CharAugmenter {
    fn apply_augmentation(&self, example: &str) -> Result<String, Box<dyn Error + Send>> {
        let mut current_example = String::from("");
        for cluster in UnicodeSegmentation::graphemes(example, true) {
            let mut rng = rand::thread_rng();
            let rand_num: f32 = rng.gen();
            if rand_num > 1. - self.augmentation_proba && self.cluster_in_augmentation_map(cluster)
            {
                let replacement = self.pick_replacement(&mut rng);
                let net_replacement = self.error_strategy.apply(replacement, Some(cluster));

                current_example.push_str(net_replacement?);
            } else {
                current_example.push_str(cluster);
            }
        }
        Ok(current_example)
    }

    fn cluster_in_augmentation_map(&self, cluster: &str) -> bool {
        self.augmentation_map.keys().any(|x| *x == cluster)
    }

    fn pick_replacement(&self, rng: &mut rand::rngs::ThreadRng) -> anyhow::Result<&str> {
        let count = self.augmentation_map.len();
        let mut picks: Vec<i32> = Vec::with_capacity(count);
        rng.fill(&mut *picks);
        let argmax = picks
            .into_iter()
            .position_max()
            .ok_or(anyhow!("Argmax could not be found when picking the key for the augmentation map. Is the kb augmentation map correctly initialized?"))?;
        let key = self.augmentation_map
            .keys()
            .nth(argmax)
            .ok_or(anyhow!("Argmax key could not be selected in the kb augmentation map. Is kb augmentation map correctly initialized?"))?;
        let key_count = &self.augmentation_map[key];
        let mut key_picks: Vec<i32> = Vec::with_capacity(key_count.len());
        rng.fill(&mut *key_picks);
        let key_argmax = key_picks.into_iter().position_max().ok_or(anyhow!(
            "Argmax could not be selected in the replacement for the given key: {:?}",
            key
        ))?;
        Ok(key_count[key_argmax])
    }
    pub fn new(
        augmentation_map: HashMap<&'static str, Vec<&'static str>>,
        augmentation_proba: f32,
        error_strategy: ErrorStrategy,
    ) -> CharAugmenter {
        CharAugmenter {
            augmentation_map,
            augmentation_proba,
            error_strategy,
        }
    }
}
