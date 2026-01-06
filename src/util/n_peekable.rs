use std::collections::VecDeque;

pub struct NPeekable<I: Iterator> {
    iter: I,
    cache: VecDeque<I::Item>,
}

impl<I: Iterator> NPeekable<I> {
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            cache: VecDeque::new(),
        }
    }

    pub fn peek_n(&mut self, idx: usize) -> Option<&I::Item> {
        while self.cache.len() <= idx {
            self.cache.push_back(self.iter.next()?);
        }
        self.cache.get(idx)
    }

    pub fn peek(&mut self) -> Option<&I::Item> {
        self.peek_n(0)
    }

    pub fn drop_n(&mut self, n: usize) -> Option<()> {
        for _ in 0..n {
            self.next()?;
        }
        Some(())
    }
}

impl<I: Iterator> Iterator for NPeekable<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.cache.pop_front().or_else(|| self.iter.next())
    }
}
