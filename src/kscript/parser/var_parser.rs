
use super::token::Token;
use super::parser_container::ParserContainer;
use super::sub_parser::SubParser;
use super::super::logger::Logger;
use super::super::controller::Controller;
use super::super::error::Error;

pub enum VarParserState {
    Nothing,
    Variable,
    Constant,
}

pub struct VarParser {
    state: VarParserState,
}

impl VarParser {
    pub fn new() -> VarParser {
        VarParser { state: VarParserState::Nothing }
    }
}
