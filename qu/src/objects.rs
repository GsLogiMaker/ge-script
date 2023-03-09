
//! Defines all types and objects used by Qu.

use std::{any::Any, ops::{Add, AddAssign, Sub, SubAssign}};

use crate::{QuVm, parser::QuOperator, import::QuStructId};
use crate::Qu;
use crate::QuMsg;
use crate::vm::QuExtFn;
use crate::vm::QuMemId;
use crate::vm::QuVoidExtFn;
use crate::vm::QuStackId;
use crate::vm::StackValue;
use std::fmt::Debug;


macro_rules! qufn {
	($name:ident($($param:ident),*) $return:ident $block:expr) => {
		{
			ExternalFunction::new(
				stringify!($name),
				&$block,
				[$(<$param as QuRegisterStruct>::name()),*].into(),
				<$return as QuRegisterStruct>::name(),
			)
		}
	};
}


pub type QuExtFnData = (String, QuExtFn, Vec<usize>, usize);
pub type QuVoidFnForm = (String, QuVoidExtFn, Vec<usize>);
type QuMethodRegistration = (String, &'static dyn Fn(&mut QuVm));


#[derive(Clone)]
pub struct ExternalFunction {
	pub name: String,
	pub pointer: QuExtFn,
	pub parameters: Vec<&'static str>,
	pub return_type: &'static str,
} impl ExternalFunction {
	pub fn new(
		name: &str,
		pointer: QuExtFn,
		parameters: Vec<&'static str>,
		return_type: &'static str,
	) -> Self{
		Self {
			name: name.into(),
			pointer,
			parameters: parameters,
			return_type,
		}
	}


	pub fn call(
		&self, vm:&mut QuVm, parameters: &Vec<QuStackId>, output_id: QuStackId,
	) -> Result<(), QuMsg> {
		(self.pointer)(vm, parameters, output_id)
	}
} impl Debug for ExternalFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExternalFunction")
			.field("name", &self.name)
			.field("parameters", &self.parameters)
			.field("return_type", &self.return_type)
			.finish()
    }
} impl Default for ExternalFunction {
    fn default() -> Self {
        Self {
			pointer: &|_, _, _| {Ok(())},
			..Default::default()
		}
    }
}


/// Defines all the types supported by Qu.
#[derive(Debug, Default, Clone)]
pub enum QuType {
	#[default] Void,
	Int,
	Bool,
	String,
	Tuple(Vec<QuType>),
	Array,
	Dictionary,
	Object(usize),
} impl From<u8> for QuType {

	fn from(f:u8) -> Self { 
		match f {
			0 => QuType::Void,
			1 => QuType::Int,
			2 => QuType::Bool,
			3 => QuType::String,
			4 => panic!(),//QuType::Tuple(Vec::default()),
			5 => QuType::Array,
			6 => QuType::Dictionary,
			7 => QuType::Object(0),
			_ => panic!(),
		}
	}

}


/// Defines all the types supported by Qu.
#[derive(Debug, Default, Clone)]
pub enum QuValue {
	#[default] Void,
	Int(isize),
	Bool(bool),
	Object(usize),
}


/// Defines a block of code.
/// 
/// Often used for Qu functions.
#[derive(Debug, Default, Clone)]
pub struct QuCodeObject {
	/// Where the [`QuCodeObject`] starts. The [`QuCodeObject`] ends at the
	/// `END` VM instruction.
	pub start_index:usize,
} impl QuCodeObject {

	/// Constructs a new [`QuCodeObject`].
	/// 
	/// # Examples
	/// 
	/// ```
	/// use qu::QuCodeObject;
	/// 
	/// let codeobj = QuCodeObject::new(0);
	/// ```
	pub fn new(start_index:usize) -> Self {
		return Self{start_index};
	}

}


/// Defines a Qu function.
#[derive(Debug, Default, Clone)]
pub struct QuFnObject {
	/// The input parameters of the function.
	pub parameters:Vec<QuType>,
	/// The function body.
	pub body:QuCodeObject,
	/// The variable type that this function returns.
	pub return_type:QuType,

} impl QuFnObject {

	pub fn new(parameters:Vec<QuType>, body:QuCodeObject, return_type:QuType
	) -> Self {
		Self {
			parameters,
			body,
			return_type,
		}
	}

}


impl QuRegisterStruct for bool {
	
	fn register_fns() -> Vec<ExternalFunction> {
		return vec![
			qufn!(and(Self, Self) Self |vm, args, return_id| {
				vm.write(return_id, *vm.read::<Self>(args[0])? && *vm.read::<Self>(args[1])?);
				Ok(())
			}),
			qufn!(or(Self, Self) Self |vm, args, return_id| {
				vm.write(return_id, *vm.read::<Self>(args[0])? || *vm.read::<Self>(args[1])?);
				Ok(())
			}),
			qufn!(eq(Self, Self) Self |vm, args, return_id| {
				let output = vm.read::<i32>(args[0])? == vm.read::<i32>(args[1])?;
				vm.hold_is_zero = output;
				if output {
					vm.write(return_id, 1i32);
				} else {
					vm.write(return_id, 0i32);
				}
				Ok(())
			}),
			qufn!(neq(Self, Self) Self |vm, args, return_id| {
				let output = vm.read::<i32>(args[0])? != vm.read::<i32>(args[1])?;
				vm.hold_is_zero = output;
				if output {
					vm.write(return_id, 1i32);
				} else {
					vm.write(return_id, 0i32);
				}
				Ok(())
			}),
			qufn!(copy(Self, Self) Self |vm, args, return_id| {
				vm.write(return_id, *vm.read::<bool>(args[0])?);
				Ok(())
			}),
		];
	}


	fn name() -> &'static str {"bool"}

}


impl QuRegisterStruct for i32 {
	
	fn register_fns() -> Vec<ExternalFunction> {
		return vec![
			qufn!(add(i32, i32) i32 |vm, args, return_id| {
				vm.write(return_id, vm.read::<i32>(args[0])? + vm.read::<i32>(args[1])?);
				Ok(())
			}),
			qufn!(sub(i32, i32) i32 |vm, args, return_id| {
				vm.write(return_id, vm.read::<i32>(args[0])? - vm.read::<i32>(args[1])?);
				Ok(())
			}),
			qufn!(mul(i32, i32) i32 |vm, args, return_id| {
				vm.write(return_id, vm.read::<i32>(args[0])? * vm.read::<i32>(args[1])?);
				Ok(())
			}),
			qufn!(div(i32, i32) i32 |vm, args, return_id| {
				vm.write(return_id, vm.read::<i32>(args[0])? / vm.read::<i32>(args[1])?);
				Ok(())
			}),
			qufn!(lesser(i32, i32) bool |vm, args, return_id| {
				let output = vm.read::<i32>(args[0])? < vm.read::<i32>(args[1])?;
				vm.hold_is_zero = output;
				vm.write(return_id, output);
				Ok(())
			}),
			qufn!(lessereq(i32, i32) bool |vm, args, return_id| {
				let output = vm.read::<i32>(args[0])? <= vm.read::<i32>(args[1])?;
				vm.hold_is_zero = output;
				vm.write(return_id, output);
				Ok(())
			}),
			qufn!(greater(i32, i32) bool |vm, args, return_id| {
				let output = vm.read::<i32>(args[0])? > vm.read::<i32>(args[1])?;
				vm.hold_is_zero = output;
				vm.write(return_id, output);
				Ok(())
			}),
			qufn!(greatereq(i32, i32) bool |vm, args, return_id| {
				let output = vm.read::<i32>(args[0])? >= vm.read::<i32>(args[1])?;
				vm.hold_is_zero = output;
				vm.write(return_id, output);
				Ok(())
			}),
			qufn!(eq(i32, i32) bool |vm, args, return_id| {
				let output = vm.read::<i32>(args[0])? == vm.read::<i32>(args[1])?;
				vm.hold_is_zero = output;
				vm.write(return_id, output);
				Ok(())
			}),
			qufn!(neq(i32, i32) bool |vm, args, return_id| {
				let output = vm.read::<i32>(args[0])? != vm.read::<i32>(args[1])?;
				vm.hold_is_zero = output;
				vm.write(return_id, output);
				Ok(())
			}),
			qufn!(copy(i32, i32) i32 |vm, args, return_id| {
				vm.write(return_id, *vm.read::<i32>(args[0])?);
				Ok(())
			}),
		];
	}


	fn name() -> &'static str {"int"}

}



#[derive(Debug, Default, Clone, Copy)]
/// Represents a void type in Qu.
pub struct QuVoid();
impl QuRegisterStruct for QuVoid {

	fn register_fns() -> Vec<ExternalFunction> {
		vec![
			qufn!(copy(Self, Self) Self |_vm, _parameters, _return_id| {
				Ok(())
			}),
		]
	}


	fn name() -> &'static str {"void"}

}


/// A trait for registering structs into the Qu programming language.
pub trait QuRegisterStruct {

	/// Returns functions that are callable by [`QuVm`].
	fn register_fns() -> Vec<ExternalFunction> where Self: Sized {
		Vec::default()
	}


	/// Returns the name that identifies the struct being registered.
	fn name() -> &'static str;

}


#[cfg(test)]
mod test_objects {

}