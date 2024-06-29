/// How to deal with errors in fallible functions.
#[derive(Debug, Clone, Copy)]
pub(crate) enum ErrorStrategy {
    // Ignore errors and replace them by the given value
    ReplaceError,
    // Return the errors
    ReturnResult,
    // Panic in case of Errors. If no Error, return the result
    Panic,
    // Return the error, unless count >= max, in which case it panics
    ReturnWithTreshold { max: u32, count: u32 },
    // Return the error, unless count >= max, in which case it panics
    ReplaceWithTreshold { max: u32, count: u32 },
}

impl ErrorStrategy {
    pub fn apply<T>(&self, result: anyhow::Result<T>, replacement: Option<T>) -> anyhow::Result<T> {
        match self {
            ErrorStrategy::ReplaceError => Ok(replacement.unwrap()),
            ErrorStrategy::ReturnResult => result,
            ErrorStrategy::Panic => Ok(result.unwrap()),
            ErrorStrategy::ReturnWithTreshold { max, mut count } => match result {
                Ok(val) => Ok(val),
                Err(e) => {
                    count += 1;
                    if count > *max {
                        panic!("Too many errors encountered. Last error: {}", e)
                    }
                    Err(e)
                }
            },
            ErrorStrategy::ReplaceWithTreshold { max, mut count } => match result {
                Ok(valr) => Ok(valr),
                Err(er) => {
                    count += 1;
                    if count > *max {
                        panic!("Too many errors encountered. Last error: {}", er)
                    }
                    Ok(replacement.unwrap())
                }
            },
        }
    }
}
