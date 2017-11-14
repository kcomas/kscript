
pub mod command;

use super::controller::Controller;
use super::logger::Logger;
use super::error::Error;
use super::parser::token_container::TokenContainer;

pub struct BuilderRunner<'a, T: Logger + 'a> {
    controller: &'a mut Controller<T>,
    operator_index: Option<(u64, usize)>,
}

impl<'a, T> BuilderRunner<'a, T>
where
    T: Logger,
{
    pub fn new(controller: &'a mut Controller<T>) -> BuilderRunner<'a, T> {
        BuilderRunner {
            controller: controller,
            operator_index: None,
        }
    }

    pub fn run(&mut self, token_container: &mut TokenContainer) -> Result<(), Error> {
        {
            self.controller.get_logger_mut().builder_start();
        }

        while !token_container.is_done() {
            // check if the token is an operator
            if token_container.is_current_token_end() {
                if let None = self.operator_index {
                    // set token as used and continue
                    token_container.set_current_used();
                } else {
                    // build commands
                    self.operator_index = None;
                }
            } else {
                let token = token_container.get_current_token();
                println!("{:?}", token);

                let presedence = token.is_operator_with_presedence();

                if presedence > 0 {
                    self.update_operator_index(
                        Some((presedence, token_container.current_position())),
                    );
                }
            }

            token_container.inc_token();
        }

        {
            self.controller.get_logger_mut().builder_end();
        }

        Ok(())
    }

    fn update_operator_index(&mut self, idx: Option<(u64, usize)>) {
        if let None = idx {
            self.operator_index = None;
            return;
        }

        if let None = self.operator_index {
            self.operator_index = idx;
            return;
        }

        let (old_presedence, _) = self.operator_index.unwrap();
        let (new_presedence, _) = idx.unwrap();

        if new_presedence > old_presedence {
            self.operator_index = idx;
        }
    }
}
