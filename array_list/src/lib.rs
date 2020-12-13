#![warn(missing_docs)]
//! array_list implements List and Queue interfaces with backing array.

/// Implementation for List optimized to realize efficient addition/removal from .
pub mod array_deque;
/// Implementation for List optimized to realize FIFO queue.
pub mod array_queue;
/// Implementation for List optimized to realize double-ended queue.
pub mod array_stack;
