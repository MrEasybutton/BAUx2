use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Value {
    Bool(bool),
    Str(String),
    Num(f64),
}

fn evaluate_arithmetic(expr: &str, variables: &HashMap<String, Value>) -> Result<f64, String> {
    let parts: Vec<&str> = expr.trim().split_whitespace().collect();

    if parts.len() != 3 {
        if parts.len() == 1 {
            return match parts[0].parse() {
                Ok(n) => Ok(n),
                Err(_) => Err("[ERROR: InvalidValue]: Invalid number/expression".to_string()),
            };
        }
        return Err("[ERROR: InvalidExpression]: Expecting 'value operator value'".to_string());
    }

    let left = evaluate_operand(parts[0], variables)?;
    let right = evaluate_operand(parts[2], variables)?;

    match parts[1] {
        "+" => Ok(left + right),
        "-" => Ok(left - right),
        "*" => Ok(left * right),
        "/" => Ok(left / right),
        "%" => Ok(left % right),
        _ => Err("[ERROR: InvalidOperator]: Operator is not supported".to_string()),
    }
}

fn evaluate_operand(operand: &str, variables: &HashMap<String, Value>) -> Result<f64, String> {
    match operand {
        s if variables.contains_key(s) => {
            match variables.get(s) {
                Some(Value::Num(n)) => Ok(*n),
                Some(Value::Bool(b)) => Ok(if *b { 1.0 } else { 0.0 }),
                _ => Err("[ERROR: InvalidValue]: Variable not found or invalid type".to_string()),
            }
        }
        "FLUFFY" => Ok(1.0),
        "FUZZY" => Ok(0.0),
        s =>
            match s.parse::<f64>() {
                Ok(n) => Ok(n),
                Err(_) => Err(format!("[ERROR: InvalidValue]: '{}' is an invalid number", s)),
            }
    }
}

pub fn run_interpreter(code: &str, variables: &mut HashMap<String, Value>, output: &mut String) {
    let mut tokens = Vec::new();
    let mut in_quote = false;
    let mut in_arithmetic = false;
    let mut current_token = String::new();
    let mut arithmetic_expr = String::new();
    let mut skip_line = false;

    for c in code.chars() {
        if skip_line {
            if c == '\n' {
                skip_line = false;
            }
            continue;
        }

        match c {
            ';' => {
                skip_line = true;
            }
            '<' if !in_quote => {
                if !current_token.is_empty() {
                    tokens.push(current_token.clone());
                    current_token.clear();
                }
                in_arithmetic = true;
            }
            '>' if !in_quote && in_arithmetic => {
                if !arithmetic_expr.is_empty() {
                    tokens.push(format!("<{}>", arithmetic_expr.trim()));
                    arithmetic_expr.clear();
                }
                in_arithmetic = false;
            }
            '"' => {
                if !in_arithmetic {
                    in_quote = !in_quote;
                    current_token.push(c);
                    if !in_quote {
                        tokens.push(current_token.clone());
                        current_token.clear();
                    }
                } else {
                    arithmetic_expr.push(c);
                }
            }
            '=' if !in_quote && !in_arithmetic => {
                if !current_token.is_empty() {
                    tokens.push(current_token.clone());
                    current_token.clear();
                }
                tokens.push("=".to_string());
            }
            c if c.is_whitespace() && !in_quote && !in_arithmetic => {
                if !current_token.is_empty() {
                    tokens.push(current_token.clone());
                    current_token.clear();
                }
            }
            c if in_arithmetic => arithmetic_expr.push(c),
            _ => current_token.push(c),
        }
    }

    if !current_token.is_empty() {
        tokens.push(current_token);
    }
    if !arithmetic_expr.is_empty() {
        tokens.push(format!("<{}>", arithmetic_expr.trim()));
    }

    let mut pc = 0;
    let mut suppress_class_messages = true;
    let mut condition_stack = Vec::new();

    if tokens.get(0) == Some(&"CHIHUAHUA".to_string()) {
        suppress_class_messages = false;
        pc += 1;
    }

    while pc < tokens.len() {
        let should_execute = condition_stack.last().copied().unwrap_or(true);

        match tokens.get(pc).map(String::as_str) {
            Some("WA") if pc + 4 < tokens.len() => {
                if should_execute {
                    pc += 1;
                    let var_type = &tokens[pc];
                    pc += 1;
                    let var_name = &tokens[pc];
                    pc += 1;

                    if tokens[pc] != "=" {
                        output.push_str("[ERROR: Syntax]: Expected '=' after variable name\n");
                        break;
                    }
                    pc += 1;

                    let value = match var_type.as_str() {
                        "KIRA" => {
                            let var_value = &tokens[pc];
                            if var_value.starts_with('"') && var_value.ends_with('"') {
                                Value::Str(var_value[1..var_value.len() - 1].to_string())
                            } else {
                                match variables.get(var_value) {
                                    Some(Value::Str(s)) => Value::Str(s.clone()),
                                    _ => {
                                        output.push_str(
                                            "[ERROR: IncompatibleType]: KIRA does not support a nonstring\n"
                                        );
                                        continue;
                                    }
                                }
                            }
                        }
                        "BAULEAN" => {
                            let var_value = &tokens[pc];
                            match var_value.as_str() {
                                "FLUFFY" => Value::Bool(true),
                                "FUZZY" => Value::Bool(false),
                                _ => {
                                    match variables.get(var_value) {
                                        Some(Value::Bool(b)) => Value::Bool(*b),
                                        _ => {
                                            output.push_str(
                                                "[ERROR: IncompatibleType]: BAULEAN requires FLUFFY/FUZZY or a declared BAULEAN-type variable\n"
                                            );
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                        "MOE" => {
                            let var_value = &tokens[pc];

                            if var_value.starts_with('<') && var_value.ends_with('>') {
                                let expr = &var_value[1..var_value.len() - 1];
                                match evaluate_arithmetic(expr, &variables) {
                                    Ok(n) => Value::Num(n),
                                    Err(e) => {
                                        output.push_str(&format!("{}\n", e));
                                        continue;
                                    }
                                }
                            } else {
                                match var_value.parse::<f64>() {
                                    Ok(n) => Value::Num(n),
                                    Err(_) => {
                                        match variables.get(var_value) {
                                            Some(Value::Num(n)) => Value::Num(*n),
                                            _ => {
                                                output.push_str(
                                                    "[ERROR: InvalidValue]: Invalid number/arithmetic expression\n"
                                                );
                                                continue;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        _ => {
                            output.push_str(&format!("Unknown type: {}\n", var_type));
                            continue;
                        }
                    };

                    variables.insert(var_name.to_string(), value);
                }
                pc += 1;
            }

            Some("CO") if pc + 3 < tokens.len() => {
                if should_execute {
                    pc += 1;
                    let var_name = &tokens[pc];
                    pc += 1;

                    if tokens[pc] != "=" {
                        output.push_str("[ERROR: Syntax]: Expected '=' in reassingment\n");
                        break;
                    }
                    pc += 1;

                    let existing_type = match variables.get(var_name) {
                        Some(Value::Str(_)) => "KIRA",
                        Some(Value::Bool(_)) => "BAULEAN",
                        Some(Value::Num(_)) => "MOE",
                        None => {
                            output.push_str(
                                &format!("[ERROR: VanishValue]: Variable could not be found in scope: {}\n", var_name)
                            );
                            continue;
                        }
                    };

                    let value = match existing_type {
                        "KIRA" => {
                            let var_value = &tokens[pc];
                            if var_value.starts_with('"') && var_value.ends_with('"') {
                                Value::Str(var_value[1..var_value.len() - 1].to_string())
                            } else {
                                match variables.get(var_value) {
                                    Some(Value::Str(s)) => Value::Str(s.clone()),
                                    _ => {
                                        output.push_str(
                                            "[ERROR: IncompatibleType]: CO requires matching type (KIRA)\n"
                                        );
                                        continue;
                                    }
                                }
                            }
                        }
                        "BAULEAN" => {
                            let var_value = &tokens[pc];
                            match var_value.as_str() {
                                "FLUFFY" => Value::Bool(true),
                                "FUZZY" => Value::Bool(false),
                                _ => {
                                    match variables.get(var_value) {
                                        Some(Value::Bool(b)) => Value::Bool(*b),
                                        _ => {
                                            output.push_str(
                                                "[ERROR: IncompatibleType]: CO requires matching type (BAULEAN)\n"
                                            );
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                        "MOE" => {
                            let var_value = &tokens[pc];
                            if var_value.starts_with('<') && var_value.ends_with('>') {
                                let expr = &var_value[1..var_value.len() - 1];
                                match evaluate_arithmetic(expr, &variables) {
                                    Ok(n) => Value::Num(n),
                                    Err(e) => {
                                        output.push_str(&format!("{}\n", e));
                                        continue;
                                    }
                                }
                            } else {
                                match var_value.parse::<f64>() {
                                    Ok(n) => Value::Num(n),
                                    Err(_) => {
                                        match variables.get(var_value) {
                                            Some(Value::Num(n)) => Value::Num(*n),
                                            _ => {
                                                output.push_str(
                                                    "[ERROR: IncompatibleType]: CO requires matching type (MOE)\n"
                                                );
                                                continue;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        _ => unreachable!(),
                    };

                    variables.insert(var_name.to_string(), value);
                }
                pc += 1;
            }

            Some("BAU") if pc + 1 < tokens.len() => {
                pc += 1;
                if should_execute {
                    let token = &tokens[pc];
                    if token.starts_with('"') && token.ends_with('"') {
                        output.push_str(&format!("{}\n", &token[1..token.len() - 1]));
                    } else {
                        match variables.get(token) {
                            Some(Value::Str(s)) => output.push_str(&format!("{}\n", s)),
                            Some(Value::Bool(b)) => output.push_str(&format!("{}\n", b)),
                            Some(Value::Num(n)) => output.push_str(&format!("{}\n", n)),
                            None =>
                                output.push_str(
                                    &format!("[ERROR: VanishValue]: Variable couldn't be found: {}\n", token)
                                ),
                        }
                    }
                }
                pc += 1;
            }

            Some("PONDE") if pc + 3 < tokens.len() => {
                if should_execute {
                    pc += 1;
                    let var_name = &tokens[pc];
                    pc += 1;

                    let range = tokens[pc].split("..").collect::<Vec<&str>>();
                    if range.len() != 2 {
                        output.push_str(
                            "[ERROR: Syntax]: Invalid range. Expected 'startInt..endInt'\n"
                        );
                        continue;
                    }

                    let start = match range[0].parse::<f64>() {
                        Ok(n) => n,
                        Err(_) => {
                            output.push_str(
                                "[ERROR: InvalidRange]: Start value must be an integer\n"
                            );
                            continue;
                        }
                    };

                    let end = match range[1].parse::<f64>() {
                        Ok(n) => n,
                        Err(_) => {
                            output.push_str(
                                "[ERROR: InvalidRange]: End value must be an integer\n"
                            );
                            continue;
                        }
                    };

                    pc += 1;

                    if tokens.get(pc) != Some(&"{".to_string()) {
                        output.push_str("[ERROR: Syntax]: Expected '{' to begin the loop\n");
                        continue;
                    }
                    pc += 1;

                    let loop_body_start = pc;
                    let mut loop_body_end = pc;

                    while loop_body_end < tokens.len() {
                        if tokens[loop_body_end] == "}" {
                            break;
                        }
                        loop_body_end += 1;
                    }

                    if loop_body_end == tokens.len() {
                        output.push_str("[ERROR: Syntax]: Could not find closing '}' for loop\n");
                        continue;
                    }

                    for i in start as i64..(end as i64) + 1 {
                        variables.insert(var_name.to_string(), Value::Num(i as f64));
                        let mut inner_pc = loop_body_start;

                        while inner_pc < loop_body_end {
                            match tokens.get(inner_pc).map(String::as_str) {
                                Some("BAU") if inner_pc + 1 < loop_body_end => {
                                    inner_pc += 1;
                                    let token = &tokens[inner_pc];
                                    if token.starts_with('"') && token.ends_with('"') {
                                        output.push_str(
                                            &format!("{}\n", &token[1..token.len() - 1])
                                        );
                                    } else {
                                        match variables.get(token) {
                                            Some(Value::Str(s)) =>
                                                output.push_str(&format!("{}\n", s)),
                                            Some(Value::Bool(b)) =>
                                                output.push_str(&format!("{}\n", b)),
                                            Some(Value::Num(n)) =>
                                                output.push_str(&format!("{}\n", n)),
                                            None =>
                                                output.push_str(
                                                    &format!("[ERROR: VanishValue]: Variable couldn't be found in scope: {}\n", token)
                                                ),
                                        }
                                    }
                                    inner_pc += 1;
                                }
                                Some("WA") if inner_pc + 4 < loop_body_end => {
                                    inner_pc += 1;
                                    let var_type = &tokens[inner_pc];
                                    inner_pc += 1;
                                    let var_name = &tokens[inner_pc];
                                    inner_pc += 1;

                                    if tokens[inner_pc] != "=" {
                                        output.push_str(
                                            "[ERROR: Syntax]: Expected '=' after variable name\n"
                                        );
                                        break;
                                    }
                                    inner_pc += 1;

                                    let value = match var_type.as_str() {
                                        "MOE" => {
                                            let var_value = &tokens[inner_pc];

                                            if
                                            var_value.starts_with('<') &&
                                                var_value.ends_with('>')
                                            {
                                                let expr = &var_value[1..var_value.len() - 1];

                                                let expr = expr.replace("counter", &i.to_string());
                                                match evaluate_arithmetic(&expr, &variables) {
                                                    Ok(n) => Value::Num(n),
                                                    Err(e) => {
                                                        output.push_str(&format!("{}\n", e));
                                                        continue;
                                                    }
                                                }
                                            } else {
                                                match var_value.parse::<f64>() {
                                                    Ok(n) => Value::Num(n),
                                                    Err(_) => {
                                                        match variables.get(var_value) {
                                                            Some(Value::Num(n)) => Value::Num(*n),
                                                            _ => {
                                                                output.push_str(
                                                                    "[ERROR: InvalidValue]: Invalid number/arithmetic expression\n"
                                                                );
                                                                continue;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        "KIRA" => {
                                            let var_value = &tokens[inner_pc];
                                            if
                                            var_value.starts_with('"') &&
                                                var_value.ends_with('"')
                                            {
                                                Value::Str(
                                                    var_value[1..var_value.len() - 1].to_string()
                                                )
                                            } else {
                                                match variables.get(var_value) {
                                                    Some(Value::Str(s)) => Value::Str(s.clone()),
                                                    _ => {
                                                        output.push_str(
                                                            "[ERROR: IncompatibleType]: KIRA does not support a nonstring\n"
                                                        );
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                        "BAULEAN" => {
                                            let var_value = &tokens[inner_pc];
                                            match var_value.as_str() {
                                                "FLUFFY" => Value::Bool(true),
                                                "FUZZY" => Value::Bool(false),
                                                _ => {
                                                    match variables.get(var_value) {
                                                        Some(Value::Bool(b)) => Value::Bool(*b),
                                                        _ => {
                                                            output.push_str(
                                                                "[ERROR: IncompatibleType]: BAULEAN requires FLUFFY/FUZZY or boolean variable\n"
                                                            );
                                                            continue;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        _ => {
                                            output.push_str(
                                                &format!("Unknown type: {}\n", var_type)
                                            );
                                            continue;
                                        }
                                    };

                                    variables.insert(var_name.to_string(), value);
                                    inner_pc += 1;
                                }
                                Some("CO") if inner_pc + 3 < loop_body_end => {
                                    inner_pc += 1;
                                    let var_name = &tokens[inner_pc];
                                    inner_pc += 1;

                                    if tokens[inner_pc] != "=" {
                                        output.push_str(
                                            "[ERROR: Syntax]: Expected '=' in a reassignment\n"
                                        );
                                        break;
                                    }
                                    inner_pc += 1;

                                    let existing_type = match variables.get(var_name) {
                                        Some(Value::Str(_)) => "KIRA",
                                        Some(Value::Bool(_)) => "BAULEAN",
                                        Some(Value::Num(_)) => "MOE",
                                        None => {
                                            output.push_str(
                                                &format!("[ERROR: VanishValue]: Variable couldn't be found in scope: {}\n", var_name)
                                            );
                                            continue;
                                        }
                                    };

                                    let value = match existing_type {
                                        "MOE" => {
                                            let var_value = &tokens[inner_pc];
                                            if
                                            var_value.starts_with('<') &&
                                                var_value.ends_with('>')
                                            {
                                                let expr = &var_value[1..var_value.len() - 1];
                                                let expr = expr.replace("counter", &i.to_string());
                                                match evaluate_arithmetic(&expr, &variables) {
                                                    Ok(n) => Value::Num(n),
                                                    Err(e) => {
                                                        output.push_str(&format!("{}\n", e));
                                                        continue;
                                                    }
                                                }
                                            } else {
                                                match var_value.parse::<f64>() {
                                                    Ok(n) => Value::Num(n),
                                                    Err(_) => {
                                                        match variables.get(var_value) {
                                                            Some(Value::Num(n)) => Value::Num(*n),
                                                            _ => {
                                                                output.push_str(
                                                                    "[ERROR: IncompatibleType]: CO requires matching type (MOE)\n"
                                                                );
                                                                continue;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        _ => {
                                            continue;
                                        }
                                    };

                                    variables.insert(var_name.to_string(), value);
                                    inner_pc += 1;
                                }
                                _ => {
                                    inner_pc += 1;
                                }
                            }
                        }
                    }

                    pc = loop_body_end + 1;
                } else {
                    while pc < tokens.len() && tokens[pc] != "}" {
                        pc += 1;
                    }
                    pc += 1;
                }
            }

            _ => {
                pc += 1;
            }
        }
    }
}
