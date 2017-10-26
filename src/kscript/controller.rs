
use std::cell::RefCell;
use std::cell::RefMut;
use super::logger::Logger;

#[derive(Debug)]
pub struct Controller<T: Logger> {
    logger: RefCell<T>,
}

impl<T> Controller<T>
where
    T: Logger,
{
    pub fn new(logger: T) -> Controller<T> {
        Controller { logger: RefCell::new(logger) }
    }

    pub fn get_logger_mut(&mut self) -> RefMut<T> {
        self.logger.borrow_mut()
    }
}
