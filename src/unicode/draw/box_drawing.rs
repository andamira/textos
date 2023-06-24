// textos::unicode::drawing::box_drawing
//
//! Box-drawing characters.
//!
//! a form of graphical text elements used in monospaced fonts.
//!
//! They can be used to create tables, charts, frames, and other structures
//! within text-only environments.
//!
//! - <https://en.wikipedia.org/wiki/Box-drawing_character>
//

///
pub struct BoxDrawing {
    // Can be customized.
    thickness: BoxDrawingThickness,
}

impl BoxDrawing {
    pub const fn new(thickness: BoxDrawingThickness) -> Self {
        Self { thickness }
    }

    pub fn thickness(&self) -> BoxDrawingThickness {
        self.thickness
    }

    pub fn set_thickness(&mut self, thickness: BoxDrawingThickness) {
        self.thickness = thickness;
    }
}

/// # Constants
impl BoxDrawing {
    // LIGHT
    pub const HORIZONTAL_LIGHT: char = '─';
    pub const VERTICAL_LIGHT: char = '│';
    pub const DOWN_RIGHT_LIGHT: char = '┌';
    pub const DOWN_LEFT_LIGHT: char = '┐';
    pub const UP_RIGHT_LIGHT: char = '└';
    pub const UP_LEFT_LIGHT: char = '┘';
    pub const VERTICAL_RIGHT_LIGHT: char = '├';
    pub const VERTICAL_LEFT_LIGHT: char = '┤';
    pub const DOWN_HORIZONTAL_LIGHT: char = '┬';
    pub const UP_HORIZONTAL_LIGHT: char = '┴';
    pub const VERTICAL_HORIZONTAL_LIGHT: char = '┼';

    // HEAVY
    pub const HORIZONTAL_HEAVY: char = '━';
    pub const VERTICAL_HEAVY: char = '┃';
    pub const DOWN_RIGHT_HEAVY: char = '┏';
    pub const DOWN_LEFT_HEAVY: char = '┓';
    pub const UP_RIGHT_HEAVY: char = '┗';
    pub const UP_LEFT_HEAVY: char = '┛';
    pub const VERTICAL_RIGHT_HEAVY: char = '┣';
    pub const VERTICAL_LEFT_HEAVY: char = '┫';
    pub const DOWN_HORIZONTAL_HEAVY: char = '┳';
    pub const UP_HORIZONTAL_HEAVY: char = '┻';
    pub const VERTICAL_HORIZONTAL_HEAVY: char = '╋';

    // DOUBLE
    pub const HORIZONTAL_DOUBLE: char = '═';
    pub const VERTICAL_DOUBLE: char = '║';
    pub const DOWN_RIGHT_DOUBLE: char = '╔';
    pub const DOWN_LEFT_DOUBLE: char = '╗';
    pub const UP_RIGHT_DOUBLE: char = '╚';
    pub const UP_LEFT_DOUBLE: char = '╝';
    pub const VERTICAL_RIGHT_DOUBLE: char = '╠';
    pub const VERTICAL_LEFT_DOUBLE: char = '╣';
    pub const DOWN_HORIZONTAL_DOUBLE: char = '╦';
    pub const UP_HORIZONTAL_DOUBLE: char = '╩';
    pub const VERTICAL_HORIZONTAL_DOUBLE: char = '╬';

    // ROUND
    pub const DOWN_RIGHT_ROUND: char = '╭';
    pub const DOWN_LEFT_ROUND: char = '╮';
    pub const UP_LEFT_ROUND: char = '╯';
    pub const UP_RIGHT_ROUND: char = '╰';
}

/// # Abbreviations
impl BoxDrawing {
    // LIGHT
    pub const H0: char = Self::HORIZONTAL_LIGHT;
    pub const V0: char = Self::VERTICAL_LIGHT;
    pub const DR0: char = Self::DOWN_RIGHT_LIGHT;
    pub const DL0: char = Self::DOWN_LEFT_LIGHT;
    pub const UR0: char = Self::UP_RIGHT_LIGHT;
    pub const UL0: char = Self::UP_LEFT_LIGHT;
    pub const VR0: char = Self::VERTICAL_RIGHT_LIGHT;
    pub const VL0: char = Self::VERTICAL_LEFT_LIGHT;
    pub const DH0: char = Self::DOWN_HORIZONTAL_LIGHT;
    pub const UH0: char = Self::UP_HORIZONTAL_LIGHT;
    pub const VH0: char = Self::VERTICAL_HORIZONTAL_LIGHT;

    // HEAVY
    pub const H1: char = Self::HORIZONTAL_HEAVY;
    pub const V1: char = Self::VERTICAL_HEAVY;
    pub const DR1: char = Self::DOWN_RIGHT_HEAVY;
    pub const DL1: char = Self::DOWN_LEFT_HEAVY;
    pub const UR1: char = Self::UP_RIGHT_HEAVY;
    pub const UL1: char = Self::UP_LEFT_HEAVY;
    pub const VR1: char = Self::VERTICAL_RIGHT_HEAVY;
    pub const VL1: char = Self::VERTICAL_LEFT_HEAVY;
    pub const DH1: char = Self::DOWN_HORIZONTAL_HEAVY;
    pub const UH1: char = Self::UP_HORIZONTAL_HEAVY;
    pub const VH1: char = Self::VERTICAL_HORIZONTAL_HEAVY;

    // DOUBLE
    pub const H2: char = Self::HORIZONTAL_DOUBLE;
    pub const V2: char = Self::VERTICAL_DOUBLE;
    pub const DR2: char = Self::DOWN_RIGHT_DOUBLE;
    pub const DL2: char = Self::DOWN_LEFT_DOUBLE;
    pub const UR2: char = Self::UP_RIGHT_DOUBLE;
    pub const UL2: char = Self::UP_LEFT_DOUBLE;
    pub const VR2: char = Self::VERTICAL_RIGHT_DOUBLE;
    pub const VL2: char = Self::VERTICAL_LEFT_DOUBLE;
    pub const DH2: char = Self::DOWN_HORIZONTAL_DOUBLE;
    pub const UH2: char = Self::UP_HORIZONTAL_DOUBLE;
    pub const VH2: char = Self::VERTICAL_HORIZONTAL_DOUBLE;

    // ROUND
    pub const DR3: char = Self::DOWN_RIGHT_ROUND;
    pub const DL3: char = Self::DOWN_LEFT_ROUND;
    pub const UR3: char = Self::UP_RIGHT_ROUND;
    pub const UL3: char = Self::UP_LEFT_ROUND;
}

/// # Functions
impl BoxDrawing {
    ///
    pub const fn piece(piece: BoxDrawingPiece, thickness: BoxDrawingThickness) -> char {
        use BoxDrawingPiece::*;
        use BoxDrawingThickness::*;
        match thickness {
            Light => match piece {
                Horizontal => Self::H0,
                Vertical => Self::V0,
                DownRight => Self::DR0,
                DownLeft => Self::DL0,
                UpRight => Self::UR0,
                UpLeft => Self::UL0,
                VerticalRight => Self::VR0,
                VerticalLeft => Self::VL0,
                DownHorizontal => Self::DH0,
                UpHorizontal => Self::UH0,
                VerticalHorizontal => Self::VH0,
            },
            Heavy => match piece {
                Horizontal => Self::H1,
                Vertical => Self::V1,
                DownRight => Self::DR1,
                DownLeft => Self::DL1,
                UpRight => Self::UR1,
                UpLeft => Self::UL1,
                VerticalRight => Self::VR1,
                VerticalLeft => Self::VL1,
                DownHorizontal => Self::DH1,
                UpHorizontal => Self::UH1,
                VerticalHorizontal => Self::VH1,
            },
            Double => match piece {
                Horizontal => Self::H2,
                Vertical => Self::V2,
                DownRight => Self::DR2,
                DownLeft => Self::DL2,
                UpRight => Self::UR2,
                UpLeft => Self::UL2,
                VerticalRight => Self::VR2,
                VerticalLeft => Self::VL2,
                DownHorizontal => Self::DH2,
                UpHorizontal => Self::UH2,
                VerticalHorizontal => Self::VH2,
            },
            Round => {
                match piece {
                    Horizontal => Self::H0,
                    Vertical => Self::V0,
                    //
                    DownRight => Self::DR1,
                    DownLeft => Self::DL1,
                    UpRight => Self::UR1,
                    UpLeft => Self::UL1,
                    //
                    VerticalRight => Self::VR0,
                    VerticalLeft => Self::VL0,
                    DownHorizontal => Self::DH0,
                    UpHorizontal => Self::UH0,
                    VerticalHorizontal => Self::VH0,
                }
            }
        }
    }

    ///
    #[inline]
    pub const fn p(&self, piece: BoxDrawingPiece) -> char {
        Self::piece(piece, self.thickness)
    }

    // pub fn ipiece(piece: impl Into<BoxDrawingPiece>, thickness: impl Into<BoxDrawingThickness>) -> char {
    //     Self::cpiece(piece.into(), thickness.into())
    // }
}

///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BoxDrawingPiece {
    Horizontal,
    Vertical,
    DownRight,
    DownLeft,
    UpRight,
    UpLeft,
    VerticalRight,
    VerticalLeft,
    DownHorizontal,
    UpHorizontal,
    VerticalHorizontal,
}

/// # Aliases
#[allow(non_upper_case_globals)]
impl BoxDrawingPiece {
    pub const RightDown: Self = Self::DownRight;
    pub const LeftDown: Self = Self::DownLeft;
    pub const LeftRight: Self = Self::UpRight;
    pub const LeftUp: Self = Self::UpLeft;
    pub const RightVertical: Self = Self::VerticalRight;
    pub const LeftVertical: Self = Self::VerticalLeft;
    pub const HorizontalDown: Self = Self::DownHorizontal;
    pub const HorizontalUp: Self = Self::UpHorizontal;
    pub const HorizontalVertical: Self = Self::VerticalHorizontal;
}

/// # Abbreviations
impl BoxDrawingPiece {
    pub const H: Self = Self::Horizontal;
    pub const V: Self = Self::Vertical;

    pub const RD: Self = Self::DownRight;
    pub const LD: Self = Self::DownLeft;
}

///
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoxDrawingThickness {
    ///
    Light = 0,

    ///
    Heavy = 1,

    ///
    Double = 2,

    /// Round corners, light walls.
    Round = 3,
}
