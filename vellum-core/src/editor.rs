use crate::cursor::{Cursor, Direction};
use crate::document::{DocumentBuffer, DocumentPosition};

pub struct Editor {
    view_size: (usize, usize),
    pub contents: DocumentBuffer,
    pub cursor: Cursor,
}

impl Editor {
    
    pub fn new(view_size: (usize, usize)) -> Self {
        Self {
            view_size: view_size,
            contents: DocumentBuffer::new(),
            cursor: Cursor::new(),
        }
    }


    pub fn shift_cursor(&mut self, dir: Direction, wrap_doc: bool, wrap_line: bool) {
        self.cursor.shift(&self.contents, dir, wrap_doc, wrap_line);
    }

    pub fn get_cursor_pos(&self) -> DocumentPosition {
        self.contents.get_position(self.cursor.pos)
    }
}