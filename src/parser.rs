use std::{iter::Peekable};

use crate::lexer::LexerToken;

pub fn parser(input: Vec<LexerToken>) -> Result<Box<SyntaxTree>,String> {
    let mut stream = input.into_iter().filter(|token| !matches!(token, LexerToken::Whitespace) && !matches!(token, LexerToken::Comment(_))).peekable();
    
    let expression = parse_term_expression(&mut stream)?;
    let end_token = expect_token_type(&mut stream, LexerToken::EndOfFile)?;

    Ok(Box::from(SyntaxTree {
        inner_expression: expression,
        end_of_file_token: end_token,
    }))
}

fn parse_expression(stream: &mut Peekable<impl Iterator<Item = LexerToken>>) -> Result<Box<Expression>,String> {
    parse_term_expression(stream)
}

fn parse_term_expression(stream: &mut Peekable<impl Iterator<Item = LexerToken>>) -> Result<Box<Expression>,String> {
    let mut left = parse_factor_expression(stream)?;
    while match stream.peek() {
        Some(LexerToken::Plus) => true,
        Some(LexerToken::Dash) => true,
        _ => false
    } {
        let operator_token = stream.next().unwrap();
        let right = parse_factor_expression( stream)?;

        left = Box::new(Expression::BinaryExpression(left, operator_token.clone(), right));
    }

    Ok(left)
}
fn parse_factor_expression(stream: &mut Peekable<impl Iterator<Item = LexerToken>>) -> Result<Box<Expression>,String> {
    let mut left = parse_primary_expression(stream)?;
    while match stream.peek() {
        Some(LexerToken::Star) => true,
        Some(LexerToken::Slash) => true,
        Some(_) => false,
        None => panic!("Unable to read")
    } {
        let operator_token = stream.next().unwrap();
        let right = parse_primary_expression( stream)?;

        left = Box::new(Expression::BinaryExpression(left, operator_token.clone(), right));
    }

    Ok(left)
}

fn parse_primary_expression(stream: &mut Peekable<impl Iterator<Item = LexerToken>>) -> Result<Box<Expression>, String> {
    if let LexerToken::OpenParenthesis = stream.peek().expect("Unable to read") {
        let left = expect_token_type(stream, LexerToken::OpenParenthesis)?;
        let expression = parse_expression(stream)?;
        let right = expect_token_type(stream, LexerToken::CloseParenthesis)?;

        Ok(Box::from(Expression::ParenthesesExpression(left,expression,right)))
    }
    else {
        let token = expect_token_type(stream, LexerToken::LiteralInt(0))?;
        Ok(Box::from(Expression::LiteralExpression(token)))
    }
}

fn expect_token_type(stream: &mut impl Iterator<Item = LexerToken>, token_type: LexerToken) -> Result<LexerToken, String> {
    match stream.next() {
        Some(token) if std::mem::discriminant(&token) == std::mem::discriminant(&token_type) => Ok(token),
        Some(token) => Err(format!("Unexpected token <{:?}>, expected <{:?}>", token, token_type)),
        None => panic!("Unable to read from data")
    }
}

pub struct SyntaxTree {
    pub inner_expression: Box<Expression>,
    pub end_of_file_token: LexerToken
}
impl PrettyPrintable for SyntaxTree {
    fn stringify(&self) -> String {
        format!("SyntaxTree")
    }

    fn get_children(&self) -> Vec<&dyn PrettyPrintable> {
        vec![&*self.inner_expression, &self.end_of_file_token]
    }
}

pub enum Expression {
    LiteralExpression(LexerToken),
    BinaryExpression(Box<Expression>, LexerToken, Box<Expression>),
    UnaryExpression(LexerToken, Box<Expression>),
    ParenthesesExpression(LexerToken, Box<Expression>, LexerToken)
}
impl Expression {

}

impl std::fmt::Debug for dyn PrettyPrintable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        let mut output = String::new();

        pretty_print(self, &mut output, "", true);
        f.write_str(&output.as_str())?;

        Ok(())
    }
}
fn pretty_print(input: &dyn PrettyPrintable, output_string: &mut String, indent: &str, is_last_child: bool) {
    output_string.push_str(if is_last_child {format!("{}{}",indent,"└──")} else {format!("{}{}",indent,"├──")}.as_str());
    output_string.push_str(input.stringify().as_str());
    output_string.push('\n');

    let children = input.get_children();
    let length_of_children = children.len();

    for (i, child) in children.into_iter().enumerate() {
        let is_last = i == length_of_children-1;

        let indent = if is_last_child {format!("{}{}",indent,"   ")} else {format!("{}{}",indent,"│  ")};

        pretty_print(child, output_string, indent.as_str(), is_last);
    }
}

pub trait PrettyPrintable {
    fn stringify(&self) -> String;
    fn get_children(&self) -> Vec<&dyn PrettyPrintable>;
}
impl PrettyPrintable for Expression {
    fn stringify(&self) -> String {
        String::from(match self {
            Expression::LiteralExpression(_) => {
                format!("LiteralExpression")
            }
            Expression::BinaryExpression(_, _, _) => {
                format!("BinaryExpression")
            },
            Expression::UnaryExpression(_, _) => {
                format!("UnaryExpression")
            },
            Expression::ParenthesesExpression(_, _, _) => {
                format!("ParenthesesExpression")
            }
        })
    }

    fn get_children(&self) -> Vec<&dyn PrettyPrintable> {
        match self {
            Expression::BinaryExpression(a, b, c) => vec![&**a, b, &**c],
            Expression::UnaryExpression(_, a) => vec![&**a],
            Expression::LiteralExpression(a) => vec![a],
            Expression::ParenthesesExpression(a,b,c)=>vec![a, &**b, c]
        }
    }

}

impl PrettyPrintable for LexerToken {
    fn stringify(&self) -> String {
        format!("{:?}",self)
    }

    fn get_children(&self) -> Vec<&dyn PrettyPrintable> {
        vec![]
    }
}