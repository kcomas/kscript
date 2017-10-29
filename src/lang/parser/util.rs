
use super::super::controller::Controller;
use super::super::logger::Logger;
use super::sub_parser::SubParser;
use super::parser_container::ParserContainer;
use super::char_container::CharContainer;
use super::token_container::TokenContainer;
use super::super::error::Error;

pub fn do_parse<T: Logger>(
    text_str: &str,
    controller: &mut Controller<T>,
    num_parsers: usize,
    parsers: &mut [Box<SubParser<T>>],
    char_container: &mut CharContainer,
) -> Result<TokenContainer, Error> {

    let mut token_container = TokenContainer::new();

    let mut parser_data = ParserContainer::new(text_str);

    while !parser_data.is_done() {
        let mut used = false;
        let (c, ci, li) = parser_data.get_as_tuple();
        {
            controller.get_logger_mut().parser_next_char(c, ci, li);
        }

        for i in 0..num_parsers {
            if parsers[i].check(c) {
                // use parser
                {
                    controller.get_logger_mut().parser_in_parser(
                        parsers[i].identify(),
                    );
                }
                let rst = parsers[i].parse(
                    controller,
                    &mut parser_data,
                    char_container,
                    &mut token_container,
                );

                if let Err(kerror) = rst {
                    return Err(kerror);
                }
                {
                    controller.get_logger_mut().parser_out_parser(
                        parsers[i].identify(),
                    );
                }
                used = true;
                break;
            }
        }
        if !used {
            parser_data.inc_char();
        }
    }

    Ok(token_container)
}
