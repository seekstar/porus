use crate::pool::{self, Pool};
use crate::stack::Stack;
use alloc::alloc::Global;
use core::marker::PhantomData;
use core::ptr::NonNull;

pub struct Node<H: Copy, T> {
    next: Option<H>,
    data: T,
}

pub struct SinglyLinkedList<T, H: Copy = NonNull<u8>, P: Pool<Node<H, T>, Handle = H> = Global> {
    pool: P,
    sentinel: Option<P::Handle>,
    _data: PhantomData<T>,
}

impl<T, H: Copy, P: Pool<Node<H, T>, Handle = H>> SinglyLinkedList<T, H, P> {
    pub const fn new_in(pool: P) -> Self {
        Self {
            pool,
            sentinel: None,
            _data: PhantomData,
        }
    }
}

impl<T, H: Copy, P: Pool<Node<H, T>, Handle = H> + Default> SinglyLinkedList<T, H, P> {
    #[must_use]
    pub fn new() -> Self {
        Self::new_in(Default::default())
    }
}

impl<T, H: Copy, P: Pool<Node<H, T>, Handle = H> + Default> Default for SinglyLinkedList<T, H, P> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, H: Copy, P: Pool<Node<H, T>, Handle = H>> Stack for SinglyLinkedList<T, H, P> {
    type Elem = T;

    fn is_empty(&self) -> bool {
        self.sentinel.is_none()
    }

    fn push(&mut self, elem: T) {
        let node = Node {
            next: self.sentinel,
            data: elem,
        };
        let handle = pool::add(&mut self.pool, node);
        self.sentinel = Some(handle);
    }

    fn pop(&mut self) -> Option<T> {
        match self.sentinel {
            None => None,
            Some(handle) => {
                let node = pool::take(&mut self.pool, handle);
                self.sentinel = node.next;
                Some(node.data)
            }
        }
    }

    fn top(&self) -> Option<&T> {
        self.sentinel
            .map(|handle| &pool::get(&self.pool, handle).data)
    }
}
