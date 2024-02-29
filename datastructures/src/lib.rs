pub mod fifo;
pub mod lifo;
pub mod list;

pub trait FIFO<T> {
    fn enqueue(&mut self, data: T);
    fn dequeue(&mut self) -> Option<T>;
    fn peek(&self) -> Option<&T>;
}

pub trait LIFO<T> {
    fn push(&mut self, data: T);
    fn pop(&mut self) -> Option<T>;
    fn peek(&self) -> Option<&T>;
}

pub trait SinglyLinkedList<T> {
    fn push_front(&mut self, data: T);
    fn pop_front(&mut self) -> Option<T>;
    fn peek_front(&self) -> Option<&T>;
}

pub trait DoublyLinkedList<T>: SinglyLinkedList<T> { 
    fn push_back(&mut self, data: T);
    fn pop_back(&mut self) -> Option<T>; 
    fn peek_back(&self) -> Option<&T>;
}