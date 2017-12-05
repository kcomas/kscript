
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use super::super::controller::Controller;
use super::super::logger::Logger;
use super::super::error::Error;
use super::super::builder::command::{DataType, DataHolder, Comparison};
use super::scope::Scope;
use super::vm_types::{DataContainer, RefHolder, RefMap, RefArray, FunctionArg};
use super::Vm;

pub fn unwrap_reference_to_type(container: &DataContainer) -> Option<DataType> {
    println!("{:?}", container);
    match *container {
        DataContainer::Scalar(ref data_type) => Some(data_type.clone()),
        DataContainer::Reference(ref reference) => {
            match *reference.borrow() {
                DataContainer::Scalar(ref data_type) => Some(data_type.clone()),
                DataContainer::Reference(ref sub_ref) => unwrap_reference_to_type(
                    &*sub_ref.borrow(),
                ),
                _ => None,
            }
        }
        _ => None,
    }
}

pub fn get_tuple_data_type(
    scope: &mut Scope,
    left_reg: usize,
    right_reg: usize,
) -> Result<(DataType, DataType), Error> {
    let left_r = scope.get_ref_holder(left_reg)?;
    let right_r = scope.get_ref_holder(right_reg)?;
    let left_b = left_r.borrow();
    let left;
    if left_b.is_reference() {
        left = unwrap_reference_to_type(&*left_b);
    } else {
        left = left_b.get_as_data_type();
    }
    let right_b = right_r.borrow();
    let right;
    if right_b.is_reference() {
        right = unwrap_reference_to_type(&*right_b);
    } else {
        right = right_b.get_as_data_type();
    }
    if left.is_none() || right.is_none() {
        return Err(Error::InvalidMathType);
    }
    Ok((left.unwrap(), right.unwrap()))
}


pub fn holder_deep_copy_conversion<T: Logger>(
    controller: &mut Controller<T>,
    scope: &mut Scope,
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
                containers.push(Rc::new(RefCell::new(
                    holder_deep_copy_conversion(controller, scope, item)?,
                )));
            }
            Ok(DataContainer::Vector(containers))
        }
        DataHolder::Dict(ref dict) => {
            let mut hash_map: RefMap = HashMap::new();
            for (key, value) in dict {
                hash_map.insert(
                    key.clone(),
                    Rc::new(RefCell::new(
                        holder_deep_copy_conversion(controller, scope, value)?,
                    )),
                );
            }
            Ok(DataContainer::Hash(hash_map))
        }
        DataHolder::ObjectAccess(ref target, ref accessor) => {
            let rst = access_object(controller, scope, target, accessor)?;
            let copy = rst.borrow().clone();
            Ok(copy)
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
            let b = scope.evaluate_conditional(
                controller,
                left_data,
                comp,
                right_data,
            )?;
            Ok(DataContainer::Scalar(DataType::Bool(b)))
        }
        DataHolder::Function(ref data_holder_args, ref commands) => {
            let func_args = holder_to_function_args(data_holder_args)?;
            Ok(DataContainer::Function(func_args, commands.clone()))
        }
        DataHolder::FunctionCall(ref target, ref args) => {
            Ok(run_function(controller, scope, target, args)?)
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
        DataHolder::ObjectAccess(ref ot, ref oa) => access_object(controller, scope, ot, oa)?,
        DataHolder::Function(_, _) => {
            Rc::new(RefCell::new(
                holder_deep_copy_conversion(controller, scope, target)?,
            ))
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
                    let rst = Rc::new(RefCell::new(
                        holder_deep_copy_conversion(controller, scope, &args[i])?,
                    ));
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
                            let rst = Rc::new(RefCell::new(
                                holder_deep_copy_conversion(controller, scope, &args[i])?,
                            ));
                            sub_scope.set_var(ref_name, rst);
                        }
                    }
                }
                FunctionArg::Const(ref name) => {
                    let rst = Rc::new(RefCell::new(
                        holder_deep_copy_conversion(controller, scope, &args[i])?,
                    ));
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
                            let rst = Rc::new(RefCell::new(
                                holder_deep_copy_conversion(controller, scope, &args[i])?,
                            ));
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

pub fn access_object<T: Logger>(
    controller: &mut Controller<T>,
    scope: &mut Scope,
    target: &DataHolder,
    accessor: &DataHolder,
) -> Result<RefHolder, Error> {
    let ref_accessor = match *accessor {
        DataHolder::Var(ref name) => {
            match scope.get_var(name) {
                Some(ref_holder) => ref_holder,
                None => return Err(Error::VarNotDeclared),
            }
        }
        DataHolder::Const(ref name) => {
            match scope.get_const(name) {
                Some(ref_holder) => ref_holder,
                None => return Err(Error::ConstNotDeclard),
            }
        }
        DataHolder::Anon(_) => Rc::new(RefCell::new(
            holder_deep_copy_conversion(controller, scope, accessor)?,
        )),
        DataHolder::ObjectAccess(ref t2, ref a2) => access_object(controller, scope, t2, a2)?,
        DataHolder::FunctionCall(ref ft, ref fa) => Rc::new(RefCell::new(
            run_function(controller, scope, ft, fa)?,
        )),
        _ => return Err(Error::InvalidObjectAccessAccessor),
    };

    if ref_accessor.borrow().is_reference() {
        let holder = unwrap_reference_to_type(&*ref_accessor.borrow()).unwrap();
        return access_object(controller, scope, target, &DataHolder::Anon(holder));
    }

    let mut ref_target = match *target {
        DataHolder::Var(ref name) => {
            match scope.get_var(name) {
                Some(ref_holder) => ref_holder,
                None => return Err(Error::VarNotDeclared),
            }
        }
        DataHolder::Const(ref name) => {
            match scope.get_const(name) {
                Some(ref_holder) => ref_holder,
                None => return Err(Error::ConstNotDeclard),
            }
        }
        DataHolder::Array(_) |
        DataHolder::Dict(_) => Rc::new(RefCell::new(
            holder_deep_copy_conversion(controller, scope, target)?,
        )),
        DataHolder::ObjectAccess(ref t2, ref a2) => access_object(controller, scope, t2, a2)?,
        DataHolder::FunctionCall(ref ft, ref fa) => Rc::new(RefCell::new(
            run_function(controller, scope, ft, fa)?,
        )),
        _ => return Err(Error::InvalidObjectAccessTarget),
    };

    while ref_target.borrow().is_reference() {
        let clone;
        {
            clone = ref_target.borrow().underlying_reference().unwrap();
        }
        ref_target = clone;
    }

    if let Some(data_type_ref) = ref_accessor.borrow().get_as_data_type_ref() {
        return match *ref_target.borrow() {
            DataContainer::Vector(ref vec_holders) => {
                match *data_type_ref {
                    DataType::Integer(int) => {
                        match vec_holders.get(int as usize) {
                            Some(array_item_holder) => Ok(array_item_holder.clone()),
                            None => Err(Error::InvalidArrayIndex),
                        }
                    }
                    _ => Err(Error::InvalidObjectAccessAccessor),
                }
            }
            DataContainer::Hash(ref map) => {
                match *data_type_ref {
                    DataType::String(ref string) => {
                        match map.get(string) {
                            Some(hash_item_holder) => Ok(hash_item_holder.clone()),
                            _ => Err(Error::InvalidHashKey),
                        }
                    }
                    _ => Err(Error::InvalidObjectAccessAccessor),
                }
            }
            _ => Err(Error::InvalidObjectAccessTarget),
        };
    }
    Err(Error::InvalidObjectAccessAccessor)
}
