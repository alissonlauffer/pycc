use crate::ast::{Binary, BinaryOperator, Identifier, Literal, LiteralValue, Node};
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{BasicValueEnum, PointerValue};
use std::collections::HashMap;

pub struct CodeGenerator<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    variables: HashMap<String, (PointerValue<'ctx>, BasicValueEnum<'ctx>)>,
    string_counter: usize,
}

impl<'ctx> CodeGenerator<'ctx> {
    pub fn new(context: &'ctx Context, module_name: &str) -> Self {
        let module = context.create_module(module_name);
        let builder = context.create_builder();

        CodeGenerator {
            context,
            module,
            builder,
            variables: HashMap::new(),
            string_counter: 0,
        }
    }

    pub fn compile(&mut self, program: &Node) -> Result<(), String> {
        match program {
            Node::Program(program) => {
                // Create main function
                let int_type = self.context.i32_type();
                let fn_type = int_type.fn_type(&[], false);
                let function = self.module.add_function("main", fn_type, None);
                let basic_block = self.context.append_basic_block(function, "entry");
                self.builder.position_at_end(basic_block);

                // Generate code for each statement
                for statement in &program.statements {
                    self.compile_statement(statement)?;
                }

                // Return 0 by default if no return statement was executed
                self.builder
                    .build_return(Some(&int_type.const_int(0, false)))
                    .unwrap();

                Ok(())
            }
            _ => Err("Expected a program node".to_string()),
        }
    }

    fn compile_statement(&mut self, statement: &Node) -> Result<(), String> {
        match statement {
            Node::Assignment(assignment) => {
                let value = self.compile_expression(&assignment.value)?;

                // For division results, ensure we use float type even if operands are integers
                let is_division = if let Node::Binary(binary) = &*assignment.value {
                    matches!(binary.operator, BinaryOperator::Divide)
                } else {
                    false
                };

                // Allocate space for the variable on the stack
                let ptr = if is_division {
                    // For division, always allocate as float
                    let float_type = self.context.f64_type();
                    self.builder
                        .build_alloca(float_type, &assignment.name)
                        .unwrap()
                } else {
                    self.builder
                        .build_alloca(value.get_type(), &assignment.name)
                        .unwrap()
                };

                // Convert value to the allocation type if needed
                let stored_value = if is_division {
                    // For division, ensure the result is stored as float
                    match value {
                        BasicValueEnum::FloatValue(_) => value,
                        BasicValueEnum::IntValue(int_val) => {
                            let float_type = self.context.f64_type();
                            self.builder
                                .build_signed_int_to_float(int_val, float_type, "int_to_float")
                                .unwrap()
                                .into()
                        }
                        _ => value,
                    }
                } else {
                    value
                };

                self.builder.build_store(ptr, stored_value).unwrap();
                self.variables
                    .insert(assignment.name.clone(), (ptr, stored_value));
                Ok(())
            }
            Node::ExpressionStatement(expr_stmt) => {
                self.compile_expression(&expr_stmt.expression)?;
                Ok(())
            }
            Node::Function(function) => {
                self.compile_function(function)?;
                Ok(())
            }
            Node::Return(return_stmt) => {
                // Handle return statement
                if let Some(value) = &return_stmt.value {
                    let return_value = self.compile_expression(value)?;
                    self.builder.build_return(Some(&return_value)).unwrap();
                    Ok(())
                } else {
                    // Return void
                    self.builder.build_return(None).unwrap();
                    Ok(())
                }
            }
            _ => Ok(()), // Ignore unsupported statements for now
        }
    }

    fn compile_function(&mut self, function: &crate::ast::Function) -> Result<(), String> {
        // Save current position
        let current_position = self.builder.get_insert_block();

        // For now, we'll use i64 as the return type for all functions
        // The f-string issue needs a different approach
        let return_type = self.context.i64_type();
        let param_types: Vec<_> = function
            .parameters
            .iter()
            .map(|_| return_type.into())
            .collect();
        let fn_type = return_type.fn_type(&param_types, false);

        // Create function
        let function_value = self.module.add_function(&function.name, fn_type, None);

        // Create basic block
        let basic_block = self.context.append_basic_block(function_value, "entry");
        self.builder.position_at_end(basic_block);

        // Create allocations for parameters
        for (i, param_name) in function.parameters.iter().enumerate() {
            let param = function_value.get_nth_param(i as u32).unwrap();
            let ptr = self.builder.build_alloca(return_type, param_name).unwrap();
            self.builder.build_store(ptr, param).unwrap();
            self.variables.insert(param_name.clone(), (ptr, param));
        }

        // Compile function body
        self.compile_statement(&function.body)?;

        // Add return instruction if not already present
        if !basic_block
            .get_last_instruction()
            .is_some_and(|inst| inst.is_terminator())
        {
            self.builder
                .build_return(Some(&return_type.const_int(0, false)))
                .unwrap();
        }

        // Restore previous position
        if let Some(block) = current_position {
            self.builder.position_at_end(block);
        }

        Ok(())
    }

    fn compile_expression(&mut self, expression: &Node) -> Result<BasicValueEnum<'ctx>, String> {
        match expression {
            Node::Literal(literal) => {
                match &literal.value {
                    LiteralValue::Integer(value) => {
                        let int_type = self.context.i64_type();
                        Ok(int_type.const_int(*value as u64, false).into())
                    }
                    LiteralValue::Float(value) => {
                        let float_type = self.context.f64_type();
                        Ok(float_type.const_float(*value).into())
                    }
                    LiteralValue::String(value) => {
                        // Create a global string constant with a unique name
                        let name = format!("str_{}", self.string_counter);
                        self.string_counter += 1;
                        let str_ptr = self.builder.build_global_string_ptr(value, &name).unwrap();
                        // Return the pointer to the string
                        Ok(str_ptr.as_pointer_value().into())
                    }
                    LiteralValue::FString(fstring) => {
                        // Handle f-string by parsing and evaluating expressions
                        let evaluated_string = self.evaluate_fstring_codegen(fstring)?;
                        Ok(evaluated_string)
                    }
                    LiteralValue::Boolean(value) => {
                        // For boolean literals, we'll use i64 but with a special marker
                        // We'll use -2 for True and -3 for False to distinguish from regular integers
                        let int_type = self.context.i64_type();
                        let bool_val = if *value { -2i64 } else { -3i64 };
                        Ok(int_type.const_int(bool_val as u64, true).into())
                    }
                    LiteralValue::None => {
                        // Represent None as 0
                        let int_type = self.context.i64_type();
                        Ok(int_type.const_int(0, false).into())
                    }
                }
            }
            Node::Identifier(identifier) => {
                if let Some((ptr, stored_value)) = self.variables.get(&identifier.name) {
                    let value = self
                        .builder
                        .build_load(stored_value.get_type(), *ptr, "loadtmp")
                        .unwrap();
                    Ok(value)
                } else {
                    Err(format!("Undefined variable: {}", identifier.name))
                }
            }
            Node::Unary(unary) => {
                let operand = self.compile_expression(&unary.operand)?;
                match unary.operator {
                    crate::ast::UnaryOperator::Plus => Ok(operand),
                    crate::ast::UnaryOperator::Minus => match operand {
                        BasicValueEnum::IntValue(int_val) => {
                            let zero = int_val.get_type().const_int(0, false);
                            let result =
                                self.builder.build_int_sub(zero, int_val, "negtmp").unwrap();
                            Ok(result.into())
                        }
                        BasicValueEnum::FloatValue(float_val) => {
                            let zero = float_val.get_type().const_float(0.0);
                            let result = self
                                .builder
                                .build_float_sub(zero, float_val, "fnegtmp")
                                .unwrap();
                            Ok(result.into())
                        }
                        _ => Err("Unsupported unary minus operation".to_string()),
                    },
                    crate::ast::UnaryOperator::Not => {
                        Err("Unsupported unary not operation".to_string())
                    }
                }
            }
            Node::Binary(binary) => {
                let left = self.compile_expression(&binary.left)?;
                let right = self.compile_expression(&binary.right)?;

                match binary.operator {
                    BinaryOperator::Add => match (left, right) {
                        (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => {
                            let result = self.builder.build_int_add(l, r, "addtmp").unwrap();
                            Ok(result.into())
                        }
                        (BasicValueEnum::FloatValue(l), BasicValueEnum::FloatValue(r)) => {
                            let result = self.builder.build_float_add(l, r, "faddtmp").unwrap();
                            Ok(result.into())
                        }
                        (BasicValueEnum::PointerValue(l), BasicValueEnum::PointerValue(r)) => {
                            // String concatenation
                            self.concatenate_strings(l, r)
                        }
                        _ => Err("Unsupported operation".to_string()),
                    },
                    BinaryOperator::Subtract => match (left, right) {
                        (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => {
                            let result = self.builder.build_int_sub(l, r, "subtmp").unwrap();
                            Ok(result.into())
                        }
                        (BasicValueEnum::FloatValue(l), BasicValueEnum::FloatValue(r)) => {
                            let result = self.builder.build_float_sub(l, r, "fsubtmp").unwrap();
                            Ok(result.into())
                        }
                        _ => Err("Unsupported operation".to_string()),
                    },
                    BinaryOperator::Multiply => match (left, right) {
                        (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => {
                            let result = self.builder.build_int_mul(l, r, "multmp").unwrap();
                            Ok(result.into())
                        }
                        (BasicValueEnum::FloatValue(l), BasicValueEnum::FloatValue(r)) => {
                            let result = self.builder.build_float_mul(l, r, "fmultmp").unwrap();
                            Ok(result.into())
                        }
                        _ => Err("Unsupported operation".to_string()),
                    },
                    BinaryOperator::Divide => match (left, right) {
                        (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => {
                            if r.get_zero_extended_constant() == Some(0) {
                                Err("Division by zero".to_string())
                            } else {
                                // Convert integers to float for true division
                                let float_type = self.context.f64_type();
                                let l_float = self
                                    .builder
                                    .build_signed_int_to_float(l, float_type, "l_float")
                                    .unwrap();
                                let r_float = self
                                    .builder
                                    .build_signed_int_to_float(r, float_type, "r_float")
                                    .unwrap();
                                let result = self
                                    .builder
                                    .build_float_div(l_float, r_float, "fdivtmp")
                                    .unwrap();
                                Ok(result.into())
                            }
                        }
                        (BasicValueEnum::FloatValue(l), BasicValueEnum::FloatValue(r)) => {
                            if r.is_null() {
                                Err("Division by zero".to_string())
                            } else {
                                let result = self.builder.build_float_div(l, r, "fdivtmp").unwrap();
                                Ok(result.into())
                            }
                        }
                        _ => Err("Unsupported operation".to_string()),
                    },
                    BinaryOperator::FloorDivide => match (left, right) {
                        (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => {
                            if r.get_zero_extended_constant() == Some(0) {
                                Err("Division by zero".to_string())
                            } else {
                                Ok(BasicValueEnum::IntValue(l))
                            }
                        }
                        (BasicValueEnum::FloatValue(l), BasicValueEnum::FloatValue(r)) => {
                            if r.is_null() {
                                Err("Division by zero".to_string())
                            } else {
                                Ok(BasicValueEnum::FloatValue(l))
                            }
                        }
                        _ => Err("Unsupported operation".to_string()),
                    },
                    BinaryOperator::Modulo => match (left, right) {
                        (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => {
                            if r.get_zero_extended_constant() == Some(0) {
                                Err("Division by zero".to_string())
                            } else {
                                let result =
                                    self.builder.build_int_signed_rem(l, r, "modtmp").unwrap();
                                Ok(result.into())
                            }
                        }
                        (BasicValueEnum::FloatValue(l), BasicValueEnum::FloatValue(r)) => {
                            if r.is_null() {
                                Err("Division by zero".to_string())
                            } else {
                                let result = self.builder.build_float_rem(l, r, "fmodtmp").unwrap();
                                Ok(result.into())
                            }
                        }
                        _ => Err("Unsupported operation".to_string()),
                    },
                    BinaryOperator::Power => match (left, right) {
                        (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(_r)) => {
                            Ok(BasicValueEnum::IntValue(l))
                        }
                        (BasicValueEnum::FloatValue(l), BasicValueEnum::FloatValue(_r)) => {
                            Ok(BasicValueEnum::FloatValue(l))
                        }
                        _ => Err("Unsupported operation".to_string()),
                    },
                    _ => Err("Unsupported binary operator".to_string()),
                }
            }
            Node::Call(call) => {
                // Look up the function in the module
                if let Some(function_value) = self.module.get_function(&call.callee) {
                    // Compile arguments
                    let mut args = Vec::new();
                    for arg in &call.arguments {
                        let value = self.compile_expression(arg)?;
                        args.push(value.into());
                    }

                    // Create function call
                    let call_result = self
                        .builder
                        .build_call(function_value, &args, "calltmp")
                        .unwrap();
                    // For now, we'll assume the function returns a value
                    // In a real implementation, we'd need to handle void returns
                    Ok(call_result.try_as_basic_value().unwrap_basic())
                } else if call.callee == "print" {
                    // Special handling for print function
                    // Get or declare printf function
                    let printf_fn = if let Some(func) = self.module.get_function("printf") {
                        func
                    } else {
                        let i32_type = self.context.i32_type();
                        let str_type = self.context.ptr_type(inkwell::AddressSpace::default());
                        let printf_fn_type = i32_type.fn_type(&[str_type.into()], true);
                        self.module.add_function("printf", printf_fn_type, None)
                    };

                    if let Some(arg) = call.arguments.first() {
                        let value = self.compile_expression(arg)?;

                        // Handle different types of values
                        match value {
                            BasicValueEnum::IntValue(int_val) => {
                                // Check if this is a boolean value (we use -2 for True, -3 for False)
                                let name = format!("fmt_{}", self.string_counter);
                                self.string_counter += 1;

                                let true_val = int_val.get_type().const_int((-2i64) as u64, true);
                                let false_val = int_val.get_type().const_int((-3i64) as u64, true);

                                let is_true = self
                                    .builder
                                    .build_int_compare(
                                        inkwell::IntPredicate::EQ,
                                        int_val,
                                        true_val,
                                        "is_true",
                                    )
                                    .unwrap();
                                let is_false = self
                                    .builder
                                    .build_int_compare(
                                        inkwell::IntPredicate::EQ,
                                        int_val,
                                        false_val,
                                        "is_false",
                                    )
                                    .unwrap();
                                let is_boolean = self
                                    .builder
                                    .build_or(is_true, is_false, "is_boolean")
                                    .unwrap();

                                // Create basic blocks for conditional branching
                                let function = self
                                    .builder
                                    .get_insert_block()
                                    .unwrap()
                                    .get_parent()
                                    .unwrap();
                                let boolean_block =
                                    self.context.append_basic_block(function, "boolean_check");
                                let numeric_block =
                                    self.context.append_basic_block(function, "print_numeric");
                                let true_print_block =
                                    self.context.append_basic_block(function, "print_true");
                                let false_print_block =
                                    self.context.append_basic_block(function, "print_false");
                                let merge_block =
                                    self.context.append_basic_block(function, "merge");

                                // Branch based on whether it's a boolean
                                self.builder
                                    .build_conditional_branch(
                                        is_boolean,
                                        boolean_block,
                                        numeric_block,
                                    )
                                    .unwrap();

                                // Block for boolean values - check if true or false
                                self.builder.position_at_end(boolean_block);
                                let is_true_val = self
                                    .builder
                                    .build_int_compare(
                                        inkwell::IntPredicate::EQ,
                                        int_val,
                                        true_val,
                                        "is_true_val",
                                    )
                                    .unwrap();
                                self.builder
                                    .build_conditional_branch(
                                        is_true_val,
                                        true_print_block,
                                        false_print_block,
                                    )
                                    .unwrap();

                                // Block for printing "True"
                                self.builder.position_at_end(true_print_block);
                                let true_format = self
                                    .builder
                                    .build_global_string_ptr("True\n", &format!("{}_true", name))
                                    .unwrap();
                                let _ = self
                                    .builder
                                    .build_call(
                                        printf_fn,
                                        &[true_format.as_pointer_value().into()],
                                        "printf_true",
                                    )
                                    .unwrap();
                                self.builder
                                    .build_unconditional_branch(merge_block)
                                    .unwrap();

                                // Block for printing "False"
                                self.builder.position_at_end(false_print_block);
                                let false_format = self
                                    .builder
                                    .build_global_string_ptr("False\n", &format!("{}_false", name))
                                    .unwrap();
                                let _ = self
                                    .builder
                                    .build_call(
                                        printf_fn,
                                        &[false_format.as_pointer_value().into()],
                                        "printf_false",
                                    )
                                    .unwrap();
                                self.builder
                                    .build_unconditional_branch(merge_block)
                                    .unwrap();

                                // Block for printing numeric values
                                self.builder.position_at_end(numeric_block);
                                // Print integers as integers, not as floats
                                let format_str = self
                                    .builder
                                    .build_global_string_ptr("%ld\n", &name)
                                    .unwrap();
                                let _ = self
                                    .builder
                                    .build_call(
                                        printf_fn,
                                        &[format_str.as_pointer_value().into(), int_val.into()],
                                        "printf",
                                    )
                                    .unwrap();
                                self.builder
                                    .build_unconditional_branch(merge_block)
                                    .unwrap();

                                // Merge block
                                self.builder.position_at_end(merge_block);
                            }
                            BasicValueEnum::FloatValue(float_val) => {
                                // Create format string for float with proper formatting
                                let name = format!("fmt_{}", self.string_counter);
                                self.string_counter += 1;

                                // Check if it's zero and print as 0.0 instead of 0
                                let zero_val = float_val.get_type().const_float(0.0);
                                let is_zero = self
                                    .builder
                                    .build_float_compare(
                                        inkwell::FloatPredicate::OEQ,
                                        float_val,
                                        zero_val,
                                        "is_zero_float",
                                    )
                                    .unwrap();

                                let function = self
                                    .builder
                                    .get_insert_block()
                                    .unwrap()
                                    .get_parent()
                                    .unwrap();
                                let zero_block = self
                                    .context
                                    .append_basic_block(function, "print_zero_float");
                                let regular_block = self
                                    .context
                                    .append_basic_block(function, "print_regular_float");
                                let merge_block =
                                    self.context.append_basic_block(function, "merge_float");

                                self.builder
                                    .build_conditional_branch(is_zero, zero_block, regular_block)
                                    .unwrap();

                                // Block for printing 0.0
                                self.builder.position_at_end(zero_block);
                                let zero_format = self
                                    .builder
                                    .build_global_string_ptr("0.0\n", &format!("{}_zero", name))
                                    .unwrap();
                                let _ = self
                                    .builder
                                    .build_call(
                                        printf_fn,
                                        &[zero_format.as_pointer_value().into()],
                                        "printf_zero",
                                    )
                                    .unwrap();
                                self.builder
                                    .build_unconditional_branch(merge_block)
                                    .unwrap();

                                // Block for printing regular float
                                self.builder.position_at_end(regular_block);
                                let format_str =
                                    self.builder.build_global_string_ptr("%g\n", &name).unwrap();
                                let _ = self
                                    .builder
                                    .build_call(
                                        printf_fn,
                                        &[format_str.as_pointer_value().into(), float_val.into()],
                                        "printf",
                                    )
                                    .unwrap();
                                self.builder
                                    .build_unconditional_branch(merge_block)
                                    .unwrap();

                                // Merge block
                                self.builder.position_at_end(merge_block);
                            }
                            BasicValueEnum::PointerValue(ptr_val) => {
                                // For string literals in print, we need to handle them specially
                                // Let's check if this is a string literal and handle it correctly
                                let name = format!("fmt_{}", self.string_counter);
                                self.string_counter += 1;
                                let format_str =
                                    self.builder.build_global_string_ptr("%s\n", &name).unwrap();
                                let _ = self
                                    .builder
                                    .build_call(
                                        printf_fn,
                                        &[format_str.as_pointer_value().into(), ptr_val.into()],
                                        "printf",
                                    )
                                    .unwrap();
                            }
                            _ => {
                                // For other types, just print a placeholder
                                let name = format!("fmt_{}", self.string_counter);
                                self.string_counter += 1;
                                let format_str = self
                                    .builder
                                    .build_global_string_ptr("Value\n", &name)
                                    .unwrap();
                                let _ = self
                                    .builder
                                    .build_call(
                                        printf_fn,
                                        &[format_str.as_pointer_value().into()],
                                        "printf",
                                    )
                                    .unwrap();
                            }
                        }
                    } else {
                        // Print just a newline
                        let name = format!("fmt_{}", self.string_counter);
                        self.string_counter += 1;
                        let format_str = self.builder.build_global_string_ptr("\n", &name).unwrap();
                        let _ = self
                            .builder
                            .build_call(
                                printf_fn,
                                &[format_str.as_pointer_value().into()],
                                "printf",
                            )
                            .unwrap();
                    }
                    // Print function returns None (represented as 0)
                    let int_type = self.context.i64_type();
                    Ok(int_type.const_int(0, false).into())
                } else {
                    Err(format!("Undefined function: {}", call.callee))
                }
            }
            _ => Err("Unsupported expression type".to_string()),
        }
    }

    pub fn print_ir(&self) {
        self.module.print_to_stderr();
    }

    pub fn write_ir_to_file(&self, filename: &str) -> Result<(), String> {
        use std::fs::File;
        use std::io::Write;

        let ir_string = self.module.print_to_string().to_string();
        let mut file =
            File::create(filename).map_err(|e| format!("Failed to create file {filename}: {e}"))?;
        file.write_all(ir_string.as_bytes())
            .map_err(|e| format!("Failed to write to file {filename}: {e}"))?;
        Ok(())
    }

    pub fn write_object_to_file(&self, filename: &str) -> Result<(), String> {
        use inkwell::targets::FileType;
        use inkwell::targets::{InitializationConfig, Target, TargetMachine};
        use std::fs::File;
        use std::io::Write;

        // Initialize LLVM targets
        let config = InitializationConfig::default();
        Target::initialize_all(&config);

        // Get the target triple for the current machine
        let target_triple = TargetMachine::get_default_triple();
        let target = Target::from_triple(&target_triple)
            .map_err(|e| format!("Failed to get target: {}", e.to_string()))?;

        // Create target machine
        let target_machine = target
            .create_target_machine(
                &target_triple,
                "generic",
                "",
                inkwell::OptimizationLevel::Default,
                inkwell::targets::RelocMode::Default,
                inkwell::targets::CodeModel::Default,
            )
            .ok_or("Failed to create target machine")?;

        // Generate object code
        let object_data = target_machine
            .write_to_memory_buffer(&self.module, FileType::Object)
            .map_err(|e| format!("Failed to generate object code: {}", e.to_string()))?;

        // Write to file
        let object_bytes = object_data.as_slice();
        let mut file =
            File::create(filename).map_err(|e| format!("Failed to create file {filename}: {e}"))?;
        file.write_all(object_bytes)
            .map_err(|e| format!("Failed to write to file {filename}: {e}"))?;

        Ok(())
    }

    fn evaluate_fstring_codegen(
        &mut self,
        fstring: &crate::ast::FString,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        // If there are no expressions, just return the string as is
        if fstring.parts.is_empty() {
            let name = format!("str_{}", self.string_counter);
            self.string_counter += 1;
            let str_ptr = self.builder.build_global_string_ptr("", &name).unwrap();
            return Ok(str_ptr.as_pointer_value().into());
        }

        // For f-strings, we need to build a proper string instead of printing directly
        // Create a format string that will be used with sprintf to build the result
        let mut format_string = String::new();
        let mut sprintf_args: Vec<inkwell::values::BasicMetadataValueEnum<'ctx>> = Vec::new();

        // Process each part to build format string and arguments
        for part in &fstring.parts {
            match part {
                crate::ast::FStringPart::Literal(literal) => {
                    // Add literal text directly to format string
                    format_string.push_str(&literal.replace("%", "%%")); // Escape % characters
                }
                crate::ast::FStringPart::Expression(expr) => {
                    // Evaluate the expression and add appropriate format specifier
                    let expr_value = self.evaluate_fstring_expression(expr)?;
                    match expr_value {
                        BasicValueEnum::IntValue(int_val) => {
                            format_string.push_str("%ld");
                            sprintf_args.push(int_val.into());
                        }
                        BasicValueEnum::FloatValue(float_val) => {
                            format_string.push_str("%.6g");
                            sprintf_args.push(float_val.into());
                        }
                        BasicValueEnum::PointerValue(ptr_val) => {
                            format_string.push_str("%s");
                            sprintf_args.push(ptr_val.into());
                        }
                        _ => {
                            format_string.push_str("%s");
                            let name = format!("unknown_{}", self.string_counter);
                            self.string_counter += 1;
                            let str_ptr = self.builder.build_global_string_ptr("?", &name).unwrap();
                            sprintf_args.push(str_ptr.as_pointer_value().into());
                        }
                    }
                }
            }
        }

        // Allocate buffer for the result string on stack
        let result_size = format_string.len() + 256; // Extra space for formatted values
        let i8_type = self.context.i8_type();
        let result_type = i8_type.array_type(result_size as u32);
        let result_alloc = self
            .builder
            .build_alloca(result_type, "fstring_result")
            .unwrap();
        let result_ptr = self
            .builder
            .build_pointer_cast(
                result_alloc,
                self.context.ptr_type(inkwell::AddressSpace::default()),
                "result_ptr",
            )
            .unwrap();

        // Initialize the buffer to zero to prevent garbage data
        let zero = i8_type.const_int(0, false);
        let memset_fn = if let Some(func) = self.module.get_function("memset") {
            func
        } else {
            let i8_ptr_type = self.context.ptr_type(inkwell::AddressSpace::default());
            let memset_fn_type = self.context.i64_type().fn_type(
                &[
                    i8_ptr_type.into(),
                    i8_type.into(),
                    self.context.i64_type().into(),
                ],
                false,
            );
            self.module.add_function("memset", memset_fn_type, None)
        };

        let size_val = self.context.i64_type().const_int(result_size as u64, false);
        let _ = self
            .builder
            .build_call(
                memset_fn,
                &[result_ptr.into(), zero.into(), size_val.into()],
                "memset_call",
            )
            .unwrap();

        // Get or declare snprintf function for safe string formatting
        let snprintf_fn = if let Some(func) = self.module.get_function("snprintf") {
            func
        } else {
            let i32_type = self.context.i32_type();
            let str_type = self.context.ptr_type(inkwell::AddressSpace::default());
            let snprintf_fn_type =
                i32_type.fn_type(&[str_type.into(), i32_type.into(), str_type.into()], true);
            self.module.add_function("snprintf", snprintf_fn_type, None)
        };

        // Create format string global
        let format_name = format!("fmt_{}", self.string_counter);
        self.string_counter += 1;
        let format_ptr = self
            .builder
            .build_global_string_ptr(&format_string, &format_name)
            .unwrap();

        // Build snprintf call with buffer size limit
        let buffer_size = self
            .context
            .i32_type()
            .const_int((result_size - 1) as u64, false); // Leave space for null terminator
        let mut all_args: Vec<inkwell::values::BasicMetadataValueEnum<'ctx>> = vec![
            result_ptr.into(),
            buffer_size.into(),
            format_ptr.as_pointer_value().into(),
        ];
        all_args.extend(sprintf_args);

        let _ = self
            .builder
            .build_call(snprintf_fn, &all_args, "snprintf_call")
            .unwrap();

        // Return the result pointer
        Ok(result_ptr.into())
    }

    #[allow(dead_code)]
    fn concatenate_string_parts(
        &mut self,
        parts: &[BasicValueEnum<'ctx>],
    ) -> Result<BasicValueEnum<'ctx>, String> {
        // For f-strings, we need to build a format string and use printf to output the result
        // This is a simplified approach that prints directly instead of returning a string

        if parts.is_empty() {
            let name = format!("empty_{}", self.string_counter);
            self.string_counter += 1;
            let str_ptr = self.builder.build_global_string_ptr("", &name).unwrap();
            Ok(str_ptr.as_pointer_value().into())
        } else if parts.len() == 1 {
            Ok(parts[0])
        } else {
            // Build a format string and use printf to output all parts
            self.build_printf_concatenation(parts)
        }
    }

    #[allow(dead_code)]
    fn build_printf_concatenation(
        &mut self,
        parts: &[BasicValueEnum<'ctx>],
    ) -> Result<BasicValueEnum<'ctx>, String> {
        // Get or declare printf function
        let printf_fn = if let Some(func) = self.module.get_function("printf") {
            func
        } else {
            let i32_type = self.context.i32_type();
            let str_type = self.context.ptr_type(inkwell::AddressSpace::default());
            let printf_fn_type = i32_type.fn_type(&[str_type.into()], true);
            self.module.add_function("printf", printf_fn_type, None)
        };

        // Build format string and arguments
        let mut format_string = String::new();
        let mut printf_args: Vec<inkwell::values::BasicMetadataValueEnum<'ctx>> = Vec::new();

        for part in parts {
            match part {
                BasicValueEnum::PointerValue(ptr_val) => {
                    // Assume this is a string pointer
                    format_string.push_str("%s");
                    printf_args.push((*ptr_val).into());
                }
                BasicValueEnum::IntValue(int_val) => {
                    format_string.push_str("%ld");
                    printf_args.push((*int_val).into());
                }
                BasicValueEnum::FloatValue(float_val) => {
                    format_string.push_str("%f");
                    printf_args.push((*float_val).into());
                }
                _ => {
                    format_string.push_str("%s");
                    let name = format!("unknown_{}", self.string_counter);
                    self.string_counter += 1;
                    let str_ptr = self.builder.build_global_string_ptr("?", &name).unwrap();
                    printf_args.push(str_ptr.as_pointer_value().into());
                }
            }
        }

        // Add newline to the format string
        format_string.push('\n');

        // Create the format string global
        let format_name = format!("fmt_{}", self.string_counter);
        self.string_counter += 1;
        let format_ptr = self
            .builder
            .build_global_string_ptr(&format_string, &format_name)
            .unwrap();

        // Build printf call with format string as first argument
        let mut all_args: Vec<inkwell::values::BasicMetadataValueEnum<'ctx>> =
            vec![format_ptr.as_pointer_value().into()];
        all_args.extend(printf_args);

        // Call printf to output the concatenated string
        let _ = self
            .builder
            .build_call(printf_fn, &all_args, "printf_concat")
            .unwrap();

        // Return an empty string as the result (since we already printed it)
        let name = format!("empty_{}", self.string_counter);
        self.string_counter += 1;
        let str_ptr = self.builder.build_global_string_ptr("", &name).unwrap();
        Ok(str_ptr.as_pointer_value().into())
    }

    #[allow(dead_code)]
    fn extract_string_from_global(&self, _global_name: &str) -> Option<String> {
        // This is a simplified version - in a full implementation we'd
        // need to look up the global variable and extract its string value
        // For now, we'll return None to indicate we can't extract it
        None
    }

    fn evaluate_fstring_expression(&mut self, expr: &str) -> Result<BasicValueEnum<'ctx>, String> {
        // Try to parse and evaluate the expression using the existing parser
        let expr = expr.trim();

        // First, try to handle simple variable names
        if let Some((ptr, stored_value)) = self.variables.get(expr) {
            // Load the current value from the variable's memory location
            let loaded_value = self
                .builder
                .build_load(stored_value.get_type(), *ptr, &format!("load_{}", expr))
                .unwrap();

            // For string variables, we need to handle them specially
            // Check if the stored value was a string pointer
            if matches!(stored_value, BasicValueEnum::PointerValue(_)) {
                // This is a string variable, return the loaded value directly
                return Ok(loaded_value);
            } else {
                // For other types, convert to string
                return self.value_to_string(loaded_value);
            }
        }

        // Try to parse as a more complex expression
        // For now, we'll handle simple arithmetic expressions
        if let Some(parsed_expr) = self.parse_simple_expression(expr)
            && let Ok(value) = self.compile_expression(&parsed_expr)
        {
            return self.value_to_string(value);
        }

        // If all else fails, return the expression as a string literal
        let name = format!("expr_{}", self.string_counter);
        self.string_counter += 1;
        let str_ptr = self.builder.build_global_string_ptr(expr, &name).unwrap();
        Ok(str_ptr.as_pointer_value().into())
    }

    fn value_to_string(
        &mut self,
        value: BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        match value {
            BasicValueEnum::IntValue(int_val) => {
                // For runtime integer values, we need to convert them to strings using snprintf
                let name = format!("int_str_{}", self.string_counter);
                self.string_counter += 1;

                // Allocate buffer for the string representation
                let i8_type = self.context.i8_type();
                let buffer_type = i8_type.array_type(32); // Enough space for 64-bit integer
                let buffer_alloc = self.builder.build_alloca(buffer_type, &name).unwrap();
                let buffer_ptr = self
                    .builder
                    .build_pointer_cast(
                        buffer_alloc,
                        self.context.ptr_type(inkwell::AddressSpace::default()),
                        "buffer_ptr",
                    )
                    .unwrap();

                // Initialize buffer to zero
                let zero = i8_type.const_int(0, false);
                let memset_fn = if let Some(func) = self.module.get_function("memset") {
                    func
                } else {
                    let i8_ptr_type = self.context.ptr_type(inkwell::AddressSpace::default());
                    let memset_fn_type = self.context.i64_type().fn_type(
                        &[
                            i8_ptr_type.into(),
                            i8_type.into(),
                            self.context.i64_type().into(),
                        ],
                        false,
                    );
                    self.module.add_function("memset", memset_fn_type, None)
                };

                let size_val = self.context.i64_type().const_int(32, false);
                let _ = self
                    .builder
                    .build_call(
                        memset_fn,
                        &[buffer_ptr.into(), zero.into(), size_val.into()],
                        "memset_int",
                    )
                    .unwrap();

                // Get or declare snprintf function
                let snprintf_fn = if let Some(func) = self.module.get_function("snprintf") {
                    func
                } else {
                    let i32_type = self.context.i32_type();
                    let str_type = self.context.ptr_type(inkwell::AddressSpace::default());
                    let snprintf_fn_type = i32_type
                        .fn_type(&[str_type.into(), i32_type.into(), str_type.into()], true);
                    self.module.add_function("snprintf", snprintf_fn_type, None)
                };

                // Create format string for integer
                let format_name = format!("int_fmt_{}", self.string_counter);
                self.string_counter += 1;
                let format_ptr = self
                    .builder
                    .build_global_string_ptr("%ld", &format_name)
                    .unwrap();

                // Call snprintf to convert integer to string
                let buffer_size = self.context.i32_type().const_int(32, false);
                let _ = self
                    .builder
                    .build_call(
                        snprintf_fn,
                        &[
                            buffer_ptr.into(),
                            buffer_size.into(),
                            format_ptr.as_pointer_value().into(),
                            int_val.into(),
                        ],
                        "snprintf_call",
                    )
                    .unwrap();

                Ok(buffer_ptr.into())
            }
            BasicValueEnum::FloatValue(float_val) => {
                // For runtime float values, we need to convert them to strings using snprintf
                let name = format!("float_str_{}", self.string_counter);
                self.string_counter += 1;

                // Allocate buffer for the string representation
                let i8_type = self.context.i8_type();
                let buffer_type = i8_type.array_type(64); // Enough space for float
                let buffer_alloc = self.builder.build_alloca(buffer_type, &name).unwrap();
                let buffer_ptr = self
                    .builder
                    .build_pointer_cast(
                        buffer_alloc,
                        self.context.ptr_type(inkwell::AddressSpace::default()),
                        "buffer_ptr",
                    )
                    .unwrap();

                // Initialize buffer to zero
                let zero = i8_type.const_int(0, false);
                let memset_fn = if let Some(func) = self.module.get_function("memset") {
                    func
                } else {
                    let i8_ptr_type = self.context.ptr_type(inkwell::AddressSpace::default());
                    let memset_fn_type = self.context.i64_type().fn_type(
                        &[
                            i8_ptr_type.into(),
                            i8_type.into(),
                            self.context.i64_type().into(),
                        ],
                        false,
                    );
                    self.module.add_function("memset", memset_fn_type, None)
                };

                let size_val = self.context.i64_type().const_int(64, false);
                let _ = self
                    .builder
                    .build_call(
                        memset_fn,
                        &[buffer_ptr.into(), zero.into(), size_val.into()],
                        "memset_float",
                    )
                    .unwrap();

                // Get or declare snprintf function
                let snprintf_fn = if let Some(func) = self.module.get_function("snprintf") {
                    func
                } else {
                    let i32_type = self.context.i32_type();
                    let str_type = self.context.ptr_type(inkwell::AddressSpace::default());
                    let snprintf_fn_type = i32_type
                        .fn_type(&[str_type.into(), i32_type.into(), str_type.into()], true);
                    self.module.add_function("snprintf", snprintf_fn_type, None)
                };

                // Create format string for float
                let format_name = format!("float_fmt_{}", self.string_counter);
                self.string_counter += 1;
                let format_ptr = self
                    .builder
                    .build_global_string_ptr("%.6g", &format_name)
                    .unwrap();

                // Call snprintf to convert float to string
                let buffer_size = self.context.i32_type().const_int(64, false);
                let _ = self
                    .builder
                    .build_call(
                        snprintf_fn,
                        &[
                            buffer_ptr.into(),
                            buffer_size.into(),
                            format_ptr.as_pointer_value().into(),
                            float_val.into(),
                        ],
                        "snprintf_call",
                    )
                    .unwrap();

                Ok(buffer_ptr.into())
            }
            BasicValueEnum::PointerValue(ptr_val) => {
                // Assume this is already a string pointer
                Ok(BasicValueEnum::PointerValue(ptr_val))
            }
            _ => {
                let name = format!("unknown_{}", self.string_counter);
                self.string_counter += 1;
                let str_ptr = self
                    .builder
                    .build_global_string_ptr("unknown", &name)
                    .unwrap();
                Ok(str_ptr.as_pointer_value().into())
            }
        }
    }

    fn parse_simple_expression(&self, expr: &str) -> Option<Node> {
        // Very simple expression parser for basic arithmetic
        // This is a simplified version - a full implementation would use the actual parser

        // Try to parse as integer
        if let Ok(int_val) = expr.parse::<i64>() {
            return Some(Node::Literal(Literal {
                value: LiteralValue::Integer(int_val),
            }));
        }

        // Try to parse as float
        if let Ok(float_val) = expr.parse::<f64>() {
            return Some(Node::Literal(Literal {
                value: LiteralValue::Float(float_val),
            }));
        }

        // Try to parse as simple binary expression (e.g., "a + b")
        // Only handle very simple cases to avoid recursion
        if let Some((left_str, op_str, right_str)) = self.parse_binary_expression(expr)
            && let Some(left_node) = self.parse_simple_expression(left_str.trim())
            && let Some(right_node) = self.parse_simple_expression(right_str.trim())
        {
            let operator = match op_str.trim() {
                "+" => Some(BinaryOperator::Add),
                "-" => Some(BinaryOperator::Subtract),
                "*" => Some(BinaryOperator::Multiply),
                "/" => Some(BinaryOperator::Divide),
                "//" => Some(BinaryOperator::FloorDivide),
                "%" => Some(BinaryOperator::Modulo),
                "**" => Some(BinaryOperator::Power),
                _ => None,
            };

            if let Some(op) = operator {
                return Some(Node::Binary(Binary {
                    left: Box::new(left_node),
                    operator: op,
                    right: Box::new(right_node),
                }));
            }
        }

        // Try to parse as identifier
        if expr.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Some(Node::Identifier(Identifier {
                name: expr.to_string(),
            }));
        }

        None
    }

    #[allow(dead_code)]
    fn parse_complex_expression(&self, expr: &str) -> Option<Node> {
        // For now, just try simple parsing to avoid recursion issues
        // If it's too complex, return None and let the caller handle it as a string
        let expr = expr.trim();

        // Only handle very simple cases
        if expr.contains('(') || expr.contains('*') || expr.contains('/') {
            return None; // Too complex for now
        }

        // Try to parse as simple binary expression
        if let Some((left_str, op_str, right_str)) = self.parse_binary_expression(expr)
            && let Some(left_node) = self.parse_simple_expression(left_str.trim())
            && let Some(right_node) = self.parse_simple_expression(right_str.trim())
        {
            let operator = match op_str.trim() {
                "+" => Some(BinaryOperator::Add),
                "-" => Some(BinaryOperator::Subtract),
                "*" => Some(BinaryOperator::Multiply),
                "/" => Some(BinaryOperator::Divide),
                "//" => Some(BinaryOperator::FloorDivide),
                "%" => Some(BinaryOperator::Modulo),
                "**" => Some(BinaryOperator::Power),
                _ => None,
            };

            if let Some(op) = operator {
                return Some(Node::Binary(Binary {
                    left: Box::new(left_node),
                    operator: op,
                    right: Box::new(right_node),
                }));
            }
        }

        // If not a binary expression, try to parse as simple expression
        self.parse_simple_expression(expr)
    }

    fn parse_binary_expression(&self, expr: &str) -> Option<(String, String, String)> {
        // Simple binary expression parser
        // Look for common operators
        let operators = ["**", "//", "+", "-", "*", "/", "%"];

        for op in &operators {
            if let Some(pos) = expr.find(op)
                && pos > 0
                && pos + op.len() < expr.len()
            {
                let left = expr[..pos].to_string();
                let right = expr[pos + op.len()..].to_string();
                return Some((left, op.to_string(), right));
            }
        }

        None
    }

    fn concatenate_strings(
        &mut self,
        left: inkwell::values::PointerValue<'ctx>,
        right: inkwell::values::PointerValue<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        // Get or declare strlen function to get string lengths
        let strlen_fn = if let Some(func) = self.module.get_function("strlen") {
            func
        } else {
            let i32_type = self.context.i32_type();
            let str_type = self.context.ptr_type(inkwell::AddressSpace::default());
            let strlen_fn_type = i32_type.fn_type(&[str_type.into()], false);
            self.module.add_function("strlen", strlen_fn_type, None)
        };

        // Get or declare malloc function for memory allocation
        let malloc_fn = if let Some(func) = self.module.get_function("malloc") {
            func
        } else {
            let i8_ptr_type = self.context.ptr_type(inkwell::AddressSpace::default());
            let malloc_fn_type = i8_ptr_type.fn_type(&[self.context.i64_type().into()], false);
            self.module.add_function("malloc", malloc_fn_type, None)
        };

        // Get or declare strcpy function for string copying
        let strcpy_fn = if let Some(func) = self.module.get_function("strcpy") {
            func
        } else {
            let i8_ptr_type = self.context.ptr_type(inkwell::AddressSpace::default());
            let strcpy_fn_type =
                i8_ptr_type.fn_type(&[i8_ptr_type.into(), i8_ptr_type.into()], false);
            self.module.add_function("strcpy", strcpy_fn_type, None)
        };

        // Get or declare strcat function for string concatenation
        let strcat_fn = if let Some(func) = self.module.get_function("strcat") {
            func
        } else {
            let i8_ptr_type = self.context.ptr_type(inkwell::AddressSpace::default());
            let strcat_fn_type =
                i8_ptr_type.fn_type(&[i8_ptr_type.into(), i8_ptr_type.into()], false);
            self.module.add_function("strcat", strcat_fn_type, None)
        };

        // Calculate lengths of both strings
        let left_len = self
            .builder
            .build_call(strlen_fn, &[left.into()], "left_len")
            .unwrap()
            .try_as_basic_value()
            .unwrap_basic()
            .into_int_value();

        let right_len = self
            .builder
            .build_call(strlen_fn, &[right.into()], "right_len")
            .unwrap()
            .try_as_basic_value()
            .unwrap_basic()
            .into_int_value();

        // Calculate total length (left + right + 1 for null terminator)
        let total_len = self
            .builder
            .build_int_add(left_len, right_len, "total_len")
            .unwrap();
        let total_len_with_null = self
            .builder
            .build_int_add(
                total_len,
                self.context.i32_type().const_int(1, false),
                "total_len_with_null",
            )
            .unwrap();

        // Convert to i64 for malloc
        let malloc_size = self
            .builder
            .build_int_cast(total_len_with_null, self.context.i64_type(), "malloc_size")
            .unwrap();

        // Allocate memory for the concatenated string
        let result_ptr = self
            .builder
            .build_call(malloc_fn, &[malloc_size.into()], "result_ptr")
            .unwrap()
            .try_as_basic_value()
            .unwrap_basic()
            .into_pointer_value();

        // Copy left string to result
        let _ = self
            .builder
            .build_call(strcpy_fn, &[result_ptr.into(), left.into()], "strcpy_left")
            .unwrap();

        // Concatenate right string to result
        let _ = self
            .builder
            .build_call(
                strcat_fn,
                &[result_ptr.into(), right.into()],
                "strcat_right",
            )
            .unwrap();

        Ok(result_ptr.into())
    }
}
