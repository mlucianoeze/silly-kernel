#![cfg_attr(not(test), no_std)]

// #[cfg(not(test))]
// extern crate alloc;

pub mod devtree;
pub mod node;
pub mod property;

pub use devtree::DevTree;
pub use node::DevTreeNode;
pub use property::Property;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let sum: i32 = vec![1, 2, 3].iter().sum();
        assert_eq!(sum, 6);
    }
}