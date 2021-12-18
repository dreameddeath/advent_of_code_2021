use crate::utils::Part;
use std::str::Chars;

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

enum ParseExprError {
    UnknownOperation,
    EmptyInput,
    NotEnoughChars,
    BadHexChar,
    BadSubExpressionCount,
    GuardReached,
}

struct ReaderState<'a> {
    curr_str: Chars<'a>,
    curr_byte: u16,
    insert_pos: u8,
    bits_read: u32,
    guards: Vec<u32>,
}

fn hex_to_str(chr: char) -> Result<u16, ParseExprError> {
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

    fn read_n_bits(&mut self, nb: u8) -> Result<u16, ParseExprError> {
        while self.insert_pos < nb {
            self.decode_next_byte()?
        }
        let unshifted_mask: u16 = (1 << nb) - 1;
        let mask = unshifted_mask << (16 - nb);
        let result = ((self.curr_byte & mask) >> (16 - nb));
        self.consume_bits(nb as u32);
        return Ok(result);
    }

    pub fn add_guard(&mut self, size: u16) {
        self.guards.push(size as u32 + self.bits_read)
    }

    pub fn remove_guard(&mut self) {
        if let Some(value) = self.guards.pop() {
            if (self.bits_read < value) {
                self.consume_bits(value - self.bits_read);
            }
        }
    }

    pub fn consume_bits(&mut self, nb: u32) -> Result<(), ParseExprError> {
        let mut remaining_to_consume = nb;
        while remaining_to_consume > self.insert_pos as u32 {
            remaining_to_consume -= self.insert_pos as u32;
            self.consume_bits(self.insert_pos as u32)?;
            self.decode_next_byte()?
        }
        self.curr_byte <<= remaining_to_consume as u8;
        self.insert_pos -= remaining_to_consume as u8;
        Result::Ok(())
    }

    fn decode_next_byte(&mut self) -> Result<(), ParseExprError> {
        if self.guards.iter().any(|max| (max) < &self.bits_read) {
            return Err(ParseExprError::GuardReached);
        }
        let char = self.curr_str.next().ok_or(ParseExprError::NotEnoughChars)?;
        let value = hex_to_str(char)?;
        self.curr_byte |= value << (16 - self.insert_pos - 4);
        self.insert_pos += 4;
        self.bits_read += 1;
        Result::Ok(())
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
            Err(ParseExprError::NotEnoughChars) => break,
            Err(error) => return Err(error),
        }
    }
    reader_state.remove_guard();
    return Ok(res);
}

fn parse_variable_with_number_of_sub_expr(
    reader_state: &mut ReaderState,
) -> Result<Vec<Expr>, ParseExprError> {
    let mut res: Vec<Expr> = vec![];
    let length = reader_state.read_n_bits(11)?;
    while (res.len() < (length as usize)) {
        let next_expr = parse_expr(reader_state);
        match next_expr {
            Ok(expr) => res.push(expr),
            Err(ParseExprError::NotEnoughChars) => break,
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
            Ok(Expr::Gt { version, a:Box::new(a), b:Box::new(b) })
        }
        6 => {
            let (a, b) = parse_tuple(reader_state)?;
            Ok(Expr::Lt { version, a:Box::new(a), b:Box::new(b) })
        }
        7=> {
            let (a, b) = parse_tuple(reader_state)?;
            Ok(Expr::Eq{ version, a:Box::new(a), b:Box::new(b) })
        }
        _ => Err(ParseExprError::UnknownOperation),
    };
}

fn parse_line(line: &String) -> Result<Expr, ParseExprError> {
    return Ok(parse_expr(&mut ReaderState::new(line))?);
}

fn parse(lines: &Vec<String>) -> Result<Expr, ParseExprError> {
    return lines
        .get(0)
        .map(|line| parse_line(line))
        .unwrap_or(Result::Err(ParseExprError::UnknownOperation));
}

fn sum_version_content(expr:Expr)->u32{
    return match expr {
        Expr::Literal { version, .. } => version as u32,
        Expr::Eq
    }
}

fn sum_version(expr:Expr)->u32{
    
}

pub fn puzzle(part: &Part, lines: &Vec<String>) {
    let expr_res = parse(lines);
    let expr =expr_res.unwrap();
    match part {
        Part::Part1 => {

            println!("Result {}", "nothing")
        }
        Part::Part2 => {
            println!("Result {}", "nothing");
        }
    }
}
