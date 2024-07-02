use crate::augmenters::{Augmenter, CharAugmenter};
use rayon::iter::IntoParallelIterator;
use std::error::Error;

#[derive(Debug, Clone)]
pub(crate) struct KeyboardAugmenter {
    augmenter: CharAugmenter,
}

impl KeyboardAugmenter {
    pub(crate) fn new(augmenter: CharAugmenter) -> KeyboardAugmenter {
        KeyboardAugmenter { augmenter }
    }
}

impl std::ops::Deref for KeyboardAugmenter {
    type Target = CharAugmenter;
    fn deref(&self) -> &Self::Target {
        &self.augmenter
    }
}
impl<'a, PI, I> Augmenter<'a, PI, I> for KeyboardAugmenter
where
    PI: IntoParallelIterator<Item = &'a str>,
    I: IntoIterator<Item = &'a str>,
{
    fn augment(&self, content: I) -> Vec<Result<String, Box<dyn Error + Send>>> {
        <CharAugmenter as Augmenter<'_, PI, I>>::augment(&self.augmenter, content)
    }
    fn parallel_augment(&self, content: PI) -> Vec<Result<String, Box<dyn Error + Send>>> {
        <CharAugmenter as Augmenter<'_, PI, I>>::parallel_augment(&self.augmenter, content)
    }
}
