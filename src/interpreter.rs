use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Value {
    Bool(bool),
    Str(String),
    Num(f64),
}

fn evaluate_variable(token: &str, variables: &HashMap<String, Value>) -> Option<Value> {
    if token.starts_with('$') { variables.get(&token[1..]).cloned() } else { None }
}

// this is to evaluate expressions in the PE/RO cases (this is quite an inflexible solution as you might notice)
fn evaluate_expression(expr: &str, variables: &HashMap<String, Value>) -> Value {
    let expr = expr.trim_matches('"');
    let parts: Vec<&str> = expr.trim().split_whitespace().collect();

    if parts.len() != 3 {
        panic!("[ERROR: InvalidExpression]: Expected format 'value operator value'");
    }

    let left = match parts[0] {
        s if s.starts_with('$') => match variables.get(&s[1..]) {
            Some(Value::Num(n)) => *n,
            Some(Value::Bool(b)) => if *b { 1.0 } else { 0.0 },
            _ => panic!("[ERROR: InvalidValue]: Variable not found or invalid type")
        },
        "FLUFFY" => 1.0,
        "FUZZY" => 0.0,
        s => match s.parse::<f64>() {
            Ok(n) => n,
            Err(_) => panic!("[ERROR: InvalidValue]: '{}\n' is not a valid number", s)
        }
    };

    let right = match parts[2] {
        s if s.starts_with('$') => match variables.get(&s[1..]) {
            Some(Value::Num(n)) => *n,
            Some(Value::Bool(b)) => if *b { 1.0 } else { 0.0 },
            _ => panic!("[ERROR: InvalidValue]: Variable not found or invalid type")
        },
        "FLUFFY" => 1.0,
        "FUZZY" => 0.0,
        s => match s.parse::<f64>() {
            Ok(n) => n,
            Err(_) => panic!("[ERROR: InvalidValue]: '{}\n' is not a valid number", s)
        }
    };

    match parts[1] {
        "+" => Value::Num(left + right),
        "-" => Value::Num(left - right),
        "*" => Value::Num(left * right),
        "/" => Value::Num(left / right),
        "%" => Value::Num(left % right),
        "==" => Value::Bool((left - right).abs() < f64::EPSILON),
        ">" => Value::Bool(left > right),
        "<" => Value::Bool(left < right),
        ">=" => Value::Bool(left >= right),
        "<=" => Value::Bool(left <= right),
        "!=" => Value::Bool((left - right).abs() >= f64::EPSILON),
        _ => panic!("[ERROR: InvalidOperator]: Unsupported operator")
    }
}
pub fn run_interpreter(code: &str, _variables: &mut HashMap<String, Value>, output: &mut String) {
    let mut variables: HashMap<String, Value> = HashMap::new();
    let mut tokens = Vec::new();
    let mut in_quote = false;
    let mut current_token = String::new();

    let mut skip_line = false;
    for c in code.chars() {
        if skip_line {
            if c == '\n' {
                skip_line = false;
            }
            continue;
        }

        match c {
            ';' => skip_line = true,
            '"' => {
                in_quote = !in_quote;
                current_token.push(c);
                if !in_quote {
                    tokens.push(current_token.clone());
                    current_token.clear();
                }
            }
            c if c.is_whitespace() && !in_quote => {
                if !current_token.is_empty() {
                    tokens.push(current_token.clone());
                    current_token.clear();
                }
            }
            _ => current_token.push(c),
        }
    }

    if !current_token.is_empty() { tokens.push(current_token); }

    let mut pc = 0;
    let mut suppress_class_messages = true;
    let mut condition_stack = Vec::new();
    let mut block_executed = false;

    // check some basic debug info, e.g. end of a class
    if tokens.get(0) == Some(&"CHIHUAHUA".to_string()) {
        suppress_class_messages = false;
        pc += 1;
    }

    while pc < tokens.len() {
        let should_execute = condition_stack.last().copied().unwrap_or(true);

        match tokens.get(pc).map(String::as_str) {
            Some("WA") if pc + 3 < tokens.len() => {
                if should_execute {
                    pc += 1;
                    let var_type = &tokens[pc];
                    pc += 1;
                    let var_name = &tokens[pc];
                    pc += 1;
                    let var_value = &tokens[pc];

                    let value = match var_type.as_str() {
                        "KIRA" => {
                            if var_value.starts_with('"') && var_value.ends_with('"') {
                                Value::Str(var_value[1..var_value.len()-1].to_string())
                            } else { panic!("[ERROR: IncompatibleType]: KIRA requires quoted string") }
                        },
                        "BAULEAN" => match var_value.as_str() {
                            "FLUFFY" => Value::Bool(true),
                            "FUZZY" => Value::Bool(false),
                            expr if expr.contains(' ') => {
                                match evaluate_expression(expr, &variables) {
                                    Value::Bool(b) => Value::Bool(b),
                                    _ => panic!("[ERROR: IncompatibleType]: Expression must evaluate to boolean")
                                }
                            },
                            _ => panic!("[ERROR: IncompleteStatement]: BAULEAN requires FLUFFY/FUZZY or expression")
                        },
                        "MOE" => {
                            if var_value.contains(' ') {
                                match evaluate_expression(var_value, &variables) {
                                    Value::Num(n) => Value::Num(n),
                                    _ => panic!("[ERROR: IncompatibleType]: Expression must evaluate to number")
                                }
                            } else if var_value.starts_with('$') {
                                match variables.get(&var_value[1..]) {
                                    Some(value) => value.clone(),
                                    None => panic!("[ERROR: VanishValue]: Variable not found")
                                }
                            } else {
                                match var_value.parse() {
                                    Ok(num) => Value::Num(num),
                                    Err(_) => panic!("[ERROR: IncompleteStatement]: MOE requires number or expression")
                                }
                            }
                        },
                        _ => panic!("Unknown type: {}\n", var_type)
                    };
                    variables.insert(var_name.to_string(), value);
                }
                pc += 1;
            },

            Some("BAU") if pc + 1 < tokens.len() => {
                pc += 1;
                if should_execute {
                    let token = &tokens[pc];
                    if token.starts_with('$') {
                        match variables.get(&token[1..]) {
                            Some(Value::Str(s)) => output.push_str(&format!("{}\n", s)),
                            Some(Value::Bool(b)) => output.push_str(&format!("{}\n", b)),
                            Some(Value::Num(n)) => output.push_str(&format!("{}\n", n)),
                            None => panic!("[ERROR: VanishValue]: Variable not found: {}\n", token)
                        }
                    } else if token.starts_with('"') && token.ends_with('"') {
                        output.push_str(&format!("{}\n", &token[1..token.len() - 1]));
                    } else {
                        panic!("[ERROR: IncompleteStatement]: BAU requires quoted string or variable");
                    }
                }
                pc += 1;
            },

            // very scuffed reassignment operation, must fix
            Some("CO") if pc + 1 < tokens.len() => {
                pc += 1;
                if should_execute {
                    let var_name = &tokens[pc];
                    pc += 1;
                    let new_value = &tokens[pc];

                    if let Some(existing_var) = variables.get(var_name) {

                        let value = match existing_var {

                            Value::Str(_) if new_value.starts_with('"') && new_value.ends_with('"') =>
                                Value::Str(new_value[1..new_value.len()-1].to_string()),

                            Value::Bool(_) => match new_value.as_str() {
                                "FLUFFY" => Value::Bool(true),
                                "FUZZY" => Value::Bool(false),
                                _ => panic!("[ERROR: IncompleteStatement]: Boolean assignment requires FLUFFY/FUZZY")
                            },

                            Value::Num(_) => {

                                match new_value.parse() {
                                    Ok(num) => Value::Num(num),
                                    Err(_) => {

                                        match evaluate_expression(new_value, &variables) {
                                            Value::Num(n) => Value::Num(n),
                                            _ => panic!("[ERROR: IncompatibleType]: Expression must evaluate to number for MOE assignment")
                                        }
                                    }
                                }
                            },
                            _ => panic!("[ERROR: InvalidAssignment]"),
                        };

                        variables.insert(var_name.to_string(), value);
                    } else {

                        let value = if new_value.contains(' ') {

                            match evaluate_expression(new_value, &variables) {
                                Value::Num(n) => Value::Num(n),
                                _ => panic!("[ERROR: IncompatibleType]: Expression must evaluate to a number for MOE assignment")
                            }
                        } else if new_value.starts_with('$') {

                            match variables.get(&new_value[1..]) {
                                Some(value) => value.clone(),
                                None => panic!("[ERROR: VanishValue]: Variable not found")
                            }
                        } else {

                            match new_value.parse() {
                                Ok(num) => Value::Num(num),
                                Err(_) => panic!("[ERROR: IncompleteStatement]: MOE assignment requires a numeric value or expression")
                            }
                        };

                        variables.insert(var_name.to_string(), value);
                    }
                }
                pc += 1;
            },

            Some("PONDE") if pc + 1 < tokens.len() => {
                pc += 1;
                if should_execute {
                    let iterations = if tokens[pc].starts_with('$') {
                        let var_name = &tokens[pc][1..];
                        match variables.get(var_name) {
                            Some(Value::Bool(b)) => if *b { 1 } else { 0 },
                            Some(Value::Num(n)) => *n as i32,
                            _ => panic!("[ERROR: IncompatibleType]: PONDE requires numeric/boolean value"),
                        }
                    } else {
                        match tokens[pc].as_str() {
                            "FLUFFY" => 1,
                            "FUZZY" => 0,
                            _ => tokens[pc].parse::<i32>().unwrap_or_else(|_| panic!("[ERROR: IncompleteStatement]: PONDE requires number/FLUFFY/FUZZY")),
                        }
                    };

                    let loop_start = pc + 1;
                    for _ in 0..iterations {
                        let mut inner_pc = loop_start;
                        while inner_pc < tokens.len() && tokens[inner_pc] != "ENDPONDE" {

                            if tokens[inner_pc] == "CO" && inner_pc + 2 < tokens.len() {
                                pc = inner_pc;
                                if should_execute {
                                    pc += 1;
                                    let var_name = &tokens[pc];
                                    pc += 1;
                                    let new_value = &tokens[pc];

                                    let value = if new_value.contains(' ') {

                                        match evaluate_expression(new_value, &variables) {
                                            Value::Num(n) => Value::Num(n),
                                            Value::Str(s) => Value::Str(s),
                                            Value::Bool(b) => Value::Bool(b),
                                        }
                                    } else {

                                        match new_value.as_str() {
                                            "FLUFFY" => Value::Bool(true),
                                            "FUZZY" => Value::Bool(false),
                                            _ => {
                                                if let Ok(num) = new_value.parse::<f64>() {
                                                    Value::Num(num)
                                                } else if new_value.starts_with('"') && new_value.ends_with('"') {
                                                    Value::Str(new_value[1..new_value.len()-1].to_string())
                                                } else {
                                                    panic!("[ERROR: IncompatibleType]: Unable to parse value '{}\n'", new_value);
                                                }
                                            }
                                        }
                                    };

                                    variables.insert(var_name.to_string(), value);
                                }
                                pc += 1;
                            }

                            if tokens[inner_pc] == "BAU" && inner_pc + 1 < tokens.len() {
                                let token = &tokens[inner_pc + 1];
                                if token.starts_with('$') {
                                    match variables.get(&token[1..]) {
                                        Some(Value::Str(s)) => output.push_str(&format!("{}\n", s)),
                                        Some(Value::Bool(b)) => output.push_str(&format!("{}\n", b)),
                                        Some(Value::Num(n)) => output.push_str(&format!("{}\n", n)),
                                        None => panic!("[ERROR: VanishValue]: Variable not found: {}\n", token)
                                    }
                                } else if token.starts_with('"') && token.ends_with('"') {
                                    output.push_str(&format!("{}\n", &token[1..token.len() - 1]));
                                } else {
                                    panic!("[ERROR: IncompleteStatement]: BAU requires quoted string or variable");
                                }
                                inner_pc += 1;
                            }
                            inner_pc += 1;
                        }
                    }
                }
                pc += 1;
            }

            // Classes are kind of useless currently
            Some("FUWA") if pc + 2 < tokens.len() => {
                pc += 1;
                if tokens[pc] == ">" {
                    pc += 1;
                    if !suppress_class_messages {
                        output.push_str(&format!("Class: {}\n", tokens[pc]));
                    }
                }
                pc += 1;
            },

            Some("MOCO") => {
                if !suppress_class_messages {
                    output.push_str(&"End class\n".to_string());
                }
                condition_stack.clear();
                block_executed = false;
                pc += 1;
            },

            // if case, supports BAULEAN and comparison operators
            Some("PE") if pc + 1 < tokens.len() => {
                pc += 1;
                let condition = if tokens[pc].contains(' ') {
                    match evaluate_expression(&tokens[pc], &variables) {
                        Value::Bool(b) => b,
                        _ => panic!("[ERROR: IncompatibleType]: Expression must evaluate to boolean")
                    }
                } else if tokens[pc].starts_with('$') {
                    match variables.get(&tokens[pc][1..]) {
                        Some(Value::Bool(b)) => *b,
                        _ => panic!("[ERROR: IncompatibleType]: PE requires a boolean variable")
                    }
                } else {
                    match tokens[pc].as_str() {
                        "FLUFFY" => true,
                        "FUZZY" => false,
                        _ => panic!("[ERROR: IncompatibleType]: PE requires boolean expression or FLUFFY/FUZZY")
                    }
                };

                condition_stack.push(condition);
                block_executed = condition;
                pc += 1;
            },

            // else if
            Some("ROPE") if pc + 1 < tokens.len() => {
                condition_stack.pop();
                pc += 1;
                let condition = match tokens[pc].as_str() {
                    "FLUFFY" => true,
                    "FUZZY" => false,
                    _ => {
                        let var_value = evaluate_variable(&*tokens[pc], &variables);
                        match var_value {
                            Some(Value::Bool(b)) => b,
                            _ => panic!("[ERROR: IncompatibleType]: ROPE requires BAULEAN")
                        }
                    }
                };

                let should_run = !block_executed;
                condition_stack.push(condition && should_run);
                if condition && should_run {
                    block_executed = true;
                }
                pc += 1;
            },

            // else
            Some("RO") => {
                condition_stack.pop();

                let condition = !block_executed;

                condition_stack.push(condition);
                if !block_executed {
                    block_executed = true;
                }

                pc += 1;
            },

            // ends program
            Some("NOEH") => {
                output.push_str(&format!("Ended program with code NOEH"));
                return;
            },

            Some(token) => {
                if should_execute {
                    output.push_str(&format!("[ERROR: Unknown token]: {}\n", token));
                }
                pc += 1;
            },

            None => break,
        }
    }
}