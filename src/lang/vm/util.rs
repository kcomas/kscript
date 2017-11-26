
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use super::super::controller::Controller;
use super::super::logger::Logger;
use super::super::error::Error;
use super::super::builder::command::{DataType, DataHolder, Comparison};
use super::scope::Scope;
use super::vm_types::{DataContainer, RefMap, RefArray, FunctionArg};
use super::Vm;

pub fn get_tuple_data_type(
    scope: &mut Scope,
    left_reg: usize,
    right_reg: usize,
) -> Result<(DataType, DataType), Error> {
    let left_r = scope.get_ref_holder(left_reg)?;
    let right_r = scope.get_ref_holder(right_reg)?;
    let left_b = left_r.borrow();
    let left = left_b.get_as_data_type();
    let right_b = right_r.borrow();
    let right = right_b.get_as_data_type();
    if left.is_none() || right.is_none() {
        return Err(Error::InvalidMathType);
    }
    Ok((left.unwrap(), right.unwrap()))
}


pub fn holder_deep_copy_conversion(
    scope: &Scope,
    data_holder: &DataHolder,
) -> Result<DataContainer, Error> {
    match *data_holder {
        DataHolder::Var(ref name) => {
            match scope.get_var(name) {
                Some(ref_holder) => Ok(ref_holder.borrow().clone()),
                None => Err(Error::VarNotDeclared),
            }
        }
        DataHolder::Const(ref name) => {
            match scope.get_const(name) {
                Some(ref_holder) => Ok(ref_holder.borrow().clone()),
                None => Err(Error::ConstNotDeclard),
            }
        }
        DataHolder::Anon(ref data_type) => Ok(DataContainer::Scalar(data_type.clone())),
        DataHolder::Array(ref data_holders) => {
            let mut containers: RefArray = Vec::new();
            for item in data_holders.iter() {
                containers.push(Rc::new(
                    RefCell::new(holder_deep_copy_conversion(scope, item)?),
                ));
            }
            Ok(DataContainer::Vector(containers))
        }
        DataHolder::Dict(ref dict) => {
            let mut hash_map: RefMap = HashMap::new();
            for (key, value) in dict {
                hash_map.insert(
                    key.clone(),
                    Rc::new(RefCell::new(holder_deep_copy_conversion(scope, value)?)),
                );
            }
            Ok(DataContainer::Hash(hash_map))
        }
        DataHolder::Math(reg) => {
            match scope.get_register(reg) {
                Some(reg_item) => {
                    match reg_item.to_ref_holder() {
                        Some(ref_holder) => Ok(ref_holder.borrow().clone()),
                        None => Err(Error::InvalidMathAccess),
                    }
                }
                None => Err(Error::InvalidMathAccess),
            }
        }
        DataHolder::Conditional(ref left_data, ref comp, ref right_data) => {
            let b = scope.evaluate_conditional(left_data, comp, right_data)?;
            Ok(DataContainer::Scalar(DataType::Bool(b)))
        }
        DataHolder::Function(ref data_holder_args, ref commands) => {
            let func_args = holder_to_function_args(data_holder_args)?;
            Ok(DataContainer::Function(func_args, commands.clone()))
        }
        _ => Err(Error::CannotDeepCopyType),
    }
}

pub fn conditional_to_parts(
    conditional: &DataHolder,
) -> Result<(&DataHolder, &Comparison, &DataHolder), Error> {
    match *conditional {
        DataHolder::Conditional(ref left, ref cond, ref right) => Ok((left, cond, right)),
        _ => Err(Error::InvalidCondititonalHolder),
    }
}

pub fn holder_to_function_args(
    data_holder_args: &Vec<DataHolder>,
) -> Result<Vec<FunctionArg>, Error> {
    let mut func_args: Vec<FunctionArg> = Vec::new();
    for arg in data_holder_args.iter() {
        let rst = match *arg {
            DataHolder::Var(ref name) => FunctionArg::Var(name.clone()),
            DataHolder::RefVar(ref name) => FunctionArg::RefVar(name.clone()),
            DataHolder::Const(ref name) => FunctionArg::Const(name.clone()),
            DataHolder::RefConst(ref name) => FunctionArg::RefConst(name.clone()),
            _ => return Err(Error::InvalidFunctionArg),
        };
        func_args.push(rst);
    }
    Ok(func_args)
}

pub fn run_function<T: Logger>(
    controller: &mut Controller<T>,
    scope: &mut Scope,
    target: &DataHolder,
    args: &Vec<DataHolder>,
) -> Result<DataContainer, Error> {
    let target = match *target {
        DataHolder::Var(ref name) => {
            match scope.get_var(name) {
                Some(ref_holder) => ref_holder.clone(),
                None => return Err(Error::InvalidFunctionTarget),
            }
        }
        DataHolder::Const(ref name) => {
            match scope.get_const(name) {
                Some(ref_holder) => ref_holder.clone(),
                None => return Err(Error::InvalidFunctionTarget),
            }
        }
        DataHolder::Function(_, _) => {
            Rc::new(RefCell::new(holder_deep_copy_conversion(scope, target)?))
        }
        _ => return Err(Error::InvalidFunctionTarget),
    };
    if let DataContainer::Function(ref data_container_args, ref commands) = *target.borrow() {
        if data_container_args.len() != args.len() {
            return Err(Error::InvalidNumberOfArguments);
        }
        let mut sub_scope = Scope::new(scope.get_id() + 1);
        // match each passed in arg with the function arg type
        for i in 0..args.len() {
            match data_container_args[i] {
                FunctionArg::Var(ref name) => {
                    let rst = Rc::new(RefCell::new(holder_deep_copy_conversion(scope, &args[i])?));
                    sub_scope.set_var(name, rst);
                }
                FunctionArg::RefVar(ref ref_name) => {
                    match args[i] {
                        DataHolder::Var(ref var_name) => {
                            match scope.get_var(var_name) {
                                Some(ref_holder) => sub_scope.set_var(ref_name, ref_holder),
                                None => return Err(Error::InvalidFunctionArgPass),
                            }
                        }
                        DataHolder::Const(_) => return Err(Error::InvalidFunctionArgPass),
                        _ => {
                            let rst = Rc::new(
                                RefCell::new(holder_deep_copy_conversion(scope, &args[i])?),
                            );
                            sub_scope.set_const(ref_name, rst);
                        }
                    }
                }
                FunctionArg::Const(ref name) => {
                    let rst = Rc::new(RefCell::new(holder_deep_copy_conversion(scope, &args[i])?));
                    sub_scope.set_const(name, rst);
                }
                FunctionArg::RefConst(ref ref_name) => {
                    match args[i] {
                        DataHolder::Var(_) => return Err(Error::InvalidFunctionArgPass),
                        DataHolder::Const(ref const_name) => {
                            match scope.get_const(const_name) {
                                Some(ref_holder) => sub_scope.set_const(ref_name, ref_holder),
                                None => return Err(Error::InvalidFunctionArgPass),
                            }
                        }
                        _ => {
                            let rst = Rc::new(
                                RefCell::new(holder_deep_copy_conversion(scope, &args[i])?),
                            );
                            sub_scope.set_const(ref_name, rst);
                        }
                    }
                }
            };
        }
        let mut sub_vm = Vm::new(controller);
        let _ = sub_vm.run(commands, &mut sub_scope)?;
        return Ok(sub_scope.get_last_register_value());
    }
    Err(Error::InvalidFunctionTarget)
}
