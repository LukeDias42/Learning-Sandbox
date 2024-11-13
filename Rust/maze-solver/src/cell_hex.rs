
pub struct CellHex {
    pub sides: [bool; 6],
    pub horizontal_diagonal: Line,
    pub side_len: f64,
    pub visited: bool,
}

