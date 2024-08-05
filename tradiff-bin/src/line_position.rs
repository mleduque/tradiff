
#[derive(Debug, Clone, PartialEq)]
pub struct LinePosition {
    /// Line number (counting from 1)
    pub line: usize,
    /// Row number (counting from 1)
    pub col: usize,
}

impl LinePosition {
    pub fn from_offset(text: &str, offset: usize) -> Option<Self> {
        if offset + 1 > text.len() {
            return None;
        }
        let mut line_start = 0;
        let mut line_num = 1usize;
        for line in text.split('\n') {
            let line_end = line_start + line.len() + "\n".len();
            if line_end > offset {
                // done searching
                return Some(Self { line: line_num, col: offset - line_start + 1 })
            }
            line_num += 1;
            line_start = line_end;
        }
        None
    }

    #[cfg(test)]
    pub fn new(line: usize, col: usize) -> Self {
        Self { line, col }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::line_position::LinePosition;

    #[test]
    fn three_lines_all_with_some_content() {
        let text = "abc\n12\nEFG";
        assert_eq!(LinePosition::from_offset(text, 0), Some(LinePosition::new(1, 1))); // a
        assert_eq!(LinePosition::from_offset(text, 1), Some(LinePosition::new(1, 2))); // b
        assert_eq!(LinePosition::from_offset(text, 2), Some(LinePosition::new(1, 3))); // c
        assert_eq!(LinePosition::from_offset(text, 3), Some(LinePosition::new(1, 4))); // \n
        assert_eq!(LinePosition::from_offset(text, 4), Some(LinePosition::new(2, 1))); // 1
        assert_eq!(LinePosition::from_offset(text, 5), Some(LinePosition::new(2, 2))); // 2
        assert_eq!(LinePosition::from_offset(text, 6), Some(LinePosition::new(2, 3))); // \n
        assert_eq!(LinePosition::from_offset(text, 7), Some(LinePosition::new(3, 1))); // E
        assert_eq!(LinePosition::from_offset(text, 8), Some(LinePosition::new(3, 2))); // F
        assert_eq!(LinePosition::from_offset(text, 9), Some(LinePosition::new(3, 3))); // G
        assert_eq!(LinePosition::from_offset(text, 10), None); // past EOF
        assert_eq!(LinePosition::from_offset(text, 11), None); // past EOF
    }

    #[test]
    fn three_lines_all_with_some_content_andfourth_empty_line_at_end() {
        let text = "abc\n12\nEFG\n";
        assert_eq!(LinePosition::from_offset(text, 0), Some(LinePosition::new(1, 1))); // a
        assert_eq!(LinePosition::from_offset(text, 1), Some(LinePosition::new(1, 2))); // b
        assert_eq!(LinePosition::from_offset(text, 2), Some(LinePosition::new(1, 3))); // c
        assert_eq!(LinePosition::from_offset(text, 3), Some(LinePosition::new(1, 4))); // \n
        assert_eq!(LinePosition::from_offset(text, 4), Some(LinePosition::new(2, 1))); // 1
        assert_eq!(LinePosition::from_offset(text, 5), Some(LinePosition::new(2, 2))); // 2
        assert_eq!(LinePosition::from_offset(text, 6), Some(LinePosition::new(2, 3))); // \n
        assert_eq!(LinePosition::from_offset(text, 7), Some(LinePosition::new(3, 1))); // E
        assert_eq!(LinePosition::from_offset(text, 8), Some(LinePosition::new(3, 2))); // F
        assert_eq!(LinePosition::from_offset(text, 9), Some(LinePosition::new(3, 3))); // G
        assert_eq!(LinePosition::from_offset(text, 10), Some(LinePosition::new(3, 4))); // \n
        assert_eq!(LinePosition::from_offset(text, 11), None); // past EOF
    }

    #[test]
    fn three_lines_with_first_empty() {
        let text = "\n12\nEFG\n";
        assert_eq!(LinePosition::from_offset(text, 0), Some(LinePosition::new(1, 1))); // \n
        assert_eq!(LinePosition::from_offset(text, 1), Some(LinePosition::new(2, 1))); // 1
        assert_eq!(LinePosition::from_offset(text, 2), Some(LinePosition::new(2, 2))); // 2
        assert_eq!(LinePosition::from_offset(text, 3), Some(LinePosition::new(2, 3))); // \n
        assert_eq!(LinePosition::from_offset(text, 4), Some(LinePosition::new(3, 1))); // E
        assert_eq!(LinePosition::from_offset(text, 5), Some(LinePosition::new(3, 2))); // F
        assert_eq!(LinePosition::from_offset(text, 6), Some(LinePosition::new(3, 3))); // G
        assert_eq!(LinePosition::from_offset(text, 7), Some(LinePosition::new(3, 4))); // \n
        assert_eq!(LinePosition::from_offset(text, 8), None);
        assert_eq!(LinePosition::from_offset(text, 9),None);
    }

    #[test]
    fn empty_text() {
        let text = "";
        assert_eq!(LinePosition::from_offset(text, 0), None);
        assert_eq!(LinePosition::from_offset(text, 1), None);
        assert_eq!(LinePosition::from_offset(text, 2), None);
    }

    #[test]
    fn single_empty_line() {
        let text = "\n";
        assert_eq!(LinePosition::from_offset(text, 0), Some(LinePosition::new(1, 1))); // \n
        assert_eq!(LinePosition::from_offset(text, 1), None);
        assert_eq!(LinePosition::from_offset(text, 2), None);
    }

    #[test]
    fn single_line_with_content() {
        let text = "ab\n";
        assert_eq!(LinePosition::from_offset(text, 0), Some(LinePosition::new(1, 1))); // a
        assert_eq!(LinePosition::from_offset(text, 1), Some(LinePosition::new(1, 2))); // b
        assert_eq!(LinePosition::from_offset(text, 2), Some(LinePosition::new(1, 3))); // \n
        assert_eq!(LinePosition::from_offset(text, 3), None);
    }
}
