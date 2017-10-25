
use super::logger::Logger;

#[derive(Debug)]
pub struct Controller<T: Logger> {
    logger: T,
}

impl<T> Controller<T>
where
    T: Logger,
{
    pub fn new(logger: T) -> Controller<T> {
        Controller { logger: logger }
    }
}
