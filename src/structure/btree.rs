// first program analyses the arrays, checking each value and position's relation to one another
// 1 -> 2, 3
// 2 -> 1, 2, 3, 4
// 3 -> 1, 4, 5
// create index path only to those with direct relations

pub mod btree {
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};

    #[derive(Debug, Clone)]
    pub struct BTree {
        pub word_count: i32,
        pub static words: Vec<Vec<i32>> = vec![
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 5],
            vec![1, 2, 2, 4],
            vec![1, 3, 1, 1],
            vec![2, 1, 2, 4]
            ],
    }

    impl BTree {
        pub fn analyze_rel() {
            for i in &Vec<Vec<i32>> {
                for j in &Vec<i32> {
                    [i][j] 
                }
            }
        }
    }
}