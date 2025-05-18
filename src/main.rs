use std::io::stdin;
use std::process::exit;

#[derive(Debug, Clone)]
enum ExprClass {
    Operand,
    Plus,
    Minus,
    Mul,
    Div,
    LParen,
    RParen,
}

#[derive(Debug, Clone)]
struct ExprToken {
    expr_class: ExprClass,
    expr_value: f64,
}

fn token_is_digit(token: char) -> bool {
    token >= '0' && token <= '9'
}

fn expr_get_tokens(expr: String) -> Vec<ExprToken> {
    let mut tokens = Vec::<ExprToken>::new();
    let mut token_value = String::new();
    let expr_len: usize = expr.len();
    let mut i = 0;
	let mut ch_tok: char = '_';
    
	while i < expr_len {
		let mut digit_found = false;
        while i < expr_len {
			ch_tok = expr.chars().nth(i).unwrap();
			if token_is_digit(ch_tok) {
				digit_found = true;
				token_value.push(ch_tok);
				i += 1;
			} else { break; }
        }
		if digit_found {
			tokens.push(ExprToken {
				expr_class: ExprClass::Operand,
				expr_value: token_value.parse::<f64>().unwrap(),
			});
			token_value = "".to_string();
		}


        match ch_tok {
            '+' => {
                tokens.push(ExprToken {
                    expr_class: ExprClass::Plus,
                    expr_value: 0.0,
                });
            }
            '-' => {
                tokens.push(ExprToken {
                    expr_class: ExprClass::Minus,
                    expr_value: 0.0,
                });
            }
            '*' => {
                tokens.push(ExprToken {
                    expr_class: ExprClass::Mul,
                    expr_value: 0.0,
                });
            }
            '/' => {
                tokens.push(ExprToken {
                    expr_class: ExprClass::Div,
                    expr_value: 0.0,
                });
            }
            '(' => {
                tokens.push(ExprToken {
                    expr_class: ExprClass::LParen,
                    expr_value: 0.0,
                });
            }
            ')' => {
                tokens.push(ExprToken {
                    expr_class: ExprClass::RParen,
                    expr_value: 0.0,
                });
            }
            '\n' => {}
            '\t' => {}
            '\r' => {}
            ' ' => {}
            err => {
                eprintln!("Unexpected character {}", err);
                exit(2);
            }
        }
        i += 1;
    }

    tokens
}

fn eval_expr(exprs: Vec::<ExprToken>) -> f64 {
	match exprs[1].expr_class {
		ExprClass::Plus => exprs[0].expr_value + exprs[2].expr_value,
		ExprClass::Minus => exprs[0].expr_value - exprs[2].expr_value,
		ExprClass::Mul => exprs[0].expr_value * exprs[2].expr_value,
		ExprClass::Div => {
			if exprs[2].expr_value == 0.0 {
				println!("WARN: denominator turned 0 here");
				println!("WARN: {} / {}", exprs[0].expr_value, exprs[2].expr_value);
			}
			exprs[0].expr_value / exprs[2].expr_value
		}
		_ => {
			eprintln!("Unexpected class: {:?}", exprs[1].expr_class);
			exit(3);
		}
	}
}

fn eval_expr_tokens(mut tokens: Vec::<ExprToken>) -> f64 {
	let mut result: f64 = 0.0;

	while tokens.len() != 1 {
		result = eval_expr(tokens[0..3].to_vec());
		tokens.remove(0);
		tokens.remove(0);
		tokens.remove(0);
		tokens.insert(0, ExprToken{
			expr_class: ExprClass::Operand,
			expr_value: result,
		});
	}
	result
}

fn bc_lite() {
    println!("bc-lite v1.0");
    let mut expr = String::new();
    match stdin().read_line(&mut expr) {
        Ok(_) => {
			// Remove new lines
			expr = String::from(expr.trim());
		}
        Err(error) => {
            println!("could not read from stdin, err: {}", error);
            exit(1);
        }
    }
    let tokens: Vec<ExprToken> = expr_get_tokens(expr.clone() + "\n");
   	let result = eval_expr_tokens(tokens);
	println!("{} = {}", expr, result);
}

fn main() {
    bc_lite();
}
