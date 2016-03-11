// Based on http://www.keithschwarz.com/interesting/code/?dir=fibonacci-heap

use std::mem;
use std::ptr;
use std::cmp::Ordering;

pub struct FibonacciHeap<T: PartialOrd> {
    size: usize,
    min: Link<T>,
    tree_table: Vec<Link<T>>,
    to_visit: Vec<Link<T>>,
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

struct Link<T: PartialOrd> {
    entry: *mut Entry<T>,
}

impl<T: PartialOrd> FibonacciHeap<T> {
    pub fn new() -> FibonacciHeap<T> {
        FibonacciHeap {
            min: Link::none(),
            size: 0,
            to_visit: Vec::new(),
            tree_table: Vec::new(),
        }
    }

    pub fn min(&self) -> Option<&T> {
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

        let mut min = Link::none();
        mem::swap(&mut min, &mut self.min);

        self.min = FibonacciHeap::<T>::merge_entries(min, link);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        self.size -= 1;

        let mut min = Link::none();
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
                current.set_parent(&Link::none());
                current = current.get_next();

                !current.are_same(&min_child)
            } {}
        }

        let mut new_min = Link::none();
        mem::swap(&mut new_min, &mut self.min);

        self.min = FibonacciHeap::<T>::merge_entries(new_min, min_child);

        if self.size == 0 {
            return min.into_value();
        }

        let mut current = self.min.clone();
        while self.to_visit.is_empty() || !self.to_visit[0].are_same(&current) {
            let next = current.get_next();
            self.to_visit.push(current);
            current = next;
        }

        for link_to_visit in &self.to_visit {
            let mut link = link_to_visit.clone();

            loop {
                let link_degree = link.get_degree();

                while link_degree >= self.tree_table.len() {
                    self.tree_table.push(Link::none());
                }

                if self.tree_table[link_degree].is_none() {
                    self.tree_table[link_degree] = link.clone();
                    break;
                }

                self.tree_table.push(Link::none());
                let other = self.tree_table.swap_remove(link_degree);

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

        self.to_visit.clear();
        self.tree_table.clear();

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
            Link::none()
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

            if x < y {
                x
            } else {
                y
            }
        }
    }
}

impl<T: PartialOrd> Entry<T> {
    pub fn new(value: T) -> Entry<T> {
        Entry {
            value: value,

            degree: 0,
            // is_marked: false,
            parent: Link::none(),
            next: Link::none(),
            prev: Link::none(),
            child: Link::none(),
        }
    }
}

impl<T: PartialOrd> Link<T> {
    #[inline]
    pub fn none() -> Link<T> {
        Link { entry: ptr::null_mut() }
    }

    #[inline]
    pub fn is_none(&self) -> bool {
        self.entry.is_null()
    }

    #[inline]
    pub fn is_some(&self) -> bool {
        !self.is_none()
    }

    pub fn new(value: T) -> Link<T> {
        let entry_box = Box::new(Entry::new(value));
        let entry = Box::into_raw(entry_box);
        let prev = entry.clone();
        let next = entry.clone();

        unsafe {
            (*entry).prev = Link { entry: prev };
            (*entry).next = Link { entry: next };
        }

        Link { entry: entry }
    }

    #[inline]
    pub fn get_degree(&self) -> usize {
        if self.is_none() {
            0
        } else {
            unsafe { (*self.entry).degree }
        }
    }

    #[inline]
    pub fn inc_degree(&mut self) {
        if self.is_some() {
            unsafe {
                (*self.entry).degree += 1;
            }
        }
    }

    pub fn into_value(mut self) -> Option<T> {
        if self.is_none() {
            None
        } else {
            let mut raw_entry: *mut Entry<T> = ptr::null_mut();
            mem::swap(&mut raw_entry, &mut self.entry);

            let entry: Entry<T>;
            unsafe {
                entry = *Box::from_raw(raw_entry);
            }

            Some(entry.value)
        }
    }

    #[inline]
    pub fn borrow(&self) -> Option<&T> {
        if self.is_none() {
            None
        } else {
            unsafe { Some(&(*self.entry).value) }
        }
    }

    #[inline]
    pub fn are_same(&self, other: &Link<T>) -> bool {
        self.entry == other.entry
    }

    #[inline]
    pub fn get_child(&self) -> Link<T> {
        if self.is_none() {
            Link::<T>::none()
        } else {
            unsafe { (*self.entry).child.clone() }
        }
    }

    #[inline]
    pub fn get_next(&self) -> Link<T> {
        if self.is_none() {
            Link::<T>::none()
        } else {
            unsafe { (*self.entry).next.clone() }
        }
    }

    #[inline]
    pub fn get_prev(&self) -> Link<T> {
        if self.is_none() {
            Link::<T>::none()
        } else {
            unsafe { (*self.entry).prev.clone() }
        }
    }

    #[inline]
    pub fn set_child(&mut self, child: &Link<T>) {
        if self.is_some() {
            unsafe {
                (*self.entry).child = child.clone();
            }
        }
    }

    #[inline]
    pub fn set_parent(&mut self, parent: &Link<T>) {
        if self.is_some() {
            unsafe {
                (*self.entry).parent = parent.clone();
            }
        }
    }

    #[inline]
    pub fn set_next(&mut self, next: &Link<T>) {
        if self.is_some() {
            unsafe {
                (*self.entry).next = next.clone();
            }
        }
    }

    #[inline]
    pub fn set_prev(&mut self, prev: &Link<T>) {
        if self.is_some() {
            unsafe {
                (*self.entry).prev = prev.clone();
            }
        }
    }
}

impl<T: PartialOrd> Clone for Link<T> {
    #[inline]
    fn clone(&self) -> Self {
        Link { entry: self.entry.clone() }
    }
}

impl<T: PartialOrd> PartialEq for Link<T> {
    #[inline]
    fn eq(&self, other: &Link<T>) -> bool {
        if self.is_none() || other.is_none() {
            false
        } else {
            unsafe { (*self.entry).value.eq(&(*other.entry).value) }
        }
    }
}

impl<T: PartialOrd> PartialOrd for Link<T> {
    #[inline]
    fn partial_cmp(&self, other: &Link<T>) -> Option<Ordering> {
        if self.is_none() || other.is_none() {
            None
        } else {
            unsafe { (*self.entry).value.partial_cmp(&(*other.entry).value) }
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
fn bench_push_pop(b: &mut ::test::Bencher) {
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
