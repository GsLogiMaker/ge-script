
//! TODO: Project level documentation.

#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]
#![warn(rustdoc::broken_intra_doc_links)]

use std::{fmt::{self, Display, Debug}, vec, collections::HashMap};


mod qu_error {

	use crate::QuToken;

	type QuErrorMessage = (String, String);


	/// Returns a [`QuErrorMessage`] for when an attempt is made to assign to
	/// an undefined variable.
	pub fn msg_assign_undefined_variable(a_var:String)
			-> QuErrorMessage {
		return (
			"UNDEFINED VARIABLE".to_string(),
			// TODO: Better message.
			format!("Can't assign to '{a_var}' because it was not previously defined.")
		);
	}


	/// Returns a [QuErrorMessage] for general errors. Don't let this be used
	/// because informative errors are very important.
	pub fn msg_general() -> QuErrorMessage {
		return (
			"GENERAL ERROR".to_string(),
			"Some error has occured. (Yo devs, Replace this message with a more descriptive one!)".to_string()
		);
	}


	/// Returns a [QuErrorMessage] for when a type is used that was not defined.
	pub fn msg_cmp_nonexistent_type(the_type:String) -> QuErrorMessage {
		return (
			"UNDEFINED TYPE".to_string(),
			format!("Can't use type '{the_type}' because it not previously defined.")
		);
	}


	/// Returns a [QuErrorMessage] for when a flow statement expects an
	/// assertion expression, but none is found.
	pub fn msg_flow_lacks_expression(flow_keyword:String) -> QuErrorMessage {
		return (
			"FLOW LACKS EXPRESSION".to_string(),
			format!("The '{flow_keyword}' statement lacks an expression.")
		);
	}


	/// Returns a [QuErrorMessage] for when a code block is expected but not
	/// found.
	pub fn msg_missing_code_block() -> QuErrorMessage {
		return (
			"MISSING CODE BLOCK".to_string(),
			"A code block was expected, but none was found.".to_string()
		);
	}


	/// Returns a [QuErrorMessage] for when a incohrent indentation is found.
	pub fn msg_prs_bad_indentation() -> QuErrorMessage {
		return (
			"BAD INDENTATION".to_string(),
			// TODO: Better message.
			"A line has an incorrect indentation level.".to_string()
		);
	}


	/// Returns a [QuErrorMessage] for when a statement is placed on the same
	/// line as a flow statement, like an if statement.
	pub fn msg_prs_bad_indentation2() -> QuErrorMessage {
		return (
			"BAD INDENTATION2".to_string(),
			// TODO: Better message.
			"A code block and flow statement are on the same line.".to_string()
		);
	}


	/// Returns a [QuErrorMessage] for when a new code block is started, but
	/// remains empty.
	pub fn msg_prs_empty_code_block() -> QuErrorMessage {
		return (
			"EMPTY CODE BLOCK".to_string(),
			// TODO: Better message.
			"A code block was started, but no code was written to it.".to_string()
		);
	}


	/// Returns a [QuErrorMessage] for when a variable is being read that
	/// is not defined.
	pub fn msg_read_undefined_variable(a_var:String)
			-> QuErrorMessage {
		return (
			"UNDEFINED VARIABLE".to_string(),
			// TODO: Better message.
			format!("Can't read from '{a_var}' because it was not previously defined.")
		);
	}


	/// Returns a [QuErrorMessage] for when a variable is being set to something
	/// invalid, basically anything that is not an expression.
	pub fn msg_invalid_variable_assignment(a_var:String, invalid_value:String)
			-> QuErrorMessage {
		return (
			"INVALID VARIABLE ASSIGNMENT".to_string(),
			// TODO: Better message.
			format!("Can't assign to variable '{a_var}' with '{invalid_value}' because it is not a valid value.")
		);
	}


	/// Returns a [QuErrorMessage] for when a parenthesy expression is not
	/// closed.
	pub fn msg_unclosed_parethesy_expression() -> QuErrorMessage {
		return (
			"UNCLOSED PARETHESY EXPRESSION".to_string(),
			"A parenthesy expression was opened, but it was nether closed.".to_string()
		);
	}


	/// Returns a [QuErrorMessage] for when a variable is defined twice.
	pub fn msg_var_redefinition(the_var:String) -> QuErrorMessage {
		return (
			"VARIABLE REDEFINITION".to_string(),
			format!("Can't define the variable '{the_var}' because it was already defined previously.")
		);
	}


	/// Returns a [QuErrorMessage] for when a variable assignment lacks an
	/// expression.
	pub fn msg_var_assign_lacks_expr(the_var:String) -> QuErrorMessage {
		return (
			"VARIABLE ASSIGNMENT LACKS EXPRESSION".to_string(),
			format!("A variable assignment for '{the_var}' lacks an expression.")
		);
	}


	/// Creates a Qu error message as a [String].
	/// 
	/// Examples:
	/// ```
	/// use qu_script::err_msg_make;
	/// use qu_script::QuError;
	/// use qu_script::QuToken;
	/// 
	/// let script = "print 1";
	/// let token = QuToken::new(0, 4, 0, 0, 0, 0);
	/// 
	/// let err_msg = err_msg_make(
	/// 	("My Error", "It crashed with My Error!"), token, script);
	/// println!(err_msg);
	/// ```
	pub fn make_message(err_msg:QuErrorMessage, tk:&QuToken, script:&String) -> String {
		// Line numbers
		let line_nm_pre_pre = (tk.row as usize).saturating_sub(1);
		let line_nm_pre = (tk.row as usize).saturating_sub(0);
		let line_nm = (tk.row as usize).saturating_add(1);
		let line_nm_post = (tk.row as usize).saturating_add(2);
		let line_nm_post_post = (tk.row as usize).saturating_add(3);

		// Line text
		let mut script_lines = script.split("\n");
		let line_pre_pre = if tk.row > 1 {
			script_lines.nth(line_nm_pre_pre-1).unwrap_or("").to_string()
		} else {
			"".to_string()
		};
		let line_pre = if tk.row > 0 {
			script_lines.next().unwrap_or("").to_string()
		} else {
			"".to_string()
		};
		let line = script_lines.next().unwrap_or("");
		let line_post =
				script_lines.next().unwrap_or("");
		let line_post_post =
				script_lines.next().unwrap_or("");

		// Build code view
		let code_view = format!(
"    {:0>4}:{}\n    {:0>4}:{}\n >> {:0>4}:{}\n    {:0>4}:{}\n    {:0>4}:{}\n\n",
			line_nm_pre_pre,
			line_pre_pre,
			line_nm_pre,
			line_pre,
			line_nm,
			line,
			line_nm_post,
			line_post,
			line_nm_post_post,
			line_post_post,
		);

		// Build error message
		let msg = format!(
			"ERROR on line {row}, col {col}; {m0}:\"{m1}\"\n{script}",
			row=tk.row+1, col=tk._col, m0=err_msg.0, m1=err_msg.1,
			script=code_view
		);
		return msg;
		
	}

}


#[derive(Debug, Clone)]
/// Defines an expression in a Qu program tree.
pub enum QuLeafExpr {
	/// A calculable expression. Contains an operator and two [`QuLeafExpr`]s.
	Equation(u8, Box<QuLeafExpr>, Box<QuLeafExpr>),
	/// A literal int value.
	Int(u64),
	/// A variable name.
	Var(QuToken),
} impl Display for QuLeafExpr {

	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			QuLeafExpr::Equation(op, lft, rht) => {
				let string = format!("{}", op);
				let opstr:&str = string.as_str();
				return write!(f, "Equate({lft} {opstr} {rht})");
			}
			QuLeafExpr::Int(val) => {
				return write!(f, "{val}:Int");
			}
			QuLeafExpr::Var(name) => {
				return write!(f, "{}:Var", name.text);
			}
			_ => {
				return write!(f, "<QuLeafExpr Unimplemented Format>");
			}
		}
	}

}



/// The interface for the Qu programming language.
pub struct Qu {
	vm:QuVm,
} impl Qu {
	
	/// Creates a new Qu instance.
	pub fn new() -> Self {
		Qu {
			vm:QuVm::new(),
		}
	}


	/// Compiles Qu code into bytecode.
	pub fn compile(&self, code:&str) -> Result<Vec<u8>, String> {
		let code_str = code.to_string();
		let tokens = &mut tokenize(&code_str, RULES);
		let mut parser = QuParser::new(tokens, &code_str);
		let instruction_vec = match parser.parse() {
			Ok(v) => v,
			// Propagate error
			Err(e) => {
				return Err(e);
			}
		};
		let leaf_block = QuLeaf::Block(instruction_vec);
		let mut c = QuCompiler::new(&code_str);
		let compiled = match c.compile(&leaf_block) {
			Ok(compiled) => compiled,
			Err(e) => {
				return Err(e);
			}
		};
		return Ok(compiled);
	}


	/// Compiles Qu code to Qu assembly.
	pub fn compile_to_asm(&mut self, code:&str) -> Result<String, String> {
		let byte_code = match self.compile(code) {
			Ok(byte_code) => byte_code,
			Err(e) => {
				return Err(e);
			}
		};

		return Ok(self.vm.code_to_asm(&byte_code, false));
	}


	/// Runs Qu code.
	pub fn run(&mut self, script:&str) -> Result<(), String> {
		let bytecode = match self.compile(script) {
			Ok(bytecode) => bytecode,
			Err(e) => {
				return Err(e);
			}
		};
		return self.run_bytes(bytecode.as_slice());
	}


	/// Runs Qu bytecode.
	pub fn run_bytes(&mut self, bytes:&[u8]) -> Result<(), String> {
		return Ok(self.vm.run_bytes(bytes));
	}

}


#[derive(Debug, Clone)]
/// A Qu instruction.
pub enum QuLeaf {
	/// A Block of leafs.
	Block(Vec<QuLeaf>),
	/// An if statement. Contains an assertion statement and a [`Vec`] of
	/// instructions.
	FlowStatement(u8, QuLeafExpr, Box<QuLeaf>),
	/// Prints a register to the console.
	Print(QuLeafExpr),
	/// A variable assignment. Contains a var name and a [`QuLeafExpr`].
	VarAssign(QuToken, QuLeafExpr),
	/// A variable declaration. Contains a var name, type(TODO), and
	/// [`QuLeafExpr`].
	VarDecl(QuToken, Option<QuToken>, Option<QuLeafExpr>),

} impl QuLeaf {

	/// Returns the [`QuLeaf`] as a [`String`] formatted into a tree.
	pub fn tree_fmt(&self, indent:u8) -> String {
		let mut indentstr = "\n".to_string();
		for _ in 0..indent {
			indentstr += "  ";
		}
		
		match self {
			QuLeaf::Block(
				body) => {
				let mut bodystr = "".to_string();
				for leaf in body {
					bodystr += &leaf.tree_fmt(indent + 1);
				}
				return format!("BLOCK:{}", bodystr);
			}
			QuLeaf::FlowStatement(
				op, 
				cond, 
				body) => {
					let bodystr = body.tree_fmt(indent + 1);
					return format!("{}FLOW {} {} {}", indentstr, op, cond, bodystr);
			}
			QuLeaf::Print(register) => {
				return format!("{}PRINT {}", indentstr, register);
			}
			QuLeaf::VarAssign(
				name, 
				val) => {
				return format!("{}ASSIGN {} = {}", indentstr, name.text, val);
			}
			QuLeaf::VarDecl(
				name, 
				_var_type, 
				_val) => {
				let val_str = match _val {
					Some(val) => format!("{}", val),
					None => "".to_string(),
				};
				// TODO: Add variable declaration type
				return format!("{}VAR {} {} = {}", indentstr, name.text, "", val_str);
			}
		}
	}

} impl Display for QuLeaf {

	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		return write!(f, "{}", self.tree_fmt(0));
	}

}

const FLOW_IF:u8 = 0;
const FLOW_WHILE:u8 = 1;
const FLOW_FOR:u8 = 2;
const FLOW_ELIF:u8 = 3;
const FLOW_ELSE:u8 = 4;

const KEYWORD_CLASS:&str = "cl";
const KEYWORD_ELSE:&str = "else";
const KEYWORD_ELIF:&str = "elif";
const KEYWORD_FUNCTION:&str = "fn";
const KEYWORD_IF:&str = "if";
const KEYWORD_PRINT:&str = "print";
const KEYWORD_TRAIT:&str = "tr";
const KEYWORD_TRAIT_IMPL:&str = "does";
const KEYWORD_VAR:&str = "vl";
const KEYWORD_WHILE:&str = "while";

const OP_ASSIGN_WORD:&str = "=";
const OP_BLOCK_START_WORD:&str = ":";

const TOKEN_TYPE_KEYWORD:u8 = 1;
const TOKEN_TYPE_SYMBOL:u8 = 2;
const TOKEN_TYPE_NUMBER:u8 = 3;
const TOKEN_TYPE_NAME:u8 = 4;

/// A [Vec] of tokenfule_* functions and their types.
///
/// This is used by [chars_fit_rule] to determin if a pattern of
/// characters should be turned into a [QuToken]. See [tokenrule_name] or 
/// [tokenrule_keyword] for examples of how a `tokenrule_*` function should
/// be structured.
type Rules<'a> = [(&'a dyn Fn(&[char])->bool, u8)];
/// A tuple of for specifying arguments for a [QuOperation].
type CommandArg = (i8,);

/// Base token rules for the Qu language.
pub const RULES:&Rules = &[
	(&tokenrule_keyword, TOKEN_TYPE_KEYWORD),
	(&tokenrule_symbols, TOKEN_TYPE_SYMBOL),
	(&tokenrule_number, TOKEN_TYPE_NUMBER),
	(&tokenrule_name, TOKEN_TYPE_NAME),
];

/// Token rules for Qu assembly.
pub const ASM_RULES:&Rules = &[
	(&tokenrule_symbols, TOKEN_TYPE_KEYWORD),
	(&tokenrule_flagref, 0),
	(&tokenrule_number, TOKEN_TYPE_NUMBER),
	(&tokenrule_name, TOKEN_TYPE_NAME),
];


/// A single Qu VM operation.
pub struct QuOperation {
	name:String,
	asm_keyword:String,
	args:Vec<CommandArg>,
	id:u8,
} impl QuOperation {
	pub fn new(name:&str, asm_keyword:&str, args:Vec<CommandArg>) -> Self {
		return Self{
			name:name.to_string(),
			asm_keyword:asm_keyword.to_string(),
			args:args,
			id:0,
		};
	}
}


/// A library of all of Qu's VM operations.
///
/// Instantiating this struct will create a library of all of Qu's VM operations.
pub struct QuOpLibrary {
	ops:Vec<QuOperation>,
	// Operations
	end:u8,
	load_const:u8,
	load_val_u8:u8,
	load_val_u16:u8,
	load_val_u32:u8,
	load_val_u64:u8,
	load_mem:u8,
	store_mem:u8,
	copy_reg:u8,
	add:u8,
	sub:u8,
	mul:u8,
	div:u8,
	modulate:u8,
	pow:u8,
	lesser:u8,
	greater:u8,
	equal:u8,
	not_equal:u8,
	not:u8,
	jump_to:u8,
	jump_by:u8,
	jump_to_if:u8,
	jump_by_if:u8,
	print:u8,
} impl QuOpLibrary {

	/// Create a new [QuOpLibrary].
	fn new() -> Self {
		let mut obj = Self{
			ops:vec![],
			end:0,
			load_const:0,
			load_val_u8:0,
			load_val_u16:0,
			load_val_u32:0,
			load_val_u64:0,
			load_mem:0,
			store_mem:0,
			copy_reg:0,
			add:0,
			sub:0,
			mul:0,
			div:0,
			modulate:0,
			pow:0,
			lesser:0,
			greater:0,
			equal:0,
			not_equal:0,
			not:0,
			jump_to:0,
			jump_by:0,
			jump_to_if:0,
			jump_by_if:0,
			print:0,
		};
		obj.generate();
		return obj;
	}


	/// Sets the operation id for the given [QuOperation].
	fn apply_op_id(&self, mut op:QuOperation) -> QuOperation {
		op.id = self.ops.len() as u8;
		return op;
	}


	/// Generate the library of Qu VM operations.
	fn generate(&mut self) {
		self.new_op("END", "End", &[]); self.end = self.ops.len() as u8 - 1;
		self.new_op("LDU8", "LoadU8", &[(1,), (1,)]); self.load_val_u8 = self.ops.len() as u8 - 1;
		self.new_op("LDU16", "LoadU16", &[(2,), (1,)]); self.load_val_u16 = self.ops.len() as u8 - 1;
		self.new_op("LDU32", "LoadU32", &[(4,), (1,)]); self.load_val_u32 = self.ops.len() as u8 - 1;
		self.new_op("LDU64", "LoadU64", &[(8,), (1,)]); self.load_val_u64 = self.ops.len() as u8 - 1;
		self.new_op("LDM", "LoadMem", &[(4,), (1,)]); self.load_mem = self.ops.len() as u8 - 1;
		self.new_op("STM", "StoreMem", &[(1,), (4,)]); self.store_mem = self.ops.len() as u8 - 1;
		self.new_op("CPY", "Copy", &[(1,), (1,)]); self.copy_reg = self.ops.len() as u8 - 1;
		self.new_op("ADD", "Add", &[(1,), (1,), (1,)]); self.add = self.ops.len() as u8 - 1;
		self.new_op("SUB", "Subtract", &[(1,), (1,), (1,)]); self.sub = self.ops.len() as u8 - 1;
		self.new_op("MUL", "Multiply", &[(1,), (1,), (1,)]); self.mul = self.ops.len() as u8 - 1;
		self.new_op("DIV", "Divide", &[(1,), (1,), (1,)]); self.div = self.ops.len() as u8 - 1;
		self.new_op("MOD", "Modulate", &[(1,), (1,), (1,)]); self.modulate = self.ops.len() as u8 - 1;
		self.new_op("POW", "Power", &[(1,), (1,), (1,)]); self.pow = self.ops.len() as u8 - 1;
		self.new_op("LES", "Lesser", &[(1,), (1,), (1,)]); self.lesser = self.ops.len() as u8 - 1;
		self.new_op("GRT", "Greater", &[(1,), (1,), (1,)]); self.greater = self.ops.len() as u8 - 1;
		self.new_op("EQ", "Equal", &[(1,), (1,), (1,)]); self.equal = self.ops.len() as u8 - 1;
		self.new_op("NEQ", "NotEqual", &[(1,), (1,), (1,)]); self.not_equal = self.ops.len() as u8 - 1;
		self.new_op("NOT", "Not", &[(1,), (1,)]); self.not = self.ops.len() as u8 - 1;
		self.new_op("JP", "JumpTo", &[(4,)]); self.jump_to = self.ops.len() as u8 - 1;
		self.new_op("JB", "JumpBy", &[(4,)]); self.jump_by = self.ops.len() as u8 - 1;
		self.new_op("JPI", "JumpIf", &[(4,), (1,)]); self.jump_to_if = self.ops.len() as u8 - 1;
		self.new_op("JBI", "JumpByIf", &[(4,), (1,)]); self.jump_by_if = self.ops.len() as u8 - 1;
		self.new_op("PRT", "Print", &[(1,)]); self.print = self.ops.len() as u8 - 1;
		
	}


	/// Create a new operation.
	fn new_op(&mut self, asm:&str, name:&str, args:&[CommandArg]) {
		self.ops.push(
			self.apply_op_id(
				QuOperation::new(name, asm, args.to_vec())
			)
		);
	}


	/// Converts a math or logic symbol to the id of the [QuOperation] that will
	/// perform it.
	fn op_id_from_symbol(&self, symbol:&str) -> u8 {
		return match symbol {
			"+" => self.add,
			"-" => self.sub,
			"*" => self.mul,
			"/" => self.div,
			"**" => self.pow,
			"%" => self.modulate,
			">" => self.greater,
			"<" => self.lesser,
			"==" => self.equal,
			"!=" => self.not_equal,
			_ => panic!("Unknown Qu VM operation symbol: {}", symbol),
		};
	}

}


/// Compiles [QuLeaf]s into Qu bytecode.
pub struct QuCompiler<'a> {
	/// Name, Type, Pointer
	variables:Vec<(String, usize, u8)>,
	script:&'a String,
	stack_layers:Vec<u8>,
	stack_idx:u8,
	types:Vec<QuType>,
	types_map:HashMap<String, usize>,
	ops:QuOpLibrary,
} impl<'a> QuCompiler<'a> {

	// TODO: Refactor so that QuCompiler does not need the script to be
	// instantiated. Instead, it should be passed to the compile function.
	/// Creates and returns a new [QuCompiler].
	pub fn new(script:&'a String) -> Self {
		let mut inst = Self{
			variables:vec![],
			script:script,
			stack_layers:vec![],
			stack_idx:0,
			types:vec![QuType::int(), QuType::uint(), QuType::bool()],
			types_map:HashMap::new(),
			ops:QuOpLibrary::new(),
		};

		let mut i:usize = 0;
		for tp in &inst.types {
			inst.types_map.insert(tp.name.clone(), i);
			i += 1;
		}

		return inst;
	}


	/// Compiles an expression into bytecode.
	fn cmp_expr(&mut self, leaf:&QuLeafExpr, output_reg:u8)
			-> Result<Vec<u8>, String> {
		return match leaf {
			QuLeafExpr::Equation(
				op,
				left,
				right
			) => self.cmp_expr_math(*op, &**left, &**right, output_reg),
			QuLeafExpr::Int(val)
				=> Ok(self.cmp_expr_int(*val, output_reg)),
			QuLeafExpr::Var(token)
				=> self.cmp_expr_val(token, output_reg),
		};
	}


	/// Compiles a math or logic expression into bytecode.
	fn cmp_expr_math(&mut self, op:u8, left:&QuLeafExpr, right:&QuLeafExpr,
			output_reg:u8) -> Result<Vec<u8>, String> {
		
		let right_reg = self.stack_reserve();

		let mut code:Vec<u8> = vec![];
		
		// Compile right expression
		let mut rgh_bytes
			= match self.cmp_expr(right, right_reg) {
				Ok(bytes) => bytes,
				Err(err) => return Err(err),
		};
		code.append(&mut rgh_bytes);
		// Compile left expression
		let mut lft_bytes
			= match self.cmp_expr(left, output_reg) {
				Ok(bytes) => bytes,
				Err(err) => return Err(err),
		};
		code.append(&mut lft_bytes);

		// Compile expression calculation
		code.append(&mut vec![op]);
		code.append(&mut vec![output_reg]);
		code.append(&mut vec![right_reg]);
		code.append(&mut vec![output_reg]);

		return Ok(code);
	}


	/// Compiles a constant integer expression into bytecode.
	fn cmp_expr_int(&mut self, val:u64, output_reg:u8) -> Vec<u8> {
		// TODO: Support other int sizes

		let mut code = vec![];
		// The vm operation
		code.push(self.ops.load_val_u8);
		// The number as bytes
		code.append(&mut (val as u8).to_be_bytes().to_vec());
		// The register to load into
		code.push(output_reg);

		return code;
	}


	/// Compiles a variable-expression into bytecode.
	fn cmp_expr_val(&mut self, token:&QuToken, output_reg:u8)
			-> Result<Vec<u8>, String> {
		let mut code = Vec::with_capacity(3);
		// The vm operation
		code.push(self.ops.copy_reg);
		// The register to copy the variable value from
		let var_reg
			= match self.get_var_register(token.text.as_str()) {
				// Set var_reg to that of the gotten variable
				Some(reg) => reg,
				// Return error, because variable could not be found
				None => {
					return Err(qu_error::make_message(
						qu_error::msg_read_undefined_variable(
							token.text(self.script)
						),
						token, self.script
					));
				},
		};
		code.append(&mut (var_reg as u8).to_be_bytes().to_vec());
		// The register to copy the variable value into
		code.push(output_reg);

		return Ok(code);
	}


	/// Compiles an *if* statement into bytecode.
	fn cmp_flow_if(&mut self, condition:&QuLeafExpr, body:&Box<QuLeaf>
	) -> Result<Vec<u8>, String> {
		self.stack_frame_push();
		let if_expr_reg = self.stack_reserve();

		// Compile expression and code block
		let mut expr_code
			= match self.cmp_expr(condition, if_expr_reg) {
				Ok(bytes) => bytes,
				Err(err) => return Err(err),
		};

		// Compile code block
		let mut block_code_result
			= self.cmp_scope(body);
		let mut block_code = match block_code_result {
			// Set block_code to compiled bytecode
			Ok(code) => code,
			// Propagate error
			Err(_) => return block_code_result,
		};

		// Compile if's jump
		let mut jump_code = Vec::with_capacity(7);
		let jump_assert_reg = self.stack_reserve();
		// Negate expression
		jump_code.push(self.ops.not);
		jump_code.push(if_expr_reg);
		jump_code.push(jump_assert_reg);
		// Jump instruction
		jump_code.push(self.ops.jump_by_if);
		jump_code.push(jump_assert_reg);
		jump_code.append(&mut (block_code.len() as i32).to_be_bytes().to_vec());

		// Compile code together
		let mut code = Vec::with_capacity(
				expr_code.len() + jump_code.len() + block_code.len());
		code.append(&mut expr_code);
		code.append(&mut jump_code);
		code.append(&mut block_code);

		self.stack_frame_pop();

		return Ok(code);
	}


	/// Compiles a *while* statement into bytecode.
	fn cmp_flow_while(&mut self, condition:&QuLeafExpr, body:&Box<QuLeaf>
	) -> Result<Vec<u8>, String> {
		self.stack_frame_push();
		let while_expr_reg = self.stack_reserve();

		// Compile expression and code block
		let mut expr_code
			= match self.cmp_expr(condition, while_expr_reg) {
				// Set expr_code to compiled bytecode
				Ok(code) => code,
				// Propagate error
				Err(err) => return Err(err),
		};
		// Compile code block
		let block_code_result
			= self.cmp_scope(body);
		let mut block_code
			= match self.cmp_scope(body) {
				// Set block code to returned code value
				Ok(code) => code,
				// Propogate error
				Err(_) => return block_code_result,
		};

		// Compile while's prejump
		let prejump_travel = block_code.len() as i32;
		let mut prejump_code = vec![];
		prejump_code.push(self.ops.jump_by);
		prejump_code.append(&mut prejump_travel.to_be_bytes().to_vec());

		// Compile while's loopback
		let loopback_travel = 
				-((block_code.len() + expr_code.len() + 6) as i32);
		let mut loopback_code = vec![];
		loopback_code.push(self.ops.jump_by_if);
		loopback_code.append(&mut loopback_travel.to_be_bytes().to_vec());
		loopback_code.push(while_expr_reg);

		// Compile code together
		let mut code = vec![];
		code.append(&mut prejump_code);
		code.append(&mut block_code);
		code.append(&mut expr_code);
		code.append(&mut loopback_code);

		self.stack_frame_pop();
		
		return Ok(code);
	}


	/// Compiles a [QuLeaf] into bytecode.
	fn cmp_leaf(&mut self, leaf:&QuLeaf) -> Result<Vec<u8>, String> {
		let result = match leaf {
			QuLeaf::Block(leafs) => {
				let mut code = vec![];
				for block_leaf in leafs {
					match self.cmp_leaf(block_leaf) {
						// Add compiled code to Vector
						Ok(mut block_code) => {
							code.append(&mut block_code);
						},
						// Propagate error
						Err(err) => {
							return Err(err);
						}
					}
				}
				return Ok(code);
			},
			QuLeaf::FlowStatement(
				flow_type,
				expr,
				statements,
			) => {
				match *flow_type{
					FLOW_IF => {
						return self.cmp_flow_if(expr, statements);
					},
					FLOW_WHILE => {
						return self.cmp_flow_while(expr, statements);
					}
					_ => unimplemented!(),
				}
			}
			QuLeaf::Print(leaf_expr) => {
				// TODO Handle errors for compiling Print
				let mut code = vec![];
				let print_reg = self.stack_reserve();
				let mut expression_code
					= match self.cmp_expr(leaf_expr, print_reg) {
						Ok(code) => code,
						// Propagate error
						Err(err) => return Err(err),
				};
				
				code.append(&mut expression_code);
				code.push(self.ops.print);
				code.push(print_reg);
				return Ok(code);
			},
			QuLeaf::VarDecl(
					name_tk,
					type_tk,
					value_leaf
			) => {
				return self.cmp_var_decl(
					name_tk, type_tk, value_leaf);
			}

			QuLeaf::VarAssign(
					name_rk,
					value_leaf
			) => {
				// TODO: Make cmp_var_assign return Result and remove this
				// Ok wrapper
				return  self.cmp_var_assign(
					name_rk, value_leaf);
			}

			_ => {
				unimplemented!()
			}
		};
		return result;
	}


	/// Compiles code variable assignment.
	fn cmp_var_assign(&mut self,
			variable_name_token:&QuToken, assign_to:&QuLeafExpr
			) -> Result<Vec<u8>, String> {
		// Get variable register
		let var_reg= match self.get_var_register(&variable_name_token.text) {
			Some(reg) => reg,
			None => {
				return Err(qu_error::make_message(
					qu_error::msg_assign_undefined_variable(
						variable_name_token.text(self.script),
					),
					variable_name_token, self.script,
				));
			},
		};
		// Compile assignment to expression
		return self.cmp_expr(assign_to, var_reg);
	}


	/// Compiles a variable declaration.
	fn cmp_var_decl(&mut self, variable_name_token:&QuToken,
			variable_type_token:&Option<QuToken>,
			assign_to:&Option<QuLeafExpr>) -> Result<Vec<u8>, String> {

		// Check if the variable is already defined
		if self.is_var_defined(&variable_name_token.text) {
			return Err(qu_error::make_message(
				qu_error::msg_var_redefinition(
					variable_name_token.text(self.script),
				),
				variable_name_token, self.script,
			));
		}

		// Get var type
		let mut var_type:usize = 0;
		if let Some(type_tk) = variable_type_token {
			let var_type_str = type_tk.text.as_str();
			if var_type_str != "" {
				match self.types_map.get(var_type_str) {
					// Set variable's type
					Some(type_id) => {
						var_type = *type_id;
					},
					// Return an error if the type is not defined
					None => {
						return Err(qu_error::make_message(
							qu_error::msg_cmp_nonexistent_type(
								type_tk.text(&self.script)),
							type_tk,
							self.script
						));
					}
				}
			}
		}

		// Create variable
		let var_reg = self.stack_reserve();
		self.variables.push(
			(variable_name_token.text.clone(), var_type, var_reg)
		);

		// Compile variable assignment
		return match assign_to {
			// Compile variable value
			Some(val_leaf)
				=> self.cmp_expr(&val_leaf, var_reg),

			// No default value, compile fallback to zero
			None => {
				// TODO: Support u64
				let mut code = Vec::with_capacity(3);
				code.push(self.ops.load_val_u8); 
				code.push(0);
				code.push(var_reg);
				Ok(code)
			},
		};
	}


	/// Compiles a scope.
	fn cmp_scope(&mut self, leaf:&QuLeaf) -> Result<Vec<u8>, String> {
		self.stack_frame_push();

		match self.cmp_leaf(leaf) {
			// Return compiled code
			Ok(code) => {
				self.stack_frame_pop();
				return Ok(code);
			},
			// Propagate error
			Err(err) => {
				self.stack_frame_pop();
				return Err(err);
			}
		}
	}


	/// Compiles from a [QuLeaf] instruction into bytecode (into a [Vec]<[u8]>.)
	pub fn compile(&mut self, leafs:&QuLeaf) -> Result<Vec<u8>, String> {
		match self.cmp_scope(leafs) {
			// Add exit code and return code
			Ok(mut code) => {
				code.push(self.ops.end);
				return Ok(code);
			},

			// Propgate error
			Err(err) => {
				return Err(err);
			}
		}
	}


	/// Gets the pointer to a variable by the variable's name.
	fn get_var_register(&self, var_name:&str) -> Option<u8> {
		for (name_, _type_, pointer_) in &self.variables {
			if name_ == var_name {
				return Some(*pointer_);
			}
		}
		return None;
	}



	/// Returns true if the given variable is already defined.
	fn is_var_defined(&self, var_name:&String) -> bool {
		// TODO: Maybe use a faster algorithm??
		for (name_, _, _) in &self.variables {
			if name_ == var_name {
				return true;
			}
		}
		return false;
	}


	/// Returns the current stack pointer and increments it.
	fn stack_reserve(&mut self) -> u8 {
		let x = self.stack_idx;
		self.stack_idx += 1;
		return x;
	}


	/// Closes the current stack frame returning the stake frame to the
	/// beginning of the frame.
	fn stack_frame_pop(&mut self) {
		self.stack_idx = self.stack_layers.pop().unwrap();
	}


	/// Starts a new stack frame.
	fn stack_frame_push(&mut self) {
		self.stack_layers.push(self.stack_idx);
	}

}


/// Parses Qu code into a [QuLeaf] tree.
pub struct QuParser<'a> {
	indent:u8,
	line:usize,
	tk_idx:usize,
	tk_stack:Vec<usize>,
	tokens:&'a Vec<QuToken>,
	script:&'a String,
	ops:QuOpLibrary,

} impl<'a> QuParser<'a> {

	// TODO: Refactor so that QuParser does not need the script or tokens to be
	// instantiated. Instead, it should be passed to the parse function.
	/// Creates and returns a new [QuParser].
	pub fn new(tokens:&'a mut Vec<QuToken>, script:&'a String) -> Self {
		tokens.push(
			QuToken::new(tokens.len() as u64, tokens.len() as u64, 
			0, 0, 0, u8::MAX)
		);
		return QuParser {
			indent:u8::MAX,
			line:0,
			tk_idx:0,
			tk_stack:vec![],
			tokens:tokens,
			script:script,
			ops:QuOpLibrary::new(),
		}
	}


	/// Attempts to parse a code block.
	fn ck_code_block(&mut self) -> Result<Option<Vec<QuLeaf>>, String> {
		let mut leafs = vec![];

		while self.tk_idx < self.tokens.len()-1 {
			// Variable declaration
			match self.ck_var_decl() {
				Ok(data_opt) => {
					if let Some(data) = data_opt {
						leafs.push(data);
						continue;
					}
				}
				Err(msg) => {
					return Err(format!("{}", msg));
				}
			}

			// Variable assignment
			match self.ck_var_assign() {
				Ok(data_opt) => {
					if let Some(data) = data_opt {
						leafs.push(data);
						continue;
					}
				}
				Err(msg) => {
					return Err(format!("{}", msg));
				}
			}

			// If Statement
			match self.ck_flow_if() {
				Ok(data_opt) => {
					if let Some(data) = data_opt {
						leafs.push(data);
						continue;
					}
				}
				Err(msg) => {
					return Err(format!("{}", msg));
				}
			}

			// while Statement
			match self.ck_flow_while() {
				Ok(data_opt) => {
					if let Some(data) = data_opt {
						leafs.push(data);
						continue;
					}
				}
				Err(msg) => {
					return Err(format!("{}", msg));
				}
			}

			// Print Statement
			match self.ch_print() {
				Ok(data_opt) => {
					if let Some(data) = data_opt {
						leafs.push(data);
						continue;
					}
				}
				Err(msg) => {
					return Err(format!("{}", msg));
				}
			}

			// End block
			break;
		}

		if leafs.len() == 0 {
			return Err(qu_error::make_message(
				qu_error::msg_missing_code_block(),
				&self.tokens[self.tk_idx-1], &self.script
			));
		}

		return Ok(Some(leafs));
	}


	/// Attempts to pasrse a code scope.
	fn ck_code_scope(&mut self) -> Result<Option<Vec<QuLeaf>>, String> {
		self.tk_push();

		// Check operator
		let start_tk = self.tk_next()
				.expect("Improper indentation TODO: Bette message");
		if start_tk != OP_BLOCK_START_WORD {
			self.tk_pop();
			return Ok(None);
		}

		self.indent += 1;
		let block_data = self.ck_code_block();
		self.indent -= 1;
		return block_data;
	}


	/// Attempts to parse a flow statement (Exp: if, while, for, etc).
	fn ck_flow(&mut self, token_type:u8) -> Result<Option<QuLeaf>, String> {
		match self.utl_statement_start() {
			Ok(opt) => (
				match opt {
					Some(_) => {/*pass*/},
					None => return Ok(None),
				}
			),
			Err(msg) => return Err(msg),
		}

		self.tk_push();

		// Check keyword
		let keyword = match token_type {
			FLOW_IF => "if",
			FLOW_WHILE => "while",
			_ => unimplemented!(),
		};
		let keyword_tk = self.tk_next()
				.expect("Improper indentation TODO: Bette message");
		if keyword_tk != keyword {
			self.tk_pop();
			return Ok(None);
		}

		// Check expression
		let expr = match self.ck_expr() {
			Ok(data_opt) => (
				match data_opt {
					Some(data) => data,
					None => {
						return Err(qu_error::make_message(
							qu_error::msg_flow_lacks_expression(
								keyword.to_string()
							),
							&self.tokens[self.tk_idx-1], &self.script
						));
					}
				}
			),
			Err(msg) => return Err(msg),
		};

		// Check for code block
		match self.ck_code_scope() {
			Ok(data_opt) => { match data_opt {

				// Code block found
				Some(scope_data) => {
					return Ok(Some(
						QuLeaf::FlowStatement(
							token_type,
							expr,
							Box::new(QuLeaf::Block(scope_data)))
					));
				},

				// No code block found, return error
				None => {
					return Err(qu_error::make_message(
						qu_error::msg_missing_code_block(),
						&self.tokens[self.tk_idx-1], &self.script
					));
				}
			}},

			// Propagate error
			Err(msg) => return Err(msg),
		};
	}


	/// Attempts to parse an if statement.
	fn ck_flow_if(&mut self) -> Result<Option<QuLeaf>, String> {
		return self.ck_flow(FLOW_IF);
	}


	/// Attempts to parse a while statement.
	fn ck_flow_while(&mut self) -> Result<Option<QuLeaf>, String> {
		return self.ck_flow(FLOW_WHILE);
	}


	/// Attempts to parse a print statement.
	fn ch_print(&mut self) -> Result<Option<QuLeaf>, String> {
		match self.utl_statement_start() {
			Ok(opt) => (
				match opt {
					Some(_) => {/*pass*/},
					None => return Ok(None),
				}
			),
			Err(msg) => return Err(msg),
		}

		// Match keyword
		let keyword_tk = self.tk_spy(0);
		if keyword_tk != KEYWORD_PRINT {
			return Ok(None);
		}
		self.tk_next().expect("Improper indentation TODO:Better msg");

		// Match register
		// Hack: Using unwrap and expect rather than properly handling errors.
		// 	Probobly isn't an issue since the print keyword is a hack and will
		// 	be removed anyway.
		let reg_tk = self.ck_value()
			.unwrap() 
			.expect("Print needs number TODO: Better msg");

		return Ok(Some(QuLeaf::Print(reg_tk)));
	}


	/// Attempts to parse a variable assignment.
	fn ck_var_assign(&mut self) -> Result<Option<QuLeaf>, String> {
		match self.utl_statement_start() {
			Ok(opt) => (
				match opt {
					Some(_) => {/*pass*/},
					None => return Ok(None),
				}
			),
			Err(msg) => return Err(msg),
		}

		self.tk_push();

		// Match variable name
		let name_data_result = self.ck_var_name();
		let name_tk = match name_data_result {
			Ok(name_data_opt) => {
				match name_data_opt {
					// Matched a variable name, set name_tk and continue
					Some(name_data) => name_data,
					// Tokens don't match a variable name, return None
					None => {return Ok(None);},
				}
			},
			// Propagate error
			Err(msg) => {
				self.tk_pop();
				return Err(msg);
			},
		};
		
		
		// Match assign operator
		let assign_op_tk = self.tk_next()
			.expect(format!("{} {}",
				"Improper indentation TODO: Bette message (I don't think",
				"this error message is reachable...)"
		).as_str());
		if assign_op_tk != OP_ASSIGN_WORD {
			self.tk_pop();
			return Ok(None);
		}

		// Match expression
		let expr_leaf = match self.ck_expr() {
			Ok(expr_data_opt) => {
				match expr_data_opt {
					Some(expr_data) => expr_data,
					None => {
						self.tk_pop();
						let false_expr = self.tk_spy(2).clone();
						return Err(qu_error::make_message(
							qu_error::msg_invalid_variable_assignment(
								name_tk.text,
								false_expr.text.clone()
							),
							&false_expr, &self.script
						));
					},
				}
			},
			Err(msg) => {
				self.tk_pop();
				return Err(msg);
			},
		};

		return Ok(Some(
			QuLeaf::VarAssign(name_tk, expr_leaf)
		));
	}


	/// Attempts to parse a variable declaration.
	fn ck_var_decl(&mut self) -> Result<Option<QuLeaf>, String> {
		match self.utl_statement_start() {
			Ok(opt) => (
				match opt {
					Some(_) => {/*pass*/},
					None => return Ok(None),
				}
			),
			Err(msg) => return Err(msg),
		}
		
		// Match keyword
		let keyword_tk = self.tk_spy(0);
		if keyword_tk != KEYWORD_VAR {
			return Ok(None);
		}
		self.tk_next().expect("Improper indentation TODO:Better msg");

		// Match variable name
		let name_data = match self.ck_var_name() {
			Ok(data) => data,
			// Propogate error
			Err(msg) => {
				return Err(msg);
			},
		};
		if let None = name_data {
			return Err(
				"Token after 'var' does not match a name. 'TODO:Better msg'"
				.to_string());
		}
		let name_tk = name_data.unwrap();

		// Match variable type
		let type_tk_opt = match self.ck_type_name() {
			Ok(data) => data,
			// Propogate error
			Err(msg) => {
				return Err(msg);
			},
		};

		// Match assign operator
		let keyword_tk = self.tk_spy(0);
		let mut assign_leaf_opt = None;
		if keyword_tk == OP_ASSIGN_WORD {
			self.tk_next()
					.expect("Improper indentation TODO:Better msg");
			assign_leaf_opt = match self.ck_expr() {
				Ok(data) => data,
				// Propogate error
				Err(msg) => {
					return Err(msg);
				},
			};
			if let None = assign_leaf_opt {
				return Err("Expected expression after '='. TODO:Better msg".to_string());
			}
		}
		
		return Ok(Some(QuLeaf::VarDecl(
			name_tk,
			type_tk_opt,
			assign_leaf_opt,
		)));
	}


	/// Attempts to parse a variable name.
	fn ck_var_name(&mut self) -> Result<Option<QuToken>, String> {
		let tk = self.tk_spy(0);

		if tk.tk_type != TOKEN_TYPE_NAME {
			return Ok(None);
		}
		let tk = tk.clone();

		match self.tk_next() {
			Ok(_) => {/*pass*/},
			Err(msg) => return Err(qu_error::make_message(
				qu_error::msg_prs_bad_indentation(),
				&self.tokens[self.tk_idx], &self.script
			)),
		};
		return Ok(Some(tk));
	}


	/// Attempts to parse an expression
	fn ck_expr(&mut self) -> Result<Option<QuLeafExpr>, String> {
		return self.ck_op_les();
	}


	/// Attempts to parse a lesser than expression.
	fn ck_op_les(&mut self) -> Result<Option<QuLeafExpr>, String>{
		return self.ck_operation("<", &Self::ck_op_grt);
	}


	/// Attempts to parse a greater than expression.
	fn ck_op_grt(&mut self) -> Result<Option<QuLeafExpr>, String> {
		return self.ck_operation(">", &Self::ck_op_eql);
	}


	/// Attempts to parse an equal to expression.
	fn ck_op_eql(&mut self) -> Result<Option<QuLeafExpr>, String> {
		return self.ck_operation("==", &Self::ck_op_not_eql);
	}


	/// Attempts to parse a not equal to expression.
	fn ck_op_not_eql(&mut self) -> Result<Option<QuLeafExpr>, String> {
		return self.ck_operation("!=", &Self::ck_op_sub);
	}


	/// Attempts to parse a subtraction expression.
	fn ck_op_sub(&mut self) -> Result<Option<QuLeafExpr>, String> {
		return self.ck_operation("-", &Self::ck_op_add);
	}


	/// Attempts to parse an addition expression.
	fn ck_op_add(&mut self) -> Result<Option<QuLeafExpr>, String>  {
		return self.ck_operation("+", &Self::ck_op_div);
	}


	/// Attempts to parse a division expression.
	fn ck_op_div(&mut self) -> Result<Option<QuLeafExpr>, String> {
		return self.ck_operation("/", &Self::ck_op_mul);
	}


	/// Attempts to parse a multiplication expression.
	fn ck_op_mul(&mut self) -> Result<Option<QuLeafExpr>, String> {
		return self.ck_operation("*", &Self::ck_op_paren_expr);
	}


	/// Attempts to parse a parenthesized expression.
	fn ck_op_paren_expr(&mut self) -> Result<Option<QuLeafExpr>, String> {
		self.tk_push();

		let tk = self.tk_next()
				.expect("Improper indentation TODO: Better message");
		if tk != "(" {
			self.tk_pop();
			self.tk_push();

			// Match for a value if no parenthesis expression is matched.
			let data = match self.ck_value() {
				Ok(data) => data,
				Err(msg) => return Err(msg),
			};
			if let None = data {
				self.tk_pop();
				return Ok(None);
			}
			return Ok(data);
		}

		let data = match self.ck_expr() {
			Ok(data) => data,
			Err(msg) => return Err(msg),
		};
		if let None = data {
			self.tk_pop();
			return Ok(None);
		}

		let closing_tk = self.tk_next()
				.expect("Improper indentation TODO: Better message");
		if closing_tk != ")" {
			return Err(qu_error::make_message(
				qu_error::msg_unclosed_parethesy_expression(),
				&self.tokens[self.tk_idx], &self.script
			));
		}

		return Ok(data);
	}


	/// A helper function for checking operations like addition or equality.
	fn ck_operation(
			&mut self, operator:&str,
			next:&dyn Fn(&mut Self)->Result<Option<QuLeafExpr>, String>,
			) -> Result<Option<QuLeafExpr>, String> {

		self.tk_push();

		// Check left side for value
		let data_l = match next(self) {
			Ok(data) => data,
			Err(msg) => return Err(msg),
		};
		if let None = data_l {
			self.tk_pop();
			return Ok(None);
		}
		let data_l = data_l.unwrap();

		// Check operator
		let tk_op = self.tk_spy(0);
		if tk_op != operator {
			return Ok(Some(data_l));
		}
		self.tk_next().expect("Improper indentation TODO: Bette message");

		// Check right side for expression
		let data_r
				= match self.ck_operation(operator, next) {
			Ok(data) => data,
			Err(msg) => return Err(msg),
		};
		if let None = data_r {
			self.tk_pop();
			return Ok(None);
		}
		let data_r = data_r.unwrap();

		return Ok(Some(
			QuLeafExpr::Equation(
				self.ops.op_id_from_symbol(operator),
				Box::new(data_l),
				Box::new(data_r)
			)
		));
	}


	/// Attempts to parse a type name.
	fn ck_type_name(&mut self) -> Result<Option<QuToken>, String> {
		return self.ck_var_name();
	}


	/// Attempts to parse a value.
	fn ck_value(&mut self) -> Result<Option<QuLeafExpr>, String> {
		self.tk_push();
		let tk = self.tk_next()
				.expect("Improper indentation TODO: Bette message");

		return match tk.text.parse::<u64>() {
			Ok(x) => Ok(Some(QuLeafExpr::Int(x))),
			Err(_) => {
				self.tk_pop();
				match self.ck_var_name() {
					Ok(var_name_opt) => {
						match var_name_opt {
							// Matched a value
							Some(data) =>
								Ok(Some(QuLeafExpr::Var(data))),
							// Didn't match a value, return None
							None => Ok(None),
						}
					}
					// Propagate error
					Err(err) => {return Err(err);}
				}
			},
		};
	}


	/// Parses a Qu script.
	pub fn parse(&mut self) -> Result<Vec<QuLeaf>, String> {
		self.tk_idx = 0;
		self.line = 0;
		self.indent = u8::MAX;

		let result = self.ck_code_block();
		return match result {
			Ok(data) => Ok(data.unwrap()),
			Err(msg) => Err(msg),
		};
	}


	/// A helper function for whenever starting to parse a statement.
	fn utl_statement_start(&mut self) -> Result<Option<()>, String> {
		let tk = self.tk_spy(0);
		let tk_row = tk.row as usize;
		let tk_indent = tk.indent as u8;

		if self.indent == u8::MAX {
			self.indent = tk_indent;
		}
		else if self.indent != tk_indent {

			if tk_indent < self.indent && tk_row != self.line {
				return Ok(None);
			}

			if self.line == tk_row {
				let tk = self.tokens[self.tk_idx].clone();
				return Err(qu_error::make_message(
					qu_error::msg_prs_bad_indentation2(),
					&tk, &self.script
				));
			}

			let tk = self.tokens[self.tk_idx-0].clone();
			self.line = tk_row;
			return Err(qu_error::make_message(
				qu_error::msg_prs_bad_indentation(),
				&tk, &self.script
			));
		}
		
		self.line = tk_row;
		return Ok(Some(()));
	}


	/// Returns the next token to parse.
	/// 
	/// Returns [`Err`] if the a parser error, Although the
	/// token can still be accessed from the [`Err`] if the indentation rules
	/// need to be ignored.
	/// 
	/// For a [`QuToken`] to follow the indentation rules it must be on
	/// the same line as its statement, unless the token is indented two times
	/// more than the statement.
	/// 
	/// Example:
	/// 
	/// ``` qu
	/// # Allowed
	/// vl counter = 1 + 2
	/// 
	/// # Allowed
	/// vl counter = 1
	/// 		+ 2
	/// 
	/// # Not allowed
	/// vl counter = 1
	/// 	+ 2
	/// 
	/// # Not allowed
	/// vl counter = 1
	/// + 2
	/// ```
	fn tk_next(&mut self) -> Result<&QuToken, &QuToken> {
		let tk = &self.tokens[self.tk_idx];

		// Check for proper indentation
		if tk.row != self.line as u64 {
			if tk.indent != self.indent+2 {
				return Err(tk);
			}
		}

		self.tk_idx += 1;
		return Ok(tk);
	}


	/// Returns to a previously saved token index.
	fn tk_pop(&mut self) {
		self.tk_idx = self.tk_stack.pop().unwrap();
	}


	/// Saves a the curent token index to return to if a parse attempt fails.
	/// see [func@`Parser::tk_pop`]
	fn tk_push(&mut self) {
		self.tk_stack.push(self.tk_idx);
	}


	/// Returns a &[`QuToken`] relative to the current token index without
	/// incrementing the current token index.
	/// 
	/// This function will not check if the token follows indentation rules.
	fn tk_spy(&mut self, at:usize) -> &QuToken {
		if self.tk_idx+at >= self.tokens.len() {
			return &self.tokens[self.tokens.len()-1];
		}
		return &self.tokens[self.tk_idx+at];
	}

}


/// A slice of a script file with information on the row, column, and indent of
/// the slice.
pub struct QuToken {
	/// Where in the script this token starts.
	pub begin:u64,
	/// Where in the script this token ends.
	pub end:u64,
	/// The row this token is on.
	pub row:u64,
	/// The column this token starts on.
	pub _col:u64,
	/// The indentation of this token.
	pub indent:u8,
	/// The text of this token.
	pub text:String,
	/// The type of this token.
	pub tk_type:u8,

} impl QuToken {

	/// Makes a new [`Token`].
	pub fn new(
			begin:u64, end:u64, row:u64, col:u64, indent:u8,
			varient:u8,) -> QuToken {
		return QuToken{
			begin,
			end,
			row,
			_col:col,
			indent,
			text:String::new(),
			tk_type:varient
		};
	}

	// TODO: Replace this function with the text parameter of the struct.
	/// Returns the text of this token.
	pub fn text(&self, source:&str) -> String {
		let mut text = String::new();
		if source.len() > 0 {
			text = source[self.begin as usize..=self.end as usize].to_string();
		}
		return text;
	}

} impl Clone for QuToken {
    fn clone(&self) -> Self {
        Self { 
			begin:self.begin.clone(),
			end:self.end.clone(),
			row:self.row.clone(),
			_col:self._col.clone(),
			indent:self.indent.clone(),
			text:self.text.clone(),
			tk_type:self.tk_type.clone()
		}
    }
} impl Display for QuToken {
	
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		return write!(
				f, "<'{}' row:{}  indent:{}>",
				self.text,
				self.row,
				self.indent,);
	}
} impl Debug for QuToken {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		return write!(f, "Hi: {}", 0);
		//f.debug_struct("QuToken")
		//	.field("begin", &self.begin)
		//	.field("end", &self.end)
		//	.field("row", &self.row)
		//	.field("_col", &self._col)
		//	.field("indent", &self.indent)
		//	.field("source", &self.source)
		//	.field("tk_type", &self.tk_type).finish()
	}
} impl PartialEq for QuToken {
	fn eq(&self, other:&Self) -> bool {
		if self.tk_type == u8::MAX {
			return false;
		}
		return self.text == other.text;
	}
} impl PartialEq<str> for QuToken {
	fn eq(&self, other:&str) -> bool {
		if self.tk_type == u8::MAX {
			return false;
		}
		return &self.text == other;
	}
} impl<'a> PartialEq<&str> for QuToken {
	fn eq(&self, other:&&str) -> bool {
		if self.tk_type == u8::MAX {
			return false;
		}
		return &self.text == *other;
	}
} impl PartialEq<String> for QuToken {
	fn eq(&self, other:&String) -> bool {
		if self.tk_type == u8::MAX {
			return false;
		}
		return &self.text == other;
	}
}


/// An object type (Example: int, bool, String, Object).
pub struct QuType {
	name:String,
	size:usize,
} impl QuType {

	/// Makes a new [`QuType`].
	fn new(name:String, size:usize) -> QuType {
		return QuType{
			name,
			size,
		};
	}


	/// Makes a boolean [`QuType`].
	fn bool() -> QuType {
		// TODO:: Make "bool" string a constant
		return QuType::new("bool".to_string(), 1);
	}


	/// Makes an integer [`QuType`].
	fn int() -> QuType {
		// TODO: Make "int" string a constant
		return QuType::new("int".to_string(), 1);
	}


	/// Makes an unsigned integer [`QuType`].
	fn uint() -> QuType {
		// TODO:: Make "uint" string a constant
		return QuType::new("uint".to_string(), 1);
	}

}


/// The virtual machine that runs Qu code.
pub struct QuVm {
	/// The registers of the VM.
	pub registers:[u64;256],
	/// Keeps track of which instruction to execute next.
	pc:usize,
	/// The source bytecode being run.
	pub source:Vec<u8>,
	/// The memory of the VM.
	pub mem:Vec<u64>,
	/// The library of available operations.
	pub op_lib:QuOpLibrary,

} impl QuVm {

	/// Makes a new [`Vm`].
	pub fn new() -> Self {
		let vm = QuVm { 
			pc:0,
			source:vec![],
			registers:[0;256],
			mem:vec![],
			op_lib:QuOpLibrary::new(),
		};

		return vm;
	}


	/// Converts byte code to human readable instructions.
	pub fn code_to_asm(&mut self, code:&Vec<u8>, include_line_columns:bool) -> String {
		let mut asm = String::new();
		
		let mut i = 0;
		while i < code.len() {
			let op_code = code[i];
			// HACK: Skip commands if they exceed the ops length.
			if op_code as usize >= self.op_lib.ops.len() {
				i += 1;
				// Add error text
				asm.push_str(format!("\n{:.>8}-{:.<8} {}",
					i, i, "INVALID OPERATION").as_str());
				continue;
			}
			let op
					= &self.op_lib.ops[op_code as usize];
			assert!(op.id == op_code);

			asm.push_str("\n");
			// Add line/index columns
			if include_line_columns {
				asm.push_str(
					format!("{:.>8}-{:.<8} ",
					i, i+op.args.len()).as_str()
				);
			}
			// Add code text
			asm.push_str(
				format!("{}",op.asm_keyword).as_str());

			// Add parameter text
			for (size,) in op.args.iter() {
				// Get value
				let val = match size {
					1 => {
						let bytes = [code[i+1]];
						i += 1;
						u8::from_be_bytes(bytes) as u64
					}
					2 => {
						let bytes = [code[i+1], code[i+2]];
						i += 2;
						u16::from_be_bytes(bytes) as u64
					}
					4 => {
						let bytes = [
							code[i+1], code[i+2], code[i+3], code[i+4]];
						i += 4;
						u32::from_be_bytes(bytes) as u64
					}
					8 => {
						let bytes = [
							code[i+1], code[i+2], code[i+3], code[i+4],
							code[i+5], code[i+6], code[i+7], code[i+8]];
						i += 8;
						u64::from_be_bytes(bytes) as u64
					}
					_ => panic!(),
				};
				asm.push_str(format!(" {}{}", "", val).as_str());
			}
			i += 1;
		}

		return asm;
	}


	fn exc_copy_reg(&mut self) {
		let from_reg = self.next_u8() as usize;
		let to_reg = self.next_u8() as usize;
		self.registers[to_reg] = self.registers[from_reg];
	}


	fn exc_jump_by(&mut self) {
		let val_by = self.next_u32() as usize;
		// Add
		if val_by as isize > 0 {
			self.pc += val_by;
		// Subtract
		} else {
			self.pc = self.pc.wrapping_add(val_by);
		}
		
	}


	fn exc_jump_by_if(&mut self) {
		let val_by = self.next_u32() as i32;
		let rg_if = self.next_u8() as usize;
		if self.registers[rg_if] as i64 > 0 {
			// Add
			if val_by > 0 {
				self.pc += val_by as usize;
			// Subtract
			} else {
				self.pc -= val_by.abs() as usize;
			}
		}
	}


	fn exc_jump_to(&mut self) {
		unimplemented!()
	}


	fn exc_jump_to_if(&mut self) {
		let val_to = self.next_u32() as usize;
		let rg_if = self.next_u8() as usize;
		if self.registers[rg_if] > 0 {
			self.pc = val_to as usize;
		}
	}


	/// Executes a load instruction.
	fn exc_load_const_u8(&mut self) {
		let src_from = self.next_u32() as usize;
		let rg_to = self.next_u8() as usize;
		self.registers[rg_to] = self.source[src_from] as u64;
	}


	fn exc_load_val_u8(&mut self) {
		let val = self.next_u8() as u64;
		let rg_to = self.next_u8() as usize;
		self.registers[rg_to] = val;
	}


	fn exc_load_val_u16(&mut self) {
		let val = self.next_u16() as u64;
		let rg_to = self.next_u8() as usize;
		self.registers[rg_to] = val;
	}


	fn exc_load_val_u32(&mut self) {
		let val = self.next_u32() as u64;
		let rg_to = self.next_u8() as usize;
		self.registers[rg_to] = val;
	}


	fn exc_load_val_u64(&mut self) {
		let val = self.next_u64() as u64;
		let rg_to = self.next_u8() as usize;
		self.registers[rg_to] = val;
	}


	fn exc_load_mem(&mut self) {
		let mem_from = self.next_u32() as usize;
		let rg_to = self.next_u8() as usize;
		self.registers[rg_to] = self.mem[mem_from] as u64;
	}


	fn exc_print(&mut self) {
		let read_from_reg = self.next_u8() as usize;
		let val = self.registers[read_from_reg];
		println!("Qu Print: {}", val);
	}


	fn exc_store_mem(&mut self) {
		let rg_from = self.next_u8() as usize;
		let mem_to = self.next_u32() as usize;
		if mem_to >= self.mem.len() {
			self.mem.resize(mem_to+1, 0);
		}
		self.mem[mem_to] = self.registers[rg_from];
	}


	fn exc_math_add(&mut self) {
		let rg_left = self.next_u8() as usize;
		let rg_right = self.next_u8() as usize;
		let rg_result = self.next_u8() as usize;
		self.registers[rg_result] = 
				self.registers[rg_left] as u64
				+ self.registers[rg_right] as u64;
	}


	fn exc_math_sub(&mut self) {
		let rg_left = self.next_u8() as usize;
		let rg_right = self.next_u8() as usize;
		let rg_result = self.next_u8() as usize;
		self.registers[rg_result] = 
				self.registers[rg_left] as u64
				- self.registers[rg_right] as u64;
	}


	fn exc_math_mul(&mut self) {
		let rg_left = self.next_u8() as usize;
		let rg_right = self.next_u8() as usize;
		let rg_result = self.next_u8() as usize;
		self.registers[rg_result] = 
				self.registers[rg_left] as u64
				* self.registers[rg_right] as u64;
	}


	fn exc_math_div(&mut self) {
		let rg_left = self.next_u8() as usize;
		let rg_right = self.next_u8() as usize;
		let rg_result = self.next_u8() as usize;
		self.registers[rg_result] = 
				self.registers[rg_left] as u64
				/ self.registers[rg_right] as u64;
	}


	fn exc_math_mod(&mut self) {
		let rg_left = self.next_u8() as usize;
		let rg_right = self.next_u8() as usize;
		let rg_result = self.next_u8() as usize;
		self.registers[rg_result] = 
				self.registers[rg_left] as u64
				% self.registers[rg_right] as u64;
	}


	fn exc_math_pow(&mut self) {
		let rg_left = self.next_u8() as usize;
		let rg_right = self.next_u8() as usize;
		let rg_result = self.next_u8() as usize;
		self.registers[rg_result] = 
				self.registers[rg_left] as u64
				^ self.registers[rg_right] as u64;
	}


	fn exc_logi_equal(&mut self) {
		let rg_left = self.next_u8() as usize;
		let rg_right = self.next_u8() as usize;
		let rg_result = self.next_u8() as usize;
		self.registers[rg_result] = 
				(self.registers[rg_left] == self.registers[rg_right]) as u64;
	}


	fn exc_logi_greater(&mut self) {
		let rg_left = self.next_u8() as usize;
		let rg_right = self.next_u8() as usize;
		let rg_result = self.next_u8() as usize;
		self.registers[rg_result] = 
				(self.registers[rg_left] > self.registers[rg_right]) as u64;
	}


	fn exc_logi_lesser(&mut self) {
		let rg_left = self.next_u8() as usize;
		let rg_right = self.next_u8() as usize;
		let rg_result = self.next_u8() as usize;
		self.registers[rg_result] = 
				(self.registers[rg_left] < self.registers[rg_right]) as u64;
	}


	fn exc_logi_not_equal(&mut self) {
		let rg_left = self.next_u8() as usize;
		let rg_right = self.next_u8() as usize;
		let rg_result = self.next_u8() as usize;
		self.registers[rg_result] = 
				(self.registers[rg_left] != self.registers[rg_right]) as u64;
	}


	fn exc_logi_not(&mut self) {
		let rg_left = self.next_u8() as usize;
		let rg_result = self.next_u8() as usize;
		let x = self.registers[rg_left];
		self.registers[rg_result] = 
				(x*0 == x) as u64;
	}


	/// Gets the next byte in the source code as a u8 int.
	fn next_u8(&mut self) -> u8 {
		let val = self.source[self.pc];
		self.pc += 1;
		return val;
	}


	/// Gets the next 2 byte in the source code as a u16 int.
	fn next_u16(&mut self) -> u16 {
		let bytes = [self.source[self.pc], self.source[self.pc+1]];
		self.pc += 2;
		return u16::from_be_bytes(bytes);
	}


	/// Gets the next 4 byte in the source code as a u32 int.
	fn next_u32(&mut self) -> u32 {
		let bytes = [
			self.source[self.pc], self.source[self.pc+1],
			self.source[self.pc+2], self.source[self.pc+3],
		];
		self.pc += 4;
		return u32::from_be_bytes(bytes);
	}


	// Gets the next 8 byte in the source code as a u64 int.
	fn next_u64(&mut self) -> u64 {
		let bytes = [
			self.source[self.pc], self.source[self.pc+1],
			self.source[self.pc+2], self.source[self.pc+3],
			self.source[self.pc+4], self.source[self.pc+5],
			self.source[self.pc+6], self.source[self.pc+7],
		];
		self.pc += 8;
		return u64::from_be_bytes(bytes);
	}

	
	pub fn run_bytes(&mut self, code:&[u8]) {
		let ops = QuOpLibrary::new();
		self.source = code.to_vec();
		loop {
			
			match self.next_u8() {
				x if x == ops.end as u8 => {break;},

				x if x == ops.load_const => self.exc_load_const_u8(),
				x if x == ops.load_val_u8 => self.exc_load_val_u8(),
				x if x == ops.load_val_u16 => self.exc_load_val_u16(),
				x if x == ops.load_val_u32 => self.exc_load_val_u32(),
				x if x == ops.load_val_u64 => self.exc_load_val_u64(),
				x if x == ops.load_mem => self.exc_load_mem(),
				x if x == ops.store_mem => self.exc_store_mem(),
				x if x == ops.copy_reg => self.exc_copy_reg(),
				x if x == ops.add => self.exc_math_add(),
				x if x == ops.sub => self.exc_math_sub(),
				x if x == ops.mul => self.exc_math_mul(),
				x if x == ops.div => self.exc_math_div(),
				x if x == ops.modulate => self.exc_math_mod(),
				x if x == ops.pow => self.exc_math_pow(),
				x if x == ops.lesser => self.exc_logi_lesser(),
				x if x == ops.greater => self.exc_logi_greater(),
				x if x == ops.equal => self.exc_logi_equal(),
				x if x == ops.not_equal => self.exc_logi_not_equal(),
				x if x == ops.not => self.exc_logi_not(),
				x if x == ops.jump_to => self.exc_jump_to(),
				x if x == ops.jump_by => self.exc_jump_by(),
				x if x == ops.jump_to_if => self.exc_jump_to_if(),
				x if x == ops.jump_by_if => self.exc_jump_by_if(),
				x if x == ops.print => self.exc_print(),

				x => { println!("{x}"); todo!(); }
			}

		}
	}

}


/// Returns *true* if the passed characters matches to an assembly line-flag.
pub fn tokenrule_flagref(added_so_far:&[char]) -> bool {
	if added_so_far[0] != '$' {
		return false;
	}

	let mut i = 1; // Set to 1 because first checked previously
	while i < added_so_far.len() {
		if added_so_far[i] == ' ' {
			return false
		}
		if !added_so_far[i].is_alphanumeric() && added_so_far[i] != '_' {
			return false;
		}
		i += 1;
	}

	return true;
}


/// Returns *true* if the passed characters match to a name.
///
/// A name could be a type, class, function name, or variable name.
/// 
/// Examples
/// ```
/// use qu_script::tokenrule_name;
///
///	let chars1:&[char] = &['_', '_', 'i', 'n', 'i', 't', '_', '_',];
///	assert!(qu_script::tokenrule_name(chars1));
///
///	let chars2:&[char] = &['a', 'b', '1', ];
///	assert!(qu_script::tokenrule_name(chars2));
///
///	let chars3:&[char] = &['a', '+', '=', ];
///	assert!(!qu_script::tokenrule_name(chars3));
/// ```
pub fn tokenrule_name(added_so_far:&[char]) -> bool {
	for char in  added_so_far {
		if *char == ' ' {
			return false
		}
		if !char.is_alphanumeric() && *char != '_' {
			return false;
		}
	}

	return true;
}


/// Returns *true* if the passed characters match to a number (like *int* or
/// *float*).
/// 
/// Examples
/// ```
/// use qu_script::tokenrule_number;
///
///	let chars1:&[char] = &['5', '.', '6',];
///	assert!(qu_script::tokenrule_number(chars1));
///
///	let chars2:&[char] = &['1','0',];
///	assert!(qu_script::tokenrule_number(chars2));
///
///	let chars3:&[char] = &['a', ];
///	assert!(!qu_script::tokenrule_number(chars3));
/// ```
pub fn tokenrule_number(added_so_far:&[char]) -> bool {
	
	for char in  added_so_far {
		if char == &' ' {
			return false
		}
		if !(char.is_numeric() || char == &'.') {
			return false;
		}
	}

	return true;
}


/// Returns *true* if the passed characters match to a keyword. 
/// 
/// Some examples of keywords are *var*, *if*, and *fn*.
/// 
/// Example
/// ```
/// use qu_script::tokenrule_keyword;
///
///	let chars1:&[char] = &['v', 'a', 'r',];
///	assert!(qu_script::tokenrule_keyword(chars1));
///
///	let chars2:&[char] = &['i','f',];
///	assert!(qu_script::tokenrule_keyword(chars2));
///
///	let chars3:&[char] = &['d', 'u', 'd', 'e',];
///	assert!(!qu_script::tokenrule_keyword(chars3));
/// ```
pub fn tokenrule_keyword(added_so_far:&[char]) -> bool {
	for word in [
		KEYWORD_VAR,
		KEYWORD_FUNCTION,
		KEYWORD_CLASS,
		KEYWORD_IF,
		KEYWORD_ELSE,
		KEYWORD_ELIF,
	] {
		let mut mismatched = false;
		for (char1, char2) in added_so_far.iter().zip(word.chars()) {
			mismatched = char1 != &char2;
			if mismatched {
				break;
			}
		}
		if !mismatched {
			return true && added_so_far.len() == word.len();
		}
	}

	return false;
}


/// Returns *true* if the passed characters match to a symbol.
/// 
/// Some examples of operators are *+*, *-*, and *+=*.
/// 
/// Examples
/// ```
/// use qu_script::tokenrule_symbols;
///
///	let chars1:&[char] = &['*',];
///	assert!(qu_script::tokenrule_symbols(chars1));
///
///	let chars2:&[char] = &['=','=',];
///	assert!(qu_script::tokenrule_symbols(chars2));
///
///	let chars3:&[char] = &['+', '1'];
///	assert!(!qu_script::tokenrule_symbols(chars3));
/// ```
pub fn tokenrule_symbols(added_so_far:&[char]) -> bool {
	return match added_so_far {
		['*',] => true,
		['/',] => true,
		['\\',] => true, // This is just '\'
		['=',] => true,
		['!',] => true,
		['?',] => true,
		['>',] => true,
		['<',] => true,
		['+',] => true,
		['-',] => true,
		['%',] => true,
		['*', '*',] => true,
		['/', '/',] => true,
		['=', '=',] => true,
		['!', '=',] => true,
		['>', '=',] => true,
		['<', '=',] => true,
		['+', '=',] => true,
		['-', '=',] => true,
		['*', '=',] => true,
		['/', '=',] => true,
		['%', '=',] => true,
		['&',] => true,
		['@',] => true,
		['|',] => true,
		['^',] => true,
		[':',] => true,
		[';',] => true,
		[',',] => true,
		['(',] => true,
		[')',] => true,
		['[',] => true,
		[']',] => true,
		['{',] => true,
		['}',] => true,
		['`',] => true,
		['"',] => true,
		['\'',] => true,
		_ => false,
	};
}


/// Tokenizes a [String] according to the passed [Rules].
/// 
/// Examples
/// ```
/// use qu_script::Token;
/// use qu_script::tokenize;
/// use qu_script::tokenrule_name;
/// use qu_script::tokenrule_symbols;
/// use qu_script::TOKEN_TYPE_NAME;
/// use qu_script::TOKEN_TYPE_SYMBOL;
/// 
/// let script:&str = " hello=world ;! ";
/// 
/// let tokens:Vec<Token> = tokenize(
/// 	&script,
/// 	&[
/// 		(&tokenrule_name, TOKEN_TYPE_NAME),
/// 		(&tokenrule_symbols, TOKEN_TYPE_SYMBOL),
/// 	]
/// );
/// 
///	assert!(tokens.len() == 5);
///	assert!(tokens[0].text(&script) == "hello");
///	assert!(tokens[1].text(&script) == "=");
///	assert!(tokens[2].text(&script) == "world");
///	assert!(tokens[3].text(&script) == ";");
///	assert!(tokens[4].text(&script) == "!");
/// ```
pub fn tokenize<'a>(script:&'a String, rules:&Rules<'a>) -> Vec<QuToken> {
	let mut tokens = vec!();

	/* WARNING: This does not account for grapheme clusters. Currently hoping
	This won't be a problem. */
	let mut row:u64 = 0;
	let mut col:u64 = 0;
	let mut indent:u8 = 0;
	let mut in_begining:bool = true;
	let mut added_so_far:Vec<char> = Vec::with_capacity(20);
	let mut curr_token = 0;
	for (idx, char) in script.char_indices() {
		col += 1;

		if char != '\t' && char != ' ' {
			in_begining = false;
		}

		// Check tab
		if char == '\t' {
			if in_begining {
				indent += 1;
				added_so_far.clear();
			}
			
		// Check newline
		} else if char == '\n' {
			col = 0;
			row += 1;
			indent = 0;
			in_begining = true;		
		}

		// Any other characters
		added_so_far.push(char);
		
		// Update token end if it fits rule, 
		// otherwise clear the added so far
		loop {
			let (does_fit, char_type) = chars_fit_rule(
				&added_so_far,
				rules,
			);
			if does_fit{
				if curr_token <= tokens.len() && added_so_far.len() == 1 {
					tokens.push(QuToken::new(
						idx as u64,
						idx as u64,
						row,
						col,
						indent,
						char_type,
					));
				}
				tokens[curr_token].end = idx as u64;
				tokens[curr_token].tk_type = char_type;

				if idx+1 == script.len() {
					tokens[curr_token].text
							= tokens[curr_token].text(script);
				}
				break;
				
			} else if added_so_far.len() == 1 {
				added_so_far.clear();
				break;
			}else {
				if curr_token+1 == tokens.len() {
					tokens[curr_token].text
							= tokens[curr_token].text(script);
					curr_token += 1;
				}
				added_so_far.clear();
				added_so_far.push(char);
			}
		}
	}

	return tokens;
}


/// Checks a [`Vec`] of [`char`]s against [`RULES`].
/// 
/// Returns *true* if the [`Vec`] of [`char`]s fits at least one of the rules
/// specified in [`RULES`].
pub fn chars_fit_rule<'a>(chars:&Vec<char>, rules:&Rules<'a>) -> (bool, u8) {
	let mut fits_rule = false;
	let mut tk_type = u8::MAX;
	for rule in rules {
		fits_rule = fits_rule || rule.0(&chars);
		if fits_rule{
			tk_type = rule.1;
			break;
		}
	}

	return (fits_rule, tk_type);
}