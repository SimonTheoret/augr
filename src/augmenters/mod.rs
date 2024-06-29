use std::error::Error;

mod keyboard;
mod ocr;

trait Augmenter {
    fn augment(self) -> anyhow::Result<()>;
}
