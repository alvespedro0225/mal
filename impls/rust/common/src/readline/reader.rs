pub struct Reader<'a> {
    tokens: Box<[&'a str]>,
    pos: usize,
}

impl<'a> Reader<'a> {
    pub fn peek(&self) {
        todo!()
    }

    pub fn next(&mut self) {
        todo!()
    }

    pub fn new(tokens: Box<[&str]>) -> Reader<'_> {
        Reader { tokens, pos: 0 }
    }
}
