#[derive(Debug, Clone)]
pub enum Token {
    PLUS,
    MINUS,
    ASTER,
    SLASH,
    LPAREN,
    RPAREN,
    DIGIT(Digit),
}

#[derive(Debug, Clone)]
pub enum Digit {
    ZERO,
    ONE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
}

impl Digit {
    pub fn new(digit: u8) -> Option<Self> {
        match digit {
            0 => Some(Digit::ZERO),
            1 => Some(Digit::ONE),
            2 => Some(Digit::TWO),
            3 => Some(Digit::THREE),
            4 => Some(Digit::FOUR),
            5 => Some(Digit::FIVE),
            6 => Some(Digit::SIX),
            7 => Some(Digit::SEVEN),
            8 => Some(Digit::EIGHT),
            9 => Some(Digit::NINE),
            _ => None,
        }
    }

    pub fn num(&self) -> u8 {
        match *self {
            Digit::ZERO => 0,
            Digit::ONE => 1,
            Digit::TWO => 2,
            Digit::THREE => 3,
            Digit::FOUR => 4,
            Digit::FIVE => 5,
            Digit::SIX => 6,
            Digit::SEVEN => 7,
            Digit::EIGHT => 8,
            Digit::NINE => 9,
        }
    }
}
