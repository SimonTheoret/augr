use crate::langs::Lang;
use anyhow::anyhow;
use derive_builder::Builder;
use itertools::Itertools;
use rand::Rng;
use regex::Regex;
use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

type KBAugmentationMap = HashMap<&'static str, Vec<&'static str>>;

#[derive(Builder, Debug)]
struct KeyboardAugmenter {
    content: Vec<String>,
    modified_content: Vec<String>,
    // List of augmentation to apply
    kb_augmentation_map: KBAugmentationMap,
    // Probability of applying an augmentation on a word
    augmentation_proba: f32,
}

impl KeyboardAugmenter {
    fn augment(mut self) -> Self {
        self.modified_content = vec![];
        for example in &self.content {
            let mut current_example = String::from("");
            for (i, cluster) in UnicodeSegmentation::grapheme_indices(example.as_str(), true) {
                let mut rng = rand::thread_rng();
                let rand_num: f32 = rng.gen();
                if rand_num > 1. - self.augmentation_proba
                    && self.cluster_in_augmentation_map(cluster)
                {
                    let replacement = self.pick_replacement(&mut rng);
                    match replacement {
                        Ok(modification) => {
                            current_example.push_str(modification);
                        }
                        Err(err) => {
                            current_example.push_str(cluster);
                            eprintln!("{}", err);
                        }
                    }
                } else {
                    current_example.push_str(cluster);
                }
            }
            self.modified_content.push(current_example)
        }
        self
    }

    fn cluster_in_augmentation_map(&self, cluster: &str) -> bool {
        self.kb_augmentation_map.keys().any(|x| *x == cluster)
    }
    fn pick_replacement(&self, rng: &mut rand::rngs::ThreadRng) -> anyhow::Result<&str> {
        let count = self.kb_augmentation_map.len();
        let mut picks: Vec<i32> = Vec::with_capacity(count);
        rng.fill(&mut *picks);
        let argmax = picks.into_iter().position_max().ok_or(anyhow!("Argmax could not be found when picking the key for the augmentation map. Is the kb augmentation map correctly initialized?"))?;
        let key = self.kb_augmentation_map.keys().nth(argmax).ok_or(anyhow!("Argmax key could not be selected in the kb augmentation map. Is kb augmentation map correctly initialized?"))?;
        let key_count = &self.kb_augmentation_map[key];
        let mut key_picks: Vec<i32> = Vec::with_capacity(key_count.len());
        rng.fill(&mut *key_picks);
        let key_argmax = key_picks.into_iter().position_max().ok_or(anyhow!(
            "Argmax could not be selected in the replacement for the given key:  {:?}",
            key
        ))?;
        Ok(key_count[key_argmax])
    }
}
