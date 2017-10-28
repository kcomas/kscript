
use std::cell::RefCell;
use std::cell::RefMut;
use super::logger::Logger;
use super::error::Error;

#[derive(Debug)]
pub struct Controller<T: Logger> {
    logger: RefCell<T>,
    error: Option<Error>,
}

impl<T> Controller<T>
where
    T: Logger,
{
    pub fn new(logger: T) -> Controller<T> {
        Controller {
            logger: RefCell::new(logger),
            error: None,
        }
    }

    pub fn get_logger_mut(&mut self) -> RefMut<T> {
        self.logger.borrow_mut()
    }
}
