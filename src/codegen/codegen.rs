use crate::ast::{BinaryOperator, LiteralValue, Node};
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

                // Allocate space for the variable on the stack
                let ptr = self
                    .builder
                    .build_alloca(value.get_type(), &assignment.name)
                    .unwrap();
                self.builder.build_store(ptr, value).unwrap();
                self.variables.insert(assignment.name.clone(), (ptr, value));
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

        // Create function type - for now we'll assume all functions return i64 and take i64 parameters
        let int_type = self.context.i64_type();
        let param_types: Vec<_> = function
            .parameters
            .iter()
            .map(|_| int_type.into())
            .collect();
        let fn_type = int_type.fn_type(&param_types, false);

        // Create function
        let function_value = self.module.add_function(&function.name, fn_type, None);

        // Create basic block
        let basic_block = self.context.append_basic_block(function_value, "entry");
        self.builder.position_at_end(basic_block);

        // Create allocations for parameters
        for (i, param_name) in function.parameters.iter().enumerate() {
            let param = function_value.get_nth_param(i as u32).unwrap();
            let ptr = self.builder.build_alloca(int_type, param_name).unwrap();
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
                .build_return(Some(&int_type.const_int(0, false)))
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
                    LiteralValue::FString(value) => {
                        // Handle f-string by parsing and evaluating expressions
                        let evaluated_string = self.evaluate_fstring_codegen(value)?;
                        Ok(evaluated_string)
                    }
                    LiteralValue::Boolean(value) => {
                        let bool_type = self.context.bool_type();
                        Ok(bool_type.const_int(*value as u64, false).into())
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
                                Ok(BasicValueEnum::IntValue(l))
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
                        (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => {
                            Ok(BasicValueEnum::IntValue(l))
                        }
                        (BasicValueEnum::FloatValue(l), BasicValueEnum::FloatValue(r)) => {
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
                    Ok(call_result.try_as_basic_value().left().unwrap())
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
                                // Create format string for integer
                                let name = format!("fmt_{}", self.string_counter);
                                self.string_counter += 1;
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
                            }
                            BasicValueEnum::FloatValue(float_val) => {
                                // Create format string for float
                                let name = format!("fmt_{}", self.string_counter);
                                self.string_counter += 1;
                                let format_str =
                                    self.builder.build_global_string_ptr("%f\n", &name).unwrap();
                                let _ = self
                                    .builder
                                    .build_call(
                                        printf_fn,
                                        &[format_str.as_pointer_value().into(), float_val.into()],
                                        "printf",
                                    )
                                    .unwrap();
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

    pub fn get_target_machine(
        &self,
        optimization: u8,
    ) -> Result<inkwell::targets::TargetMachine, String> {
        use inkwell::OptimizationLevel;
        use inkwell::targets::{InitializationConfig, Target, TargetMachine};

        // Initialize LLVM targets
        let config = InitializationConfig::default();
        Target::initialize_all(&config);

        // Get the target triple for the current machine
        let target_triple = TargetMachine::get_default_triple();
        let target = Target::from_triple(&target_triple)
            .map_err(|e| format!("Failed to get target: {}", e.to_string()))?;

        // Set optimization level
        let opt_level = match optimization {
            0 => OptimizationLevel::None,
            1 => OptimizationLevel::Less,
            2 => OptimizationLevel::Default,
            3 => OptimizationLevel::Aggressive,
            _ => OptimizationLevel::Default,
        };

        // Create target machine
        let target_machine = target
            .create_target_machine(
                &target_triple,
                "generic",
                "",
                opt_level,
                inkwell::targets::RelocMode::Default,
                inkwell::targets::CodeModel::Default,
            )
            .ok_or("Failed to create target machine")?;

        Ok(target_machine)
    }

    fn evaluate_fstring_codegen(&mut self, fstring: &str) -> Result<BasicValueEnum<'ctx>, String> {
        // Parse f-string and evaluate expressions
        let mut literal_parts = Vec::new();
        let mut expressions = Vec::new();
        let mut current_part = String::new();
        let mut in_expression = false;
        let chars = fstring.chars().peekable();

        for ch in chars {
            if in_expression {
                if ch == '}' {
                    // End of expression
                    expressions.push(current_part.clone());
                    current_part.clear();
                    in_expression = false;
                } else {
                    current_part.push(ch);
                }
            } else if ch == '{' {
                // Start of expression
                if !current_part.is_empty() {
                    literal_parts.push(current_part.clone());
                    current_part.clear();
                }
                in_expression = true;
            } else {
                current_part.push(ch);
            }
        }

        // Add the final literal part if it exists
        if !current_part.is_empty() {
            literal_parts.push(current_part.clone());
        }

        // If there are no expressions, just return the string as is
        if expressions.is_empty() {
            let name = format!("str_{}", self.string_counter);
            self.string_counter += 1;
            let str_ptr = self
                .builder
                .build_global_string_ptr(fstring, &name)
                .unwrap();
            return Ok(str_ptr.as_pointer_value().into());
        }

        // Create a vector to hold all parts (literals and evaluated expressions)
        let mut all_parts = Vec::new();

        // Process literals and expressions alternately
        let max_parts = literal_parts.len().max(expressions.len());
        for i in 0..max_parts {
            // Add literal part if it exists
            if i < literal_parts.len() {
                let name = format!("lit_{}", self.string_counter);
                self.string_counter += 1;
                let str_ptr = self
                    .builder
                    .build_global_string_ptr(&literal_parts[i], &name)
                    .unwrap();
                all_parts.push(str_ptr.as_pointer_value().into());
            }

            // Add expression result if it exists
            if i < expressions.len() {
                let expr_value = self.evaluate_fstring_expression(&expressions[i])?;
                all_parts.push(expr_value);
            }
        }

        // Concatenate all parts
        if all_parts.is_empty() {
            let name = format!("empty_{}", self.string_counter);
            self.string_counter += 1;
            let str_ptr = self.builder.build_global_string_ptr("", &name).unwrap();
            Ok(str_ptr.as_pointer_value().into())
        } else {
            // For now, just return the first part to see what it contains
            Ok(all_parts[0])
        }
    }

    fn evaluate_fstring_expression(&mut self, expr: &str) -> Result<BasicValueEnum<'ctx>, String> {
        // For now, we'll handle simple variable names
        // A full implementation would parse and evaluate complex expressions
        let expr = expr.trim();
        if let Some((_, value)) = self.variables.get(expr) {
            // Return the value of the variable as a string representation
            match value {
                BasicValueEnum::IntValue(int_val) => {
                    // Convert integer to string
                    let name = format!("int_str_{}", self.string_counter);
                    self.string_counter += 1;
                    if let Some(val) = int_val.get_zero_extended_constant() {
                        let str_val = val.to_string();
                        let str_ptr = self
                            .builder
                            .build_global_string_ptr(&str_val, &name)
                            .unwrap();
                        Ok(str_ptr.as_pointer_value().into())
                    } else {
                        let str_ptr = self.builder.build_global_string_ptr("0", &name).unwrap();
                        Ok(str_ptr.as_pointer_value().into())
                    }
                }
                BasicValueEnum::FloatValue(float_val) => {
                    // Convert float to string
                    let name = format!("float_str_{}", self.string_counter);
                    self.string_counter += 1;
                    // This is a simplification - in a real implementation we'd need proper float formatting
                    let (float_val, _) = float_val.get_constant().unwrap_or((0.0, false));
                    let str_val = format!("{float_val:.6}");
                    let str_ptr = self
                        .builder
                        .build_global_string_ptr(&str_val, &name)
                        .unwrap();
                    Ok(str_ptr.as_pointer_value().into())
                }
                BasicValueEnum::PointerValue(ptr_val) => {
                    // Assume this is already a string pointer
                    Ok(BasicValueEnum::PointerValue(*ptr_val))
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
        } else {
            // If not found as a variable, return the expression as a string
            let name = format!("expr_{}", self.string_counter);
            self.string_counter += 1;
            let str_ptr = self.builder.build_global_string_ptr(expr, &name).unwrap();
            Ok(str_ptr.as_pointer_value().into())
        }
    }

    fn concatenate_strings(
        &mut self,
        parts: &[BasicValueEnum<'ctx>],
    ) -> Result<BasicValueEnum<'ctx>, String> {
        if parts.is_empty() {
            let name = format!("empty_{}", self.string_counter);
            self.string_counter += 1;
            let str_ptr = self.builder.build_global_string_ptr("", &name).unwrap();
            return Ok(str_ptr.as_pointer_value().into());
        }

        if parts.len() == 1 {
            return Ok(parts[0]);
        }

        // For now, we'll create a simple concatenated string representation
        // In a full implementation, we would use LLVM's string concatenation functions
        // For now, we'll just return the first part as a placeholder
        Ok(parts[0])
    }
}
