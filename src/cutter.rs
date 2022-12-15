pub struct Cutter {
    input: Vec<char>,
    start: usize,
    current: usize,
    pub line: usize,
}

impl Cutter {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn copy(&self) -> String {
        self.input[self.start..self.current].into_iter().collect()
    }

    pub fn cut(&mut self) -> String {
        let lexeme = self.copy();

        self.start = self.current;

        lexeme
    }

    pub fn peek(&self) -> Option<char> {
        self.input.get(self.current).copied()
    }

    pub fn next(&mut self) -> Option<char> {
        let symbol = self.peek();

        if let Some(c) = symbol {
            if c == '\n' {
                self.line += 1;
            }
            self.current += 1;
        }

        symbol
    }

    pub fn next_if<Pred: Fn(char) -> bool>(&mut self, pred: Pred) -> bool {
        let cond = self.peek().map(&pred).unwrap_or(false);

        if cond {
            self.next();
        }

        cond
    }

    pub fn next_eq(&mut self, c: char) -> bool {
        self.next_if(|x| x == c)
    }

    pub fn next_while<Pred: Fn(char) -> bool>(&mut self, pred: Pred) {
        while self.next_if(&pred) {
        }
    }

    pub fn eof(&self) -> bool {
        self.peek().is_none()
    }
}
