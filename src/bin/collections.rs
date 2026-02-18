//! Examples showing basic use of Rust Collections (from std::collections).

//use std::collections::{BinaryHeap, BTreeMap, BTreeSet, HashMap, HashSet,
//    LinkedList, VecDeque};
use std::collections::{LinkedList, VecDeque};
use std::vec::Vec;

fn main() {
    println!("--- Rust Collections");
    // Sequences
    vec_1();
    vec_deque_1();
    linked_list_1();
    // Maps
    hash_map_1();
    b_tree_map_1();
    // Sets
    hash_set_1();
    b_tree_set_1();
    // Misc
    binary_heap_1();
}

fn vec_1() {
    println!("--- Vec 1");
    let mut v = Vec::new();
    for i in 1..10 {
        v.push(i);
    }
    println!("v = {:?}", v);
    assert_eq!(v.pop(), Some(9));
    assert_eq!(v.pop(), Some(8));
    assert_eq!(v.pop(), Some(7));
    println!("v = {:?}", v);

    v.extend([5, 4, 3, 2, 1].iter().copied());
    println!("v = {:?}", v);
}

fn vec_deque_1() {
    println!("--- Vec Deque 1");
    let mut q = VecDeque::new();
    q.push_front(2);
    q.push_front(1);
    q.push_back(3);
    q.push_back(4);
    println!("q = {:?}", q);
    let _ = q.pop_front();
    q.push_back(5);
    println!("q = {:?}", q);
}

fn linked_list_1() {
    println!("--- Linked List 1");
    let mut ll = LinkedList::new();
    ll.push_front(2);
    ll.push_front(1);
    ll.push_back(3);
    ll.push_back(4);
    println!("ll = {:?}", ll);

    ll.clear();
    assert_eq!(ll.len(), 0);
    assert!(ll.is_empty());

    for i in 1..5 {
        ll.push_back(i);
    }
    println!("ll = {:?}", ll);

    for x in ll.iter_mut() {
        *x *= *x;
    }
    println!("ll = {:?}", ll);
}

/// See [https://doc.rust-lang.org/std/collections/hash_map/struct.HashMap.html]
fn hash_map_1() {
    println!("--- Hash Map 1");
}

fn b_tree_map_1() {
    println!("--- B-Tree Map 1");
}

fn hash_set_1() {
    println!("--- Hash Set 1");
}

fn b_tree_set_1() {
    println!("--- B-Tree Set 1");
}

fn binary_heap_1() {
    println!("--- Binary Heap 1");
}
