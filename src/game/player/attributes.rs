use std::{
    array,
    ops::{Add, Index, IndexMut},
};

use strum::{EnumCount, EnumIter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount, EnumIter, Hash)]
pub enum Attr {
    /// Attack damage.
    Damage,
    /// Attack speed.
    Speed,
    /// Crit chance.
    Crit,
    /// Health points.
    Health,
    /// Health regen.
    Regen,
    /// Lifesteal on attack chance.
    Lifesteal,
    /// Attack range.
    Range,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Attrs([i32; Attr::COUNT]);

impl Attrs {
    pub fn new() -> Self {
        Self(Default::default())
    }
    pub fn from_attr(stat: Attr, value: i32) -> Self {
        let mut res = Self::new();
        res[stat] = value;
        res
    }
}

impl Index<Attr> for Attrs {
    type Output = i32;

    fn index(&self, index: Attr) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl IndexMut<Attr> for Attrs {
    fn index_mut(&mut self, index: Attr) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

impl Add<Self> for Attrs {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(array::from_fn(|idx| self.0[idx] + rhs.0[idx]))
    }
}
