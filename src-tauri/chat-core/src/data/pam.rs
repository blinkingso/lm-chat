use super::object::Object;
use bitflags::bitflags;
use std::collections::HashMap;
use std::fmt::Debug;

bitflags! {
    pub struct BitMask: u64 {
        const READ = 0b0000_0001;
        const WRITE = 0b0000_0010;
        const MANAGE = 0b0000_0100;
        const DELETE = 0b0000_1000;
        const CREATE = 0b0001_0000;
    }
}
