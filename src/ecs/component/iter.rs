use crate::ecs::component::Entity;

#[derive(Debug, Clone)]
pub struct Values<'a, T>(pub(super) std::slice::Iter<'a, T>);

impl<'a, T> Iterator for Values<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.0.nth(n)
    }
}

#[derive(Debug)]
pub struct ValuesMut<'a, T>(pub(super) std::slice::IterMut<'a, T>);

impl<'a, T> Iterator for ValuesMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.0.nth(n)
    }
}

#[derive(Debug, Clone)]
pub struct Entities<'a>(pub(super) std::slice::Iter<'a, Entity>);

impl<'a> Iterator for Entities<'a> {
    type Item = Entity;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().copied()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.0.nth(n).copied()
    }
}

#[derive(Debug, Clone)]
pub struct Iter<'a, T> {
    pub(super) entity_iter: Entities<'a>,
    pub(super) value_iter: Values<'a, T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = (Entity, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        Some((self.entity_iter.next()?, self.value_iter.next()?))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.entity_iter.size_hint()
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        Some((self.entity_iter.nth(n)?, self.value_iter.nth(n)?))
    }
}

#[derive(Debug)]
pub struct IterMut<'a, T> {
    pub(super) entity_iter: Entities<'a>,
    pub(super) value_iter: ValuesMut<'a, T>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = (Entity, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        Some((self.entity_iter.next()?, self.value_iter.next()?))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.entity_iter.size_hint()
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        Some((self.entity_iter.nth(n)?, self.value_iter.nth(n)?))
    }
}
