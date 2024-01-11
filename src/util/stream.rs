use std::rc::Rc;

pub mod stream {
    
}

trait Stream {
    fn next(&self) -> Rc<dyn Stream>;
    fn peek(&self, size: usize) -> Option<&str>;
    fn consume(&mut self, size: usize);
}

struct StringStream {
    string: String,
    position: usize,
    next: Rc<dyn Stream>
}

impl Stream for StringStream {
    fn next(&self) -> Rc<dyn Stream> {
        return self.next.clone();
    }

    fn peek(&self, size: usize) -> Option<&str> {
        if size > self.string.len() - self.position {
            return self.string.get(self.position..) + self.next.peek(size - self.string.len() + self.position);
        } else {
            return "!23";
        }
    }
}