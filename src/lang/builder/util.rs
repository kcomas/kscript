
use super::super::controller::Controller;
use super::super::logger::Logger;
use super::super::error::Error;
use super::super::parser::token_container::TokenContainer;
use super::command_container::CommandContainer;
use super::command::Command;
use super::sub_builder::SubBuilder;

pub fn set_type_registers<T: Logger>(
    controller: &mut Controller<T>,
    token_container: &mut TokenContainer,
    command_container: &mut CommandContainer,
) -> Result<usize, Error> {
    let mut current_register: usize = 0;
    while token_container.in_slice() {
        if let Some(ref mut token) = token_container.get_slice_token_mut() {
            {
                controller.get_logger_mut().builder_check_token(token);
            }
            if let Some(data_holder) = token.to_data_holder() {
                {
                    command_container.add_command(
                        controller,
                        Command::SetRegister(current_register, data_holder),
                    );
                }
                token.set_as_register(current_register);
                current_register += 1;
            }
        }
        token_container.inc_slice_position();
    }
    Ok(current_register)
}

pub fn set_operator_registers<T: Logger>(
    controller: &mut Controller<T>,
    token_container: &mut TokenContainer,
    command_container: &mut CommandContainer,
    current_register: &mut usize,
    builders: &mut [Box<SubBuilder<T>>],
    num_builders: usize,
) -> Result<(), Error> {
    while token_container.in_slice() {
        let mut highest_presedence: u64 = 0;
        let mut builder_presedence_index: usize = 0;
        for i in 0..num_builders {
            if let Some(ref token) = token_container.get_slice_token() {
                if builders[i].check(token) {
                    let pres = builders[i].presedence();
                    if pres > highest_presedence {
                        highest_presedence = pres;
                        builder_presedence_index = i;
                    }
                }
            } else {
                return Err(Error::InvalidTokenAccess);
            }
        }
        if highest_presedence > 0 {
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
        } else {
            token_container.inc_slice_position();
        }
    }
    Ok(())
}
