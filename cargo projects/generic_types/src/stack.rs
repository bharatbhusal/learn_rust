// Implement a generic stack data structure that can hold elements of any type.
// It should have push, pop, and peek methods.
#[derive(Debug)]
pub struct MyStack<T> {
    list: Vec<T>,
    top: usize,
    size: usize,
}

impl<T> MyStack<T> {
    pub fn new(size: usize) -> MyStack<T> {
        MyStack {
            list: Vec::new(),
            top: 0,
            size,
        }
    }
    pub fn get_elelemts_list(&self) -> Option<&Vec<T>> {
        match self.get_top_index() {
            Some(_) => Some(&self.list),
            _ => None,
        }
    }

    fn get_size(&self) -> usize {
        self.size
    }

    fn get_top_index(&self) -> Option<usize> {
        if self.top > 0 {
            Some(self.top)
        } else {
            None
        }
    }

    pub fn push(&mut self, element: T) -> Option<usize> {
        if self.get_top_index() < Some(self.get_size()) {
            self.list.push(element);
            self.top += 1;
            Some(self.get_top_index().unwrap())
        } else {
            None
        }
    }
    pub fn pop(&mut self) -> Option<usize> {
        match self.get_top_index() {
            Some(_) => {
                self.list.pop();
                self.top -= 1;
                Some(self.get_top_index().unwrap())
            }
            None => return None,
        }
    }
    pub fn peek(&self) -> Option<&T> {
        match self.get_top_index() {
            Some(ind) => Some(&self.get_elelemts_list().unwrap()[ind - 1]),
            None => None,
        }
    }
}
