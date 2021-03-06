use std::fmt;

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Square
{
    Black,
    White,
    Empty,
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Square::Black => "Black",
            Square::White => "White",
            Square::Empty => "<undefined>",
        })
    }
}

impl Square {
    pub fn opposite(&self) -> Square
    {
        match *self {
            Square::Black => Square::White,
            Square::White => Square::Black,
            Square::Empty => Square::Empty
        }
    }

    pub fn to_char(&self) -> char
    {
        match *self {
            Square::Black => 'B',
            Square::White => 'W',
            Square::Empty => '-',
        }
    }

    pub fn to_str(&self) -> &'static str
    {
        match *self {
            Square::Black => "B",
            Square::White => "W",
            Square::Empty => "-",
        }
    }
}

