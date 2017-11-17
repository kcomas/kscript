
use super::super::controller::Controller;
use super::super::logger::Logger;
use super::super::error::Error;
use super::super::parser::token_container::TokenContainer;
use super::super::parser::token::Token;
use super::command_container::CommandContainer;
use super::command::{Command, DataHolder, DataType};
use super::sub_builder::SubBuilder;
use super::single_command_builder::SingleCommandBuilder;
use super::double_command_builder::DoubleCommandBuilder;
use super::io_builder::IoBuilder;
use super::add_sub_builder::AddSubBuilder;
use super::mul_div_mod_builder::MulDivModBuilder;

pub fn token_to_data_type<T: Logger>(
    controller: &mut Controller<T>,
    command_container: &mut CommandContainer,
    current_register: &mut usize,
    token: &Token,
) -> Result<Option<DataHolder>, Error> {
    match *token {
        Token::Var(ref name) => Ok(Some(DataHolder::Var(name.clone()))),
        Token::Const(ref name) => Ok(Some(DataHolder::Const(name.clone()))),
        Token::String(ref string) => Ok(Some(DataHolder::Anon(DataType::String(string.clone())))),
        Token::Integer(int) => Ok(Some(DataHolder::Anon(DataType::Integer(int)))),
        Token::Float(float) => Ok(Some(DataHolder::Anon(DataType::Float(float)))),
        Token::Array(ref arr) => {
            let mut container: Vec<DataHolder> = Vec::new();
            for token in arr.iter() {
                if let Some(item) = token_to_data_type(
                    controller,
                    command_container,
                    current_register,
                    token,
                )?
                {
                    container.push(item);
                }
            }
            Ok(Some((DataHolder::Array(container))))
        }
        Token::ObjectAccess(ref target, ref accessor) => {
            let t_holder =
                token_to_data_type(controller, command_container, current_register, target)?;
            let a_holder =
                token_to_data_type(controller, command_container, current_register, accessor)?;
            if t_holder.is_some() && a_holder.is_some() {
                return Ok(Some(DataHolder::ObjectAccess(
                    Box::new(t_holder.unwrap()),
                    Box::new(a_holder.unwrap()),
                )));
            }
            Err(Error::UnableToBuildDataType)
        }
        Token::Math(ref math_tokens) => {
            let mut math_container = TokenContainer::from_token_vec(math_tokens.clone());
            let mut math_builders = math_builders();
            create_commands(
                controller,
                &mut math_container,
                command_container,
                current_register,
                &mut math_builders,
            )?;
            Ok(Some(DataHolder::Math(*current_register)))
        }
        _ => Ok(None),
    }
}

pub fn set_type_registers<T: Logger>(
    controller: &mut Controller<T>,
    token_container: &mut TokenContainer,
    command_container: &mut CommandContainer,
    current_register: &mut usize,
) -> Result<(), Error> {
    while token_container.in_slice() {
        if let Some(ref mut token) = token_container.get_slice_token_mut() {
            {
                controller.get_logger_mut().builder_check_token(token);
            }
            if let Some(data_holder) =
                token_to_data_type(controller, command_container, current_register, token)?
            {
                command_container.add_command(
                    controller,
                    Command::SetRegister(*current_register, data_holder),
                );
                token.set_as_register(*current_register);
                *current_register += 1;
            }
        }
        token_container.inc_slice_position();
    }
    Ok(())
}

pub fn set_operator_registers<T: Logger>(
    controller: &mut Controller<T>,
    token_container: &mut TokenContainer,
    command_container: &mut CommandContainer,
    current_register: &mut usize,
    builders: &mut [Box<SubBuilder<T>>],
) -> Result<(), Error> {
    loop {
        let mut highest_presedence: u64 = 0;
        let mut builder_presedence_index: usize = 0;
        let mut token_index: usize = 0;
        while token_container.in_slice() {
            for i in 0..builders.len() {
                if let Some(ref token) = token_container.get_slice_token() {
                    if builders[i].check(token) {
                        let pres = builders[i].presedence();
                        if pres > highest_presedence {
                            highest_presedence = pres;
                            builder_presedence_index = i;
                            token_index = token_container.get_slice_token_index();
                        }
                    }
                } else {
                    return Err(Error::InvalidTokenAccess);
                }
            }
            token_container.inc_slice_position();
        }
        if highest_presedence > 0 {
            token_container.set_slice_token_index(token_index);
            {
                controller.get_logger_mut().builder_in_builder(
                    builders
                        [builder_presedence_index]
                        .identify(),
                );
            }
            builders[builder_presedence_index].build(
                controller,
                token_container,
                command_container,
                current_register,
            )?;
            {
                controller.get_logger_mut().builder_out_builder(
                    builders
                        [builder_presedence_index]
                        .identify(),
                );
            }
            if builders[builder_presedence_index].do_clear() {
                command_container.add_command(controller, Command::ClearRegisters);
                *current_register = 0;
            }
            token_container.reset_slice_position();
        } else {
            break;
        }
    }
    Ok(())
}

pub fn create_commands<T: Logger>(
    controller: &mut Controller<T>,
    token_container: &mut TokenContainer,
    command_container: &mut CommandContainer,
    current_register: &mut usize,
    builders: &mut [Box<SubBuilder<T>>],
) -> Result<(), Error> {
    while !token_container.is_done() {
        let mut run = false;
        let mut use_clear = true;
        if token_container.is_current_token_end() {
            run = true;
            use_clear = true;
        }
        if token_container.is_current_token_last() {
            run = true;
            use_clear = false;
        }
        // check if the token is an operator
        if run {
            token_container.update_slice_end();
            set_type_registers(
                controller,
                token_container,
                command_container,
                current_register,
            )?;
            token_container.reset_slice_position();
            // set operators
            set_operator_registers(
                controller,
                token_container,
                command_container,
                current_register,
                builders,
            )?;
            // check if the last command is a clear
            if !command_container.is_last_clear() && use_clear {
                command_container.add_command(controller, Command::ClearRegisters);
            }
            token_container.set_current_end_as_used();
            // skip the used
            token_container.inc_token();
            token_container.update_slice_start();
        }
        token_container.inc_token();
    }
    Ok(())
}


pub fn top_level_builders<T: Logger>() -> [Box<SubBuilder<T>>; 3] {
    [
        Box::new(SingleCommandBuilder::new()),
        Box::new(DoubleCommandBuilder::new()),
        Box::new(IoBuilder::new()),
    ]
}

pub fn math_builders<T: Logger>() -> [Box<SubBuilder<T>>; 2] {
    [
        Box::new(AddSubBuilder::new()),
        Box::new(MulDivModBuilder::new()),
    ]
}

pub fn get_left_and_right(token_container: &mut TokenContainer) -> Result<(usize, usize), Error> {
    let left_counter;
    let right_counter;

    if let Some(reg_counter) = token_container.get_right_register_and_use() {
        right_counter = reg_counter;
    } else {
        return Err(Error::InvalidRightRegisterAccess);
    }

    if let Some(reg_counter) = token_container.get_left_register_and_use() {
        left_counter = reg_counter;
    } else {
        return Err(Error::InvalidLeftRegisterAccess);
    }

    Ok((left_counter, right_counter))
}
