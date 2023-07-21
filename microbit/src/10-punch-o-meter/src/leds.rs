type Screen = [[u8; 5]; 5];

#[derive(Debug)]
pub enum LedError {
    InvalidNumber
}

const ZERO: Screen = [
    [0, 0, 1, 0, 0],
    [0, 1, 0, 1, 0],   
    [0, 1, 0, 1, 0],
    [0, 1, 0, 1, 0],
    [0, 0, 1, 0, 0],   
];

const ONE: Screen = [
    [0, 1, 1, 0, 0],
    [0, 0, 1, 0, 0],   
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 1, 1, 1, 0],   
];

const TWO: Screen = [
    [0, 1, 1, 1, 0],
    [0, 0, 0, 1, 0],   
    [0, 1, 1, 1, 0],
    [0, 1, 0, 0, 0],
    [0, 1, 1, 1, 0],   
];


const THREE: Screen = [
    [0, 1, 1, 1, 0],
    [0, 0, 0, 1, 0],   
    [0, 1, 1, 1, 0],
    [0, 0, 0, 1, 0],
    [0, 1, 1, 1, 0],   
];

const FOUR: Screen = [
    [0, 1, 0, 1, 0],
    [0, 1, 0, 1, 0],   
    [0, 1, 1, 1, 0],
    [0, 0, 0, 1, 0],
    [0, 0, 0, 1, 0],   
];

const FIVE: Screen = [
    [0, 1, 1, 1, 0],
    [0, 1, 0, 0, 0],   
    [0, 1, 1, 1, 0],
    [0, 0, 0, 1, 0],
    [0, 1, 1, 1, 0],   
];

const SIX: Screen = [
    [0, 1, 1, 1, 0],
    [0, 1, 0, 0, 0],   
    [0, 1, 1, 1, 0],
    [0, 1, 0, 1, 0],
    [0, 1, 1, 1, 0],   
];

const SEVEN: Screen = [
    [0, 1, 1, 1, 0],
    [0, 0, 0, 1, 0],   
    [0, 0, 0, 1, 0],
    [0, 0, 0, 1, 0],
    [0, 0, 0, 1, 0],   
];

const EIGHT: Screen = [
    [0, 1, 1, 1, 0],
    [0, 1, 0, 1, 0],   
    [0, 1, 1, 1, 0],
    [0, 1, 0, 1, 0],
    [0, 1, 1, 1, 0],   
];

const NINE: Screen = [
    [0, 1, 1, 1, 0],
    [0, 1, 0, 1, 0],   
    [0, 1, 1, 1, 0],
    [0, 0, 0, 1, 0],
    [0, 0, 0, 1, 0],   
];

pub fn num_to_screen(num: i32) -> Result<Screen, LedError> {
    match num {
        0 => Ok(ZERO),
        1 => Ok(ONE),
        2 => Ok(TWO),
        3 => Ok(THREE),
        4 => Ok(FOUR),
        5 => Ok(FIVE),
        6 => Ok(SIX),
        7 => Ok(SEVEN),
        8 => Ok(EIGHT),
        9 => Ok(NINE),
        _ => Err(LedError::InvalidNumber)
    }
}
