use crate::utils::Part;
use std::{
    cmp::{max, min},
    str::Chars,
};

#[derive(PartialEq, Eq, Debug)]
enum Expr {
    Literal {
        version: u8,
        value: u64,
    },
    Eq {
        version: u8,
        a: Box<Expr>,
        b: Box<Expr>,
    },
    Lt {
        version: u8,
        a: Box<Expr>,
        b: Box<Expr>,
    },
    Gt {
        version: u8,
        a: Box<Expr>,
        b: Box<Expr>,
    },
    Sum {
        version: u8,
        args: Vec<Expr>,
    },
    Mul {
        version: u8,
        args: Vec<Expr>,
    },
    Min {
        version: u8,
        args: Vec<Expr>,
    },
    Max {
        version: u8,
        args: Vec<Expr>,
    },
}

#[derive(Debug)]
enum ParseExprError {
    UnknownOperation,
    NotEnoughChars,
    BadHexChar,
    BadSubExpressionCount,
    GuardReached,
    BadGuardState,
}

struct ReaderState<'a> {
    curr_str: Chars<'a>,
    curr_byte: u32,
    insert_pos: u8,
    bits_read: u32,
    guards: Vec<u32>,
}

fn hex_to_str(chr: char) -> Result<u32, ParseExprError> {
    match chr {
        '0' => Ok(0),
        '1' => Ok(1),
        '2' => Ok(2),
        '3' => Ok(3),
        '4' => Ok(4),
        '5' => Ok(5),
        '6' => Ok(6),
        '7' => Ok(7),
        '8' => Ok(8),
        '9' => Ok(9),
        'A' => Ok(10),
        'B' => Ok(11),
        'C' => Ok(12),
        'D' => Ok(13),
        'E' => Ok(14),
        'F' => Ok(15),
        _ => Err(ParseExprError::BadHexChar),
    }
}

impl<'a> ReaderState<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            curr_str: input.chars(),
            curr_byte: 0,
            insert_pos: 0,
            bits_read: 0,
            guards: vec![],
        }
    }

    fn read_n_bits(&mut self, nb: u8) -> Result<u32, ParseExprError> {
        while self.insert_pos < nb {
            self.decode_next_byte()?
        }
        let unshifted_mask: u32 = (1 << nb) - 1;
        let mask = unshifted_mask << (32 - nb);
        let result = (self.curr_byte & mask) >> (32 - nb);
        self.consume_bits(nb as u32)?;
        return Ok(result);
    }

    pub fn add_guard(&mut self, size: u32) {
        self.guards.push(size + self.bits_read)
    }

    pub fn remove_guard(&mut self) -> Result<(), ParseExprError> {
        if let Some(value) = self.guards.pop() {
            if self.bits_read < value {
                self.consume_bits(value - self.bits_read)?;
            }
            return Ok(());
        }
        return Err(ParseExprError::BadGuardState);
    }

    pub fn consume_bits(&mut self, nb: u32) -> Result<(), ParseExprError> {
        let mut remaining_to_consume = nb;
        while remaining_to_consume > self.insert_pos as u32 {
            remaining_to_consume -= self.insert_pos as u32;
            self.consume_bits(self.insert_pos as u32)?;
            self.decode_next_byte()?
        }
        self.check_guard()?;
        self.curr_byte <<= remaining_to_consume as u8;
        self.insert_pos -= remaining_to_consume as u8;
        self.bits_read += remaining_to_consume;
        Result::Ok(())
    }

    fn decode_next_byte(&mut self) -> Result<(), ParseExprError> {
        self.check_guard()?;
        let char = self.curr_str.next().ok_or(ParseExprError::NotEnoughChars)?;
        let value = hex_to_str(char)?;
        self.curr_byte |= value << (32 - self.insert_pos - 4);
        self.insert_pos += 4;
        Result::Ok(())
    }

    fn check_guard(&self) -> Result<(), ParseExprError> {
        if self.guards.iter().any(|max| (max) <= &self.bits_read) {
            return Err(ParseExprError::GuardReached);
        }
        return Ok(());
    }
}

fn parse_literal(reader_state: &mut ReaderState) -> Result<u64, ParseExprError> {
    let mut result: u64 = 0;
    loop {
        let next_value = reader_state.read_n_bits(5)?;
        result <<= 4;
        result |= (next_value & (0b1111)) as u64;
        if (next_value & 0b10000) == 0 {
            break;
        }
    }
    return Ok(result);
}

fn parse_variable_with_length(reader_state: &mut ReaderState) -> Result<Vec<Expr>, ParseExprError> {
    let mut res: Vec<Expr> = vec![];
    let length = reader_state.read_n_bits(15)?;
    reader_state.add_guard(length);
    loop {
        let next_expr = parse_expr(reader_state);
        match next_expr {
            Ok(expr) => res.push(expr),
            Err(ParseExprError::GuardReached) => break,
            Err(error) => return Err(error),
        }
    }
    reader_state.remove_guard()?;
    return Ok(res);
}

fn parse_variable_with_number_of_sub_expr(
    reader_state: &mut ReaderState,
) -> Result<Vec<Expr>, ParseExprError> {
    let mut res: Vec<Expr> = vec![];
    let length = reader_state.read_n_bits(11)?;
    while res.len() < (length as usize) {
        let next_expr = parse_expr(reader_state);
        match next_expr {
            Ok(expr) => res.push(expr),
            Err(error) => return Err(error),
        }
    }
    return Ok(res);
}

fn parse_variable(reader_state: &mut ReaderState) -> Result<Vec<Expr>, ParseExprError> {
    let length_type = reader_state.read_n_bits(1)?;
    if length_type == 0 {
        return parse_variable_with_length(reader_state);
    } else {
        return parse_variable_with_number_of_sub_expr(reader_state);
    }
}

fn parse_tuple(reader_state: &mut ReaderState) -> Result<(Expr, Expr), ParseExprError> {
    let mut result = parse_variable(reader_state)?;
    let second = result.pop();
    let first = result.pop();
    if result.len() > 0 {
        return Err(ParseExprError::BadSubExpressionCount);
    }
    return first
        .zip(second)
        .map(|items| Ok(items))
        .unwrap_or(Err(ParseExprError::BadSubExpressionCount));
}

fn parse_expr(reader_state: &mut ReaderState) -> Result<Expr, ParseExprError> {
    let version = reader_state.read_n_bits(3)? as u8;
    let type_id = reader_state.read_n_bits(3)?;
    return match type_id {
        0 => Ok(Expr::Sum {
            version,
            args: parse_variable(reader_state)?,
        }),
        1 => Ok(Expr::Mul {
            version,
            args: parse_variable(reader_state)?,
        }),
        2 => Ok(Expr::Min {
            version,
            args: parse_variable(reader_state)?,
        }),
        3 => Ok(Expr::Max {
            version,
            args: parse_variable(reader_state)?,
        }),
        4 => Ok(Expr::Literal {
            version: version as u8,
            value: parse_literal(reader_state)?,
        }),
        5 => {
            let (a, b) = parse_tuple(reader_state)?;
            Ok(Expr::Gt {
                version,
                a: Box::new(a),
                b: Box::new(b),
            })
        }
        6 => {
            let (a, b) = parse_tuple(reader_state)?;
            Ok(Expr::Lt {
                version,
                a: Box::new(a),
                b: Box::new(b),
            })
        }
        7 => {
            let (a, b) = parse_tuple(reader_state)?;
            Ok(Expr::Eq {
                version,
                a: Box::new(a),
                b: Box::new(b),
            })
        }
        _ => Err(ParseExprError::UnknownOperation),
    };
}

fn parse_line(line: &str) -> Result<Expr, ParseExprError> {
    return Ok(parse_expr(&mut ReaderState::new(line))?);
}

fn parse(lines: &Vec<String>) -> Result<Expr, ParseExprError> {
    return lines
        .get(0)
        .map(|line| parse_line(line))
        .unwrap_or(Result::Err(ParseExprError::UnknownOperation));
}

fn calc_version(expr: &Expr) -> u32 {
    return match expr {
        Expr::Literal { version, .. } => *version as u32,
        Expr::Eq { version, a, b } | Expr::Gt { version, a, b } | Expr::Lt { version, a, b } => {
            *version as u32 + calc_version(a.as_ref()) + calc_version(b.as_ref())
        }
        Expr::Sum { version, args }
        | Expr::Mul { version, args }
        | Expr::Min { version, args }
        | Expr::Max { version, args } => args
            .iter()
            .fold(*version as u32, |sum, arg| sum + calc_version(arg)),
    };
}

fn evaluate(expr: &Expr) -> u64 {
    return match expr {
        Expr::Literal { value, .. } => *value,
        Expr::Eq { a, b, .. } => {
            if evaluate(a.as_ref()) == evaluate(b.as_ref()) {
                1
            } else {
                0
            }
        }
        Expr::Gt { a, b, .. } => {
            if evaluate(a.as_ref()) > evaluate(b.as_ref()) {
                1
            } else {
                0
            }
        }
        Expr::Lt { a, b, .. } => {
            if evaluate(a.as_ref()) < evaluate(b.as_ref()) {
                1
            } else {
                0
            }
        }
        Expr::Sum { args, .. } => args.iter().fold(0, |sum, val| sum + evaluate(val)),
        Expr::Mul { args, .. } => args.iter().fold(1, |mul, val| mul * evaluate(val)),
        Expr::Min { args, .. } => args
            .iter()
            .fold(std::u64::MAX, |min_val, val| min(min_val, evaluate(val))),
        Expr::Max { args, .. } => args
            .iter()
            .fold(0 as u64, |max_val, val| max(max_val, evaluate(val))),
    };
}

pub fn puzzle(part: &Part, lines: &Vec<String>) {
    let expr_res = parse(lines);
    let expr = expr_res.unwrap();
    match part {
        Part::Part1 => {
            let result = calc_version(&expr);
            println!("Result Version {}", result)
        }
        Part::Part2 => {
            let result = evaluate(&expr);
            println!("Result Eval {}", result);
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parse_literal() {
        let parse_res = parse_line("D2FE28").unwrap();
        assert_eq!(
            parse_res,
            Expr::Literal {
                version: 6,
                value: 2021
            }
        );
    }

    #[test]
    fn test_parse_max_nb_sub_packets() {
        let parse_res = parse_line("EE00D40C823060").unwrap();
        assert_eq!(
            parse_res,
            Expr::Max {
                version: 7,
                args: vec![
                    Expr::Literal {
                        version: 2,
                        value: 1
                    },
                    Expr::Literal {
                        version: 4,
                        value: 2
                    },
                    Expr::Literal {
                        version: 1,
                        value: 3
                    }
                ]
            }
        );
    }

    #[test]
    fn test_parse_lt_variable_length() {
        let parse_res = parse_line("38006F45291200").unwrap();
        assert_eq!(
            parse_res,
            Expr::Lt {
                version: 1,
                a: Box::new(Expr::Literal {
                    version: 6,
                    value: 10
                }),
                b: Box::new(Expr::Literal {
                    version: 2,
                    value: 20
                })
            }
        );
    }

    #[test]
    fn test_sum_version_packets() {
        assert_eq!(calc_version(&parse_line("8A004A801A8002F478").unwrap()), 16);
        assert_eq!(
            calc_version(&parse_line("620080001611562C8802118E34").unwrap()),
            12
        );
        assert_eq!(
            calc_version(&parse_line("C0015000016115A2E0802F182340").unwrap()),
            23
        );
        assert_eq!(
            calc_version(&parse_line("A0016C880162017C3686B18A3D4780").unwrap()),
            31
        )
    }

    #[test]
    fn test_calc_packets() {
        assert_eq!(evaluate(&parse_line("C200B40A82").unwrap()), 3);
        assert_eq!(evaluate(&parse_line("04005AC33890").unwrap()), 54);
        assert_eq!(evaluate(&parse_line("880086C3E88112").unwrap()), 7);
        assert_eq!(evaluate(&parse_line("CE00C43D881120").unwrap()), 9);
        assert_eq!(evaluate(&parse_line("D8005AC2A8F0").unwrap()), 1);
        assert_eq!(evaluate(&parse_line("F600BC2D8F").unwrap()), 0);
        assert_eq!(evaluate(&parse_line("9C005AC2F8F0").unwrap()), 0);
        assert_eq!(
            evaluate(&parse_line("9C0141080250320F1802104A08").unwrap()),
            1
        );
    }
}
