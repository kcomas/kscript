
mod controller;
pub mod logger;
pub mod parser;
pub mod builder;
mod error;
mod util;

use self::controller::Controller;
use self::logger::Logger;
use self::parser::ParserRunner;
use self::builder::BuilderRunner;
use self::parser::token_container::TokenContainer;
use self::parser::token::Token;
use self::builder::command::Command;
use self::error::Error;
use self::util::load_file_to_string;

pub struct Kscript<T: Logger> {
    controller: Controller<T>,
    tokens: Vec<Token>,
    commands: Vec<Command>,
}

impl<T> Kscript<T>
where
    T: Logger,
{
    pub fn new(logger: T) -> Kscript<T> {
        Kscript {
            controller: Controller::new(logger),
            tokens: Vec::new(),
            commands: Vec::new(),
        }
    }

    pub fn get_tokens(&self) -> &Vec<Token> {
        &self.tokens
    }

    pub fn get_commands(&self) -> &Vec<Command> {
        &self.commands
    }

    pub fn run(&mut self, text_str: &str) -> Result<(), Error> {
        self.run_build_tokens_commands(text_str)?;
        Ok(())
    }

    pub fn run_file(&mut self, file_name: &str) -> Result<(), Error> {
        match load_file_to_string(file_name) {
            Ok(ref file_string) => self.run(file_string),
            Err(file_error) => Err(Error::FileLoadFail(file_error)),
        }
    }

    pub fn run_build_tokens(&mut self, text_str: &str) -> Result<(), Error> {
        let mut parser_runner = ParserRunner::new(&mut self.controller);
        self.tokens.clear();
        parser_runner.run(text_str, &mut self.tokens)?;
        Ok(())
    }

    pub fn run_build_tokens_commands(&mut self, text_str: &str) -> Result<(), Error> {
        self.run_build_tokens(text_str)?;
        {
            let mut builder_runner = BuilderRunner::new(&mut self.controller);
            let mut token_container = TokenContainer::new(&mut self.tokens);
            self.commands.clear();
            builder_runner.run(&mut token_container, &mut self.commands)?;
        }
        Ok(())
    }
}
