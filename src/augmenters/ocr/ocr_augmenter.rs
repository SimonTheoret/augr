use crate::augmenters::Augmenter;
use crate::augmenters::CharAugmenter;

#[derive(Debug, Clone)]
struct OCRAugmenter {
    augmenter: CharAugmenter,
}

impl std::ops::Deref for OCRAugmenter {
    type Target = CharAugmenter;
    fn deref(&self) -> &Self::Target {
        &(self.augmenter)
    }
}
impl Augmenter for OCRAugmenter {
    fn augment(&mut self) -> anyhow::Result<()> {
        self.augmenter.augment()
    }
    fn parallel_augment(&mut self) -> anyhow::Result<()> {
        self.augmenter.augment()
    }
}
