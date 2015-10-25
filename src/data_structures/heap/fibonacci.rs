// Based on http://www.keithschwarz.com/interesting/code/?dir=fibonacci-heap

use std::mem;
use std::rc::Rc;
use std::cell::{RefCell, Ref};
use std::cmp::Ordering;

pub struct FibonacciHeap<T: PartialOrd> {
    size: usize,
    min: Link<T>,
}

struct Entry<T: PartialOrd> {
    pub value: T,

    pub degree: usize,
    // TODO: Implement decrease_key and delete
    // pub is_marked: bool,

    pub parent: Link<T>,
    pub next: Link<T>,
    pub prev: Link<T>,
    pub child: Link<T>,
}

enum Link<T: PartialOrd> {
    None,
    Some(Rc<RefCell<Entry<T>>>)
}

impl <T: PartialOrd> FibonacciHeap<T> {
    pub fn new() -> FibonacciHeap<T> {
        FibonacciHeap {
            min: Link::None,
            size: 0,
        }
    }

    pub fn min(&self) -> Option<Ref<T>> {
        self.min.borrow()
    }

    pub fn is_empty(&self) -> bool {
        self.min.is_none()
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn push(&mut self, value: T) {
        let link = Link::new(value);

        let mut min: Link<T> = Link::None;
        mem::swap(&mut min, &mut self.min);

        self.min = FibonacciHeap::<T>::merge_entries(min, link);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        self.size -= 1;

        let mut min = Link::None;
        mem::swap(&mut min, &mut self.min);

        if !min.get_next().are_same(&min) {
            let mut min_prev = min.get_prev();
            let mut min_next = min.get_next();
            min_prev.set_next(&min_next);
            min_next.set_prev(&min_prev);
            self.min = min_next;
        }

        let min_child = min.get_child();
        if !min_child.is_none() {
            let mut current = min_child.clone();
            while {
                current.set_parent(&Link::None);
                current = current.get_next();

                !current.are_same(&min_child)
            } {}
        }

        let mut new_min = Link::None;
        mem::swap(&mut new_min, &mut self.min);

        self.min = FibonacciHeap::<T>::merge_entries(new_min, min_child);

        if self.size == 0 {
            return min.into_value();
        }

        let mut tree_table = vec![];
        let mut to_visit = Vec::<Link<T>>::new();

        let mut current = self.min.clone();
        while to_visit.is_empty() || !to_visit[0].are_same(&current) {
            let next = current.get_next();
            to_visit.push(current);
            current = next;
        }

        for link_to_visit in to_visit {
            let mut link = link_to_visit;

            loop {
                let link_degree = link.get_degree();

                while link_degree >= tree_table.len() {
                    tree_table.push(Link::None);
                }

                if tree_table[link_degree].is_none() {
                    tree_table[link_degree] = link.clone();
                    break;
                }

                tree_table.push(Link::None);
                let other = tree_table.swap_remove(link_degree);

                let (mut min_link, mut max) = if link < other {
                    (link, other)
                } else {
                    (other, link)
                };

                let mut max_next = max.get_next();
                let mut max_prev = max.get_prev();
                max_next.set_prev(&max_prev);
                max_prev.set_next(&max_next);

                let max_clone = max.clone();
                max.set_prev(&max_clone);
                max.set_next(&max_clone);

                max.set_parent(&min_link);

                let min_link_child = min_link.get_child();
                min_link.set_child(&FibonacciHeap::<T>::merge_entries(min_link_child, max));
                // max.is_marked = false;

                min_link.inc_degree();

                link = min_link;
            }

            if link <= self.min {
                self.min = link;
            }
        }

        min.into_value()
    }

    pub fn merge(x: FibonacciHeap<T>, y: FibonacciHeap<T>) -> FibonacciHeap<T> {
        let mut result = FibonacciHeap::new();

        result.min = FibonacciHeap::<T>::merge_entries(x.min, y.min);
        result.size = x.size + y.size;

        result
    }

    fn merge_entries(mut x: Link<T>, mut y: Link<T>) -> Link<T> {
        if x.is_none() && y.is_none() {
            Link::None
        } else if !x.is_none() && y.is_none() {
            x
        } else if x.is_none() && !y.is_none() {
            y
        } else {
            let mut x_next = x.get_next();
            let mut y_next = y.get_next();
            x.set_next(&y_next);
            y_next.set_prev(&x);
            y.set_next(&x_next);
            x_next.set_prev(&y);

            if x < y { x } else { y }
        }
    }
}

impl <T: PartialOrd> Entry<T> {
    pub fn new(value: T) -> Entry<T> {
        Entry {
            value: value,

            degree: 0,
            // is_marked: false,

            parent: Link::None,
            next: Link::None,
            prev: Link::None,
            child: Link::None,
        }
    }
}

impl <T: PartialOrd> Link<T> {
    pub fn is_none(&self) -> bool {
        match self {
            &Link::None => true,
            _ => false
        }
    }

    pub fn new(value: T) -> Link<T> {
        let entry = Entry::new(value);
        let rc = Rc::new(RefCell::new(entry));
        let prev = rc.clone();
        let next = rc.clone();

        {
            let mut mut_entry = (*rc).borrow_mut();
            (*mut_entry).next = Link::Some(next);
            (*mut_entry).prev = Link::Some(prev);
        }

        Link::Some(rc)
    }

    pub fn get_degree(&self) -> usize {
        match self {
            &Link::None => 0,
            &Link::Some(ref rc) => rc.borrow().degree
        }
    }

    pub fn inc_degree(&mut self) {
        match self {
            &mut Link::Some(ref rc) => rc.borrow_mut().degree += 1,
            _ => {}
        }
    }

    pub fn into_value(mut self) -> Option<T> {
        self.set_next(&Link::None);
        self.set_prev(&Link::None);

        match self {
            Link::None => None,
            Link::Some(rc) => {
                let cell = Rc::try_unwrap(rc).ok().unwrap();
                let entry = cell.into_inner();

                Some(entry.value)
            }
        }
    }

    pub fn borrow(&self) -> Option<Ref<T>> {
        match self {
            &Link::Some(ref rc) => Some(Ref::map(rc.borrow(), |entry| &entry.value)),
            &Link::None => None
        }
    }

    pub fn are_same(&self, other: &Link<T>) -> bool {
        match self {
            &Link::None => other.is_none(),
            &Link::Some(ref rc) => {
                match other {
                    &Link::None => false,
                    &Link::Some(ref other_rc) => {
                        &(*rc.borrow()) as *const Entry<T> == &(*other_rc.borrow()) as *const Entry<T>
                    }
                }
            }
        }
    }

    pub fn get_child(&self) -> Link<T> {
        match self {
            &Link::Some(ref rc) => {
                let entry = rc.borrow();
                match &entry.child {
                    &Link::Some(ref child_rc) => Link::Some(child_rc.clone()),
                    &Link::None => Link::None
                }
            },
            &Link::None => Link::None
        }
    }

    pub fn get_next(&self) -> Link<T> {
        match self {
            &Link::Some(ref rc) => {
                let entry = rc.borrow();
                match &entry.next {
                    &Link::Some(ref next_rc) => Link::Some(next_rc.clone()),
                    &Link::None => Link::None
                }
            },
            &Link::None => Link::None
        }
    }

    pub fn get_prev(&self) -> Link<T> {
        match self {
            &Link::Some(ref rc) => {
                let entry = rc.borrow();
                match &entry.prev {
                    &Link::Some(ref prev_rc) => Link::Some(prev_rc.clone()),
                    &Link::None => Link::None
                }
            },
            &Link::None => Link::None
        }
    }

    pub fn set_child(&mut self, child: &Link<T>) {
        match self {
            &mut Link::Some(ref rc) => {
                let mut entry = rc.borrow_mut();
                entry.child = child.clone();
            },
            _ => {}
        }
    }

    pub fn set_parent(&mut self, parent: &Link<T>) {
        match self {
            &mut Link::Some(ref rc) => {
                let mut entry = rc.borrow_mut();
                entry.parent = parent.clone();
            },
            _ => {}
        }
    }

    pub fn set_next(&mut self, next: &Link<T>) {
        match self {
            &mut Link::Some(ref rc) => {
                let mut entry = rc.borrow_mut();
                entry.next = next.clone();
            },
            _ => {}
        }
    }

    pub fn set_prev(&mut self, prev: &Link<T>) {
        match self {
            &mut Link::Some(ref rc) => {
                let mut entry = rc.borrow_mut();
                entry.prev = prev.clone();
            },
            _ => {}
        }
    }
}

impl <T: PartialOrd> Clone for Link<T> {
    fn clone(&self) -> Self {
        match self {
            &Link::None => Link::None,
            &Link::Some(ref rc) => Link::Some(rc.clone())
        }
    }
}

impl <T: PartialOrd> PartialEq for Link<T> {
    #[inline]
    fn eq(&self, other: &Link<T>) -> bool {
        match self {
            &Link::None => false,
            &Link::Some(ref rc) => {
                match other {
                    &Link::None => false,
                    &Link::Some(ref other_rc) => {
                        rc.borrow().value.eq(&other_rc.borrow().value)
                    }
                }
            }
        }
    }
}

impl <T: PartialOrd> PartialOrd for Link<T> {
    #[inline]
    fn partial_cmp(&self, other: &Link<T>) -> Option<Ordering> {
        match self {
            &Link::None => None,
            &Link::Some(ref rc) => {
                match other {
                    &Link::None => None,
                    &Link::Some(ref other_rc) => {
                        rc.borrow().value.partial_cmp(&other_rc.borrow().value)
                    }
                }
            }
        }
    }
}

#[test]
fn test_size() {
    let mut heap = FibonacciHeap::new();

    heap.push(1);

    assert_eq!(1, heap.size);
}

#[test]
fn test_min() {
    let mut heap = FibonacciHeap::new();

    heap.push(1);

    assert_eq!(1, *heap.min().unwrap());
}

#[test]
fn test_pop() {
    let mut heap = FibonacciHeap::new();

    heap.push(1);

    assert_eq!(1, heap.pop().unwrap());
}

#[test]
fn test_order() {
    let mut heap = FibonacciHeap::new();

    heap.push(7);
    heap.push(1);
    heap.push(8);
    heap.push(4);
    heap.push(5);
    heap.push(2);
    heap.push(3);
    heap.push(6);

    assert_eq!(1, heap.pop().unwrap());
    assert_eq!(2, heap.pop().unwrap());
    assert_eq!(3, heap.pop().unwrap());
    assert_eq!(4, heap.pop().unwrap());
    assert_eq!(5, heap.pop().unwrap());
    assert_eq!(6, heap.pop().unwrap());
    assert_eq!(7, heap.pop().unwrap());
    assert_eq!(8, heap.pop().unwrap());
}

#[bench]
fn bench_push_pop_fibonacci(b: &mut ::test::Bencher) {
    b.iter(|| {
        // TODO: Optimize heap
        let mut heap = FibonacciHeap::new();

        for i in 1..10001 {
            heap.push(10001 - i);
        }

        for _ in 1..10001 {
            heap.pop();
        }
    })
}

#[bench]
fn bench_push_pop_binary(b: &mut ::test::Bencher) {
    b.iter(|| {
        let mut heap = ::std::collections::BinaryHeap::new();

        for i in 1..10001 {
            heap.push(10001 - i);
        }

        for _ in 1..10001 {
            heap.pop();
        }
    })
}
