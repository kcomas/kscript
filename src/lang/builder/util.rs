
use super::super::controller::Controller;
use super::super::logger::Logger;
use super::super::error::Error;
use super::super::parser::token_container::TokenContainer;
use super::command_container::CommandContainer;
use super::command::Command;
use super::sub_builder::SubBuilder;
use super::single_command_builder::SingleCommandBuilder;
use super::double_command_builder::DoubleCommandBuilder;
use super::io_builder::IoBuilder;

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
            if let Some(data_holder) = token.to_data_holder() {
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

pub fn top_level_builders<T: Logger>() -> [Box<SubBuilder<T>>; 3] {
    [
        Box::new(SingleCommandBuilder::new()),
        Box::new(DoubleCommandBuilder::new()),
        Box::new(IoBuilder::new()),
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
