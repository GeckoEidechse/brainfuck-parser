use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, value},
    multi::many0,
    sequence::tuple,
    IResult,
};

/// All instructions
#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    RightShift,
    LeftShift,
    Increment,
    Decrement,
    Output,
    Input,
    Loop(Vec<Instruction>),
}

// The main structure of a brainfuck program
pub struct BrainfuckProgram(Vec<Instruction>);

/// Parse entire brainfuck code
/// This is the main point of entry
pub fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    // Fail on remaining output, i.e. unexpected tokens
    match many0(parse_instruction)(input) {
        Ok(("", instructions)) => Ok(("", instructions)),
        Ok((rest, _)) => Err(nom::Err::Failure(nom::error::Error::new(
            rest,
            nom::error::ErrorKind::Eof,
        ))),
        Err(e) => Err(e),
    }
}

/// Parse a basic instruction or loop
fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        value(Instruction::RightShift, tag(">")),
        value(Instruction::LeftShift, tag("<")),
        value(Instruction::Increment, tag("+")),
        value(Instruction::Decrement, tag("-")),
        value(Instruction::Output, tag(".")),
        value(Instruction::Input, tag(",")),
        map(parse_loop, Instruction::Loop),
    ))(input)
}

/// Parse a loop, i.e. `[ ]`
fn parse_loop(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, (_, instructions, _)) =
        tuple((tag("["), many0(parse_instruction), tag("]")))(input)?;
    Ok((input, instructions))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_instruction() {
        // Check parsing individual instructions
        assert_eq!(parse_instruction(">"), Ok(("", Instruction::RightShift)));
        assert_eq!(parse_instruction("<"), Ok(("", Instruction::LeftShift)));
        assert_eq!(parse_instruction("+"), Ok(("", Instruction::Increment)));
        assert_eq!(parse_instruction("-"), Ok(("", Instruction::Decrement)));
        assert_eq!(parse_instruction("."), Ok(("", Instruction::Output)));
        assert_eq!(parse_instruction(","), Ok(("", Instruction::Input)));
    }

    #[test]
    fn test_parse_basic_loop_brainfuck() {
        // Check parsing basic single loop
        parse_loop("[->+<]").unwrap();
    }

    #[test]
    fn test_parse_basic_brainfuck() {
        // super simple loop
        parse("+>>+[->+<]-").unwrap();
    }

    #[test]
    fn test_parse_brainfuck_hello_world() {
        // taken from https://en.wikipedia.org/wiki/Brainfuck#Hello_World!
        parse("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.").unwrap();
    }

    #[test]
    fn test_parse_brainfuck_rot13() {
        // taken from https://en.wikipedia.org/wiki/Brainfuck#ROT13
        parse("-,+[-[>>++++[>++++++++<-]<+<-[>+>+>-[>>>]<[[>+<-]>>+>]<<<<<-]]>>>[-]+>--[-[<->+++[-]]]<[++++++++++++<[>-[>+>>]>[+[<+>-]>+>>]<<<<<-]>>[<+>-]>[-[-<<[-]>>]<<[<<->>-]>>]<<[<<+>>-]]<[-]<.[-]<-,+]").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_parse_wrong_instruction() {
        // Check if wrong instruction correctly gets detected as such
        parse_instruction("s").unwrap();
    }
}
