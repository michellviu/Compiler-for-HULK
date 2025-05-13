#[derive(Debug)]
pub enum LexerError {
    UnexpectedCharacter(char, usize),
}

#[derive(Debug)]
pub enum ParserError {
    UnexpectedToken(String, usize),
    ExpectedIdentifier(String, usize),
}
