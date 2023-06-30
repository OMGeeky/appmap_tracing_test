use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct EventId(u32);
impl Deref for EventId {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for EventId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self
    }
}

impl From<u32> for EventId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct ObjectId(u64);
impl Deref for ObjectId {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for ObjectId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self
    }
}
