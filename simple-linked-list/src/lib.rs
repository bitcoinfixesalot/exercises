pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    pub fn len(&self) -> usize {
        let mut counter = 0;
        let mut node = self.head.as_ref();
        while let Some(n) = node {
            node = n.next.as_ref();
            counter += 1;
        }
        counter
    }

    pub fn push(&mut self, _element: T) {
        let node = Some(Box::new(Node {
            data: _element,
            next: self.head.take(),
        }));
        self.head = node;
    }

    pub fn pop(&mut self) -> Option<T> {
        let mut head = self.head.take();
        self.head = match head {
            Some(ref mut node) => node.next.take(),
            None => None,
        };
        head.map(|boxed| boxed.data)
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|boxed| &boxed.data)
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        let mut linked_list = SimpleLinkedList::new();
        let mut head = &self.head;

        while let Some(node) = head {
            linked_list.push(node.data.clone());
            head = &node.next;
        }
        linked_list
    }
}

impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(_item: &[T]) -> Self {
        let mut linked_list = SimpleLinkedList::new();

        for item in _item.iter() {
            linked_list.push(item.clone())
        }
        linked_list
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(mut self) -> Vec<T> {
        let mut result: Vec<T> = Vec::new();
        while let Some(value) = self.pop() {
            result.push(value);
        }
        result.reverse();
        result
    }
}
