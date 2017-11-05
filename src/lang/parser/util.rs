
use super::super::controller::Controller;
use super::super::logger::Logger;
use super::sub_parser::SubParser;
use super::end_parser::EndParser;
use super::var_parser::VarParser;
use super::number_parser::NumberParser;
use super::math_parser::MathParser;
use super::operator_parser::OperatorParser;
use super::io_parser::IoParser;
use super::comment_parser::CommentParser;
use super::file_parser::FileParser;
use super::string_parser::StringParser;
use super::array_parser::ArrayParser;
use super::dict_parser::DictParser;
use super::parser_container::ParserContainer;
use super::char_container::CharContainer;
use super::token_container::TokenContainer;
use super::super::error::Error;

pub fn do_parse_single<T: Logger>(
    c: char,
    parser_data: &mut ParserContainer,
    controller: &mut Controller<T>,
    num_parsers: usize,
    parsers: &mut [Box<SubParser<T>>],
    char_container: &mut CharContainer,
    token_container: &mut TokenContainer,
) -> Result<(bool, bool), Error> {
    for i in 0..num_parsers {
        if parsers[i].check(c) {
            // use parser
            {
                controller.get_logger_mut().parser_in_parser(
                    parsers[i].identify(),
                );
            }
            let mabe_exit = parsers[i].parse(
                controller,
                parser_data,
                char_container,
                token_container,
            )?;

            if mabe_exit {
                return Ok((true, true));
            }

            parsers[i].reset();

            {
                controller.get_logger_mut().parser_out_parser(
                    parsers[i].identify(),
                );
            }
            return Ok((false, true));
        }
    }
    Ok((false, false))
}

pub fn do_parse<T: Logger>(
    parser_data: &mut ParserContainer,
    controller: &mut Controller<T>,
    num_parsers: usize,
    parsers: &mut [Box<SubParser<T>>],
    char_container: &mut CharContainer,
    token_container: &mut TokenContainer,
) -> Result<(), Error> {
    while !parser_data.is_done() {
        let (c, ci, li) = parser_data.get_as_tuple();
        {
            controller.get_logger_mut().parser_next_char(c, ci, li);
        }

        let (exit, used) = do_parse_single(
            c,
            parser_data,
            controller,
            num_parsers,
            parsers,
            char_container,
            token_container,
        )?;

        if exit {
            break;
        }

        if !used {
            parser_data.inc_char();
        }
    }
    Ok(())
}

pub fn top_level_parsers<T: Logger>() -> ([Box<SubParser<T>>; 11], usize) {
    (
        [
            Box::new(EndParser::new()),
            Box::new(VarParser::new()),
            Box::new(OperatorParser::new()),
            Box::new(NumberParser::new()),
            Box::new(MathParser::new()),
            Box::new(IoParser::new()),
            Box::new(CommentParser::new()),
            Box::new(FileParser::new()),
            Box::new(StringParser::new()),
            Box::new(ArrayParser::new()),
            Box::new(DictParser::new()),
        ],
        11,
    )
}

pub fn object_value_parsers<T: Logger>() -> ([Box<SubParser<T>>; 8], usize) {
    (
        [
            Box::new(VarParser::new()),
            Box::new(NumberParser::new()),
            Box::new(MathParser::new()),
            Box::new(CommentParser::new()),
            Box::new(FileParser::new()),
            Box::new(StringParser::new()),
            Box::new(ArrayParser::new()),
            Box::new(DictParser::new()),
        ],
        8,
    )
}
