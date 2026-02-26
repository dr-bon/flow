use crate::cursor::{Cursor, Direction, WrapMode};
use crate::document::{DocumentBuffer, DocumentPosition};
use unicode_segmentation::UnicodeSegmentation;

#[allow(dead_code)]
pub struct Editor {
    view_size: (usize, usize),
    pub contents: DocumentBuffer,
    pub cursor: Cursor,
}

impl Editor {
    pub fn new(view_size: (usize, usize)) -> Self {
        Self {
            view_size,
            contents: DocumentBuffer::new(),
            cursor: Cursor::new(),
        }
    }

    pub fn write(&mut self, text: &str, char_idx: usize) -> DocumentPosition {
        self.contents.write(text, char_idx);
        self.shift_cursor(
            Direction::Right,
            text.graphemes(true).count(),
            WrapMode::None,
        )
    }

    pub fn delete(&mut self, start: usize, end: usize) -> DocumentPosition {
        self.contents.delete(start, end);
        self.set_cursor_pos(start)
    }

    pub fn set_cursor_pos(&mut self, _char_idx: usize) -> DocumentPosition {
        // TODO: Implement me
        self.get_cursor_pos()
    }

    pub fn shift_cursor(
        &mut self,
        dir: Direction,
        mag: usize,
        wrap_mode: WrapMode,
    ) -> DocumentPosition {
        self.cursor.shift(&self.contents, dir, mag, wrap_mode);
        self.get_cursor_pos()
    }

    pub fn get_cursor_pos(&self) -> DocumentPosition {
        self.contents.get_position(self.cursor.pos)
    }
}
