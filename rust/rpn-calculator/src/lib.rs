#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}


pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack = vec![];
    for input in inputs {
        match input {
            CalculatorInput::Value(value) => {
                stack.push(CalculatorInput::Value(*value));
            },
            CalculatorInput::Add => {
                if let Some((left, right)) = pop_values(&mut stack) {
                    stack.push(CalculatorInput::Value(left + right));
                } else {
                    return None;
                }
            },
            CalculatorInput::Subtract => {
                if let Some((left, right)) = pop_values(&mut stack) {
                    stack.push(CalculatorInput::Value(left - right));
                } else {
                    return None;
                }
            },
            CalculatorInput::Multiply => {
                if let Some((left, right)) = pop_values(&mut stack) {
                    stack.push(CalculatorInput::Value(left * right));
                } else {
                    return None;
                }
            },
            CalculatorInput::Divide => {
                if let Some((left, right)) = pop_values(&mut stack) {
                    stack.push(CalculatorInput::Value(left / right));
                } else {
                    return None;
                }
            }
        }
    }
    if stack.len() != 1 {
        return None;
    }
    return if let Some(CalculatorInput::Value(value)) = stack.pop() {
        Some(value)
    } else {
        None
    }
}

fn pop_values(mut vec: &mut Vec<CalculatorInput>) -> Option<(i32, i32)> {
    let right = pop_value(vec);
    let left = pop_value(vec);
    if left.is_none() || right.is_none() {
        return None
    }
    return Some((left.unwrap(), right.unwrap()));
}
fn pop_value(mut vec: &mut Vec<CalculatorInput>) -> Option<i32> {
    let pop = vec.pop();
    return if let Some(CalculatorInput::Value(value)) = pop {
        Some(value)
    } else {
        None
    }
}