use std::iter::Iterator;
use std::vec::Vec;

pub struct BufferedIterator<T, TIter: Iterator<Item = T>> {
	itr: TIter,
	buf: Vec<T>,
}

impl<T, TIter: Iterator<Item = T>> BufferedIterator<T, TIter> {
	pub fn new(itr: TIter) -> BufferedIterator<T, TIter> {
		BufferedIterator {
			itr,
			buf: Vec::new(),
		}
	}

	pub fn pop(&mut self) -> Option<T> {
		if self.buf.is_empty() {
			self.itr.next()
		} else {
			self.buf.pop()
		}
	}

	pub fn push(&mut self, item: T) {
		self.buf.push(item);
	}
}
