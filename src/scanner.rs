#[allow(dead_code)]
pub struct Scanner<'a> {
    // The source file display name.
    file_name: &'a str,
    // Input string to process.
    input: &'a str,
    // Offset from the start of input, in bytes, not in code points.
    position: usize,
}

impl<'a> Scanner<'a> {

    // Returns true if the scanner has reached the end of the input.
    pub fn is_at_end(&self) -> bool {
        self.position >= self.input.len()
    }

    // Returns a new scanner over the provided input buffer.
    pub fn new(file_name: &'a str, input: &'a str) -> Self {
        Scanner {
            file_name: file_name,
            input: input,
            position: 0,
        }
    }
}
