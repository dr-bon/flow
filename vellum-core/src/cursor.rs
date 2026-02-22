use crate::document::DocumentBuffer;


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
    TokenLeft,
    TokenRight,
    LineStart,
    LineEnd,
    DocStart,
    DocEnd,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cursor {
    // Position within the DocumentBuffer / rope
    pub pos: usize,
    // Used to remember the starting column when shifting between rows of different column sizes
    preferred_col: Option<usize>
}

impl Default for Cursor {
    fn default() -> Self {
        Self::new()
    }
}

impl Cursor {

    pub fn new() -> Self {
        Self {
            pos: 0,
            preferred_col: Some(0),
        }
    }

    pub fn shift(&mut self, doc: &DocumentBuffer, dir: Direction, wrap_doc: bool, wrap_line: bool) {
        match dir {
            Direction::Left => {
                // If pos is doc start
                    // if doc wrapping enabled, move to doc end, set preferred col
                    // else, don't move
                // if pos is line start
                    // if line wrapping enabled, move to end of prev line, set preferred col
                    // else, don't move
                // move left, preferred col -= 1
            }
            Direction::Right => {
                // If pos is doc end
                    // if doc wrapping enabled, move to doc start, preferred col = 0
                    // else, don't move
                // if pos is line end
                    // if line wrapping enabled, move to start of next line, preferred col = 0
                    // else, don't move
                // move right, preferred col += 1
            }
            Direction::Up => {
                // if first line
                    // if doc wrap enabled, go to last line at min(first_line_col, last_line_col)
                    // else, don't move
                // go to prev line at min(cur_line_col, prev_line_col)
            }
            Direction::Down => {
                // if last line
                    // if doc wrap enabled, go to first line at min(last_line_col, first_line_col)
                    // else, don't move
                // go to next line at min(cur_line_col, next_line_col)
            }
            Direction::TokenLeft => {
                // if doc start
                    // if doc wrap enabled, go to TokenLeft of last token of last line, set preferred col
                    // else, don't move
                // if line start
                    // if line wrap enabled, go to TokenLeft of last token of prev line, set preferred col
                    // else, don't move
                // else
                    // go to TokenLeft, set preferred col
                }
            Direction::TokenRight => {
                // if doc end
                    // if doc wrap enabled, go to TokenRight of first token of first line, set preferred col
                    // else, don't move
                // if line end
                    // if line wrap enabled, go to TokenRight of first token of next line, set preferred col
                    // else, don't move
                // else
                    // go to TokenRight, set preferred col
                    
            }
            Direction::LineStart => {
                // if line start
                    // do nothing
                // go to current line start, preferred col = 0
            }
            Direction::LineEnd => {
                // if line end
                    // do nothing
                // go to current line end, set preferred col
            }
            Direction::DocStart => {
                // if doc start
                    // do nothing
                // go to doc start, preferred col = 0
            }
            Direction::DocEnd => {
                // if doc end
                    // do nothing
                // go to doc end, set preferred col
            }
        }
    }
}