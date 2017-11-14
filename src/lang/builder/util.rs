
use super::super::controller::Controller;
use super::super::logger::Logger;
use super::super::error::Error;
use super::super::parser::token_container::TokenContainer;
use super::command_container::CommandContainer;
use super::command::Command;

pub fn set_type_registers<T: Logger>(
    controller: &mut Controller<T>,
    token_container: &mut TokenContainer,
    command_container: &mut CommandContainer,
) -> Result<(), Error> {
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
    {
        command_container.add_command(controller, Command::ClearRegisters);
    }
    Ok(())
}
