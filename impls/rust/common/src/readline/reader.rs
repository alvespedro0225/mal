pub struct Reader<'a> {
    tokens: Box<[&'a str]>,
    pos: usize,
}

impl<'a> Reader<'a> {
    pub fn peek(&self) -> Option<&str> {
        if self.tokens.len() > self.pos {
            Some(self.tokens[self.pos])
        } else {
            None
        }
    }

    pub fn next(&mut self) -> Option<&str> {
        if self.tokens.len() > self.pos {
            self.pos += 1;
            Some(self.tokens[self.pos - 1])
        } else {
            None
        }
    }

    pub fn new(tokens: Box<[&str]>) -> Reader<'_> {
        Reader { tokens, pos: 0 }
    }
}
