use std::fmt::{Display, Formatter};
use int_enum::IntEnum;
use enum_iterator::{all, Sequence};


#[repr(usize)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Sequence, IntEnum)]
pub enum ResistorColor {
 Black = 0,
 Brown = 1,
 Red = 2,
 Orange = 3,
 Yellow = 4,
 Green = 5,
 Blue = 6,
 Violet = 7,
 Grey = 8,
 White = 9,
}
impl Display for ResistorColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
pub fn color_to_value(_color: ResistorColor) -> usize {
    _color.int_value()
}

pub fn value_to_color_string(value: usize) -> String {
    return ResistorColor::from_int(value).map_or("value out of range".to_string(), |v| v.to_string());
}

pub fn colors() -> Vec<ResistorColor> {
    all::<ResistorColor>().collect()
}
