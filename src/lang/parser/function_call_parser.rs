
use super::token::Token;
use super::token_container::TokenContainer;
use super::char_container::CharContainer;
use super::parser_container::ParserContainer;
use super::sub_parser::SubParser;
use super::var_parser::VarParser;
use super::super::logger::Logger;
use super::super::controller::Controller;
use super::super::error::Error;
use super::util::do_parse_single;

pub enum FunctionCallParserState {
    Nothing,
    MabeArguments,
    LoadArguments,
}

pub struct FunctionCallParser {
    state: FunctionCallParserState,
    arg_container: TokenContainer,
}

impl FunctionCallParser {
    pub fn new() -> FunctionCallParser {
        FunctionCallParser {
            state: FunctionCallParserState::Nothing,
            arg_container: TokenContainer::new(),
        }
    }
}
