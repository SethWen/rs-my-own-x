#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Plus,
    Minus,
    Mul,
    Div,

    Power,

    LParen,
    RParen,

    Eq,

    Num(String),

    Illegal,
    Eof,
}

impl From<Token> for String {
    fn from(value: Token) -> Self {
        match value {
            Token::Plus => "+".to_owned(),
            Token::Minus => "-".to_owned(),
            Token::Mul => "*".to_owned(),
            Token::Div => "/".to_owned(),

            Token::Power => "^".to_owned(),

            Token::LParen => "(".to_owned(),
            Token::RParen => ")".to_owned(),

            Token::Eq => "=".to_owned(),

            // FIXME: 2024/05/29 16:56:20
            Token::Num(literal) => literal,

            // FIXME: 2024/05/29 16:56:29
            Token::Illegal => "Illegal".to_owned(),
            Token::Eof => "Eof".to_owned(),
        }
    }
}

impl Token {
    pub fn could_be_prefix_expr(&self) -> bool {
        matches!(self, Token::Minus | Token::LParen | Token::Num(_))
    }

    pub fn could_be_infix_expr(&self) -> bool {
        matches!(self, Token::Plus | Token::Minus | Token::Mul | Token::Div | Token::Power)
    }

    pub fn prefix_op(&self, literal: String) -> f64 {
        match self {
            Token::Minus => 0f64 - literal.parse::<f64>().unwrap(),
            Token::LParen => todo!(),
            Token::Num(_) => todo!(),
            _ => todo!(),
        }
    }

    pub fn infix_op(&self, left: String, right: String) -> f64 {
        match self {
            Token::Plus => left.parse::<f64>().unwrap() + right.parse::<f64>().unwrap(),
            Token::Minus => left.parse::<f64>().unwrap() - right.parse::<f64>().unwrap(),
            Token::Mul => left.parse::<f64>().unwrap() * right.parse::<f64>().unwrap(),
            Token::Div => left.parse::<f64>().unwrap() / right.parse::<f64>().unwrap(),
            Token::Power => left
                .parse::<f64>()
                .unwrap()
                .powf(right.parse::<f64>().unwrap()),
            _ => todo!(),
        }
    }
}

#[test]
fn test_basic() {
    assert!(Token::Plus == Token::Plus.clone());
    assert!(Token::Minus == Token::Minus.clone());
    assert!(Token::Mul != Token::Div.clone());

    assert_eq!(String::from(Token::Plus), "+".to_owned());
    assert_eq!(String::from(Token::Minus), "-".to_owned());
    assert_eq!(String::from(Token::Mul), "*".to_owned());
    assert_eq!(String::from(Token::Div), "/".to_owned());

    assert_eq!(String::from(Token::Power), "^".to_owned());

    assert_eq!(String::from(Token::LParen), "(".to_owned());
    assert_eq!(String::from(Token::RParen), ")".to_owned());

    assert_eq!(String::from(Token::Eq), "=".to_owned());

    assert_eq!(String::from(Token::Num("123".to_owned())), "123".to_owned());

    assert_eq!(String::from(Token::Illegal), "Illegal".to_owned());
    assert_eq!(String::from(Token::Eof), "Eof".to_owned());
}
