use std::vec::Vec;
use std::iter::Iterator;

pub struct BufferedIterator<T, TIter:Iterator<Item=T>> {
	itr: TIter,
	buf: Vec<T>
}

impl<T, TIter:Iterator<Item=T>> BufferedIterator<T, TIter> {
	pub fn new(itr: TIter) -> BufferedIterator<T, TIter> {
		BufferedIterator {
			itr: itr,
			buf: Vec::new()
		}
	} // new

	pub fn pop(&mut self) -> Option<T> {
		return match self.buf.is_empty() {
			true => self.itr.next(),
			false => self.buf.pop()
		};
	} // pop

	pub fn push(&mut self, item: T) {
		self.buf.push(item);
	} // push
}