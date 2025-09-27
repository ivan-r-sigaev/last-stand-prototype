use std::{collections::VecDeque, ops::Range};

use macroquad::math::{Circle, Rect, Vec2};
use nonmax::NonMaxU16;

use crate::ecs::{
    component::{Component, ComponetPool},
    entity::Entity,
};

/// A mask determining the layer within collision system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct CollisionMask(pub u8);

impl CollisionMask {
    /// Returns `true` if the two collision masks have
    /// overlapping layers.
    pub fn overlaps(self, other: Self) -> bool {
        (self.0 & other.0) != 0
    }
}

/// A collider component.
#[derive(Debug, Clone, Copy)]
pub struct Collider {
    /// Shape and position of the collider.
    pub shape: Circle,
    /// The layers inside of which this collider can be detected.
    pub monitorable: CollisionMask,
    /// The layers that the collider will scan to detect collisions.
    pub monitoring: CollisionMask,
}

impl Component for Collider {}

/// Determines the shape and precision of the [`CollisionGrid`].
#[derive(Debug, Clone, Copy, Default)]
pub struct CollisionGridParams {
    pub bounding_rect: Rect,
    pub resolution: (usize, usize),
}

/// Data structure that allows to efficiently detect collisions
/// for the [`Collider`] components.
#[derive(Debug, Clone, Default)]
pub struct CollisionGrid {
    params: CollisionGridParams,
    entities: Vec<Entity>,
    cell_ranges: Vec<Range<u16>>,
}

impl CollisionGrid {
    /// Constructs and empty collision grid.
    pub fn new() -> Self {
        Default::default()
    }
    /// Returns the params from the last time the grid was updated.
    pub fn params(&self) -> &CollisionGridParams {
        &self.params
    }
    /// Returns the entities that might have collisions in the cell \[`x`, `y`\].
    ///
    /// # Panics
    /// Panics if the `x` or `y` exceeds the current resolution.
    pub fn cell(&self, x: usize, y: usize) -> &[Entity] {
        let range = self.cell_ranges[self.cell_index(x, y)].clone();
        let range = Range {
            start: range.start as usize,
            end: range.end as usize,
        };
        &self.entities[range]
    }
    /// Rebuilds the collision grid with the new parameters.
    pub fn update(&mut self, colliders: &ComponetPool<Collider>, params: CollisionGridParams) {
        self.params = params;
        let cell_count = params.resolution.0.strict_mul(params.resolution.1);

        // Pass 1: count cells.
        self.cell_ranges.clear();
        self.cell_ranges.resize(cell_count, Range::default());
        for (entity, &collider) in colliders.iter() {
            for (x, y) in self.overlapping_cells(collider) {
                let index = self.cell_index(x, y);
                self.cell_ranges[index].end += 1;
            }
        }
        // `self.cell_ranges[i]` is `0..count`, where `count` is number of entities in the cell

        // Pass 2: build ranges.
        let mut total = 0;
        for y in 0..params.resolution.0 {
            for x in 0..params.resolution.1 {
                let index = self.cell_index(x, y);
                let range = &mut self.cell_ranges[index];
                let count = range.end as usize;
                range.start = total as u16;
                range.end = total as u16;
                total += count;
            }
        }
        // `self.cell_ranges[i]` is `index..index` where `index` is the future begining of the range.
        // `total` is the total number of entities.

        // Pass 3: fill entities.
        self.entities.clear();
        self.entities.reserve(total);
        for (entity, &collider) in colliders.iter() {
            for (x, y) in self.overlapping_cells(collider) {
                let index = self.cell_index(x, y);
                let range = &mut self.cell_ranges[index];
                self.entities.spare_capacity_mut()[range.end as usize].write(entity);
                range.end += 1;
            }
        }
        // `self.cell_ranges[i]` is `start..end` where `start` and `end` indicate the positions
        // of the corresponding entities within the spare capacity of `self.entities`.

        // SAFETY: This is safe since `total` is the total number of entities
        // and we filled in the entities into the spare capacity in "pass 3".
        unsafe {
            self.entities.set_len(total);
        }
    }
    /// Returns the other entity from the first collision pair with the supplied entity.
    ///
    /// # Panics
    /// Panics if the supplied entity does not have a [`Collider`] component.
    pub fn any_collision(
        &self,
        colliders: &ComponetPool<Collider>,
        entity: Entity,
    ) -> Option<Entity> {
        let collider = colliders
            .get(entity)
            .copied()
            .expect("Entity does not have a collider!");
        for (x, y) in self.overlapping_cells(collider) {
            for &entity in self.cell(x, y) {
                let other = colliders.get(entity).unwrap();
                if !collider.monitoring.overlaps(other.monitorable) {
                    continue;
                }
                if !collider.shape.overlaps(&other.shape) {
                    continue;
                }
                return Some(entity);
            }
        }
        None
    }
    /// Returns the index in the cell array for the specified point.
    ///
    /// # Panics
    /// Panics if the point is out of bounds.
    fn cell_index(&self, x: usize, y: usize) -> usize {
        let res = self.params.resolution;
        assert!(x < res.0 && y < res.1, "The cell index is out of bounds!");
        y * res.0 + x
    }
    /// Returns an iterator of all cell indices that overlap with the provided collider.
    fn overlapping_cells(
        &self,
        collider: Collider,
    ) -> impl Iterator<Item = (usize, usize)> + 'static {
        let min = Vec2::new(
            collider.shape.x - collider.shape.r,
            collider.shape.y - collider.shape.r,
        );
        let max = Vec2::new(
            collider.shape.x + collider.shape.r,
            collider.shape.y + collider.shape.r,
        );
        let res = Vec2::new(
            self.params.resolution.0 as f32,
            self.params.resolution.1 as f32,
        );
        let origin = self.params.bounding_rect.point();
        let local_min = ((min - origin) / res).max(Vec2::ZERO).floor();
        let local_max = ((max - origin) / res).min(res).ceil();
        let range_x = Range {
            start: local_min.x as usize,
            end: local_max.x as usize,
        };
        let range_y = Range {
            start: local_min.y as usize,
            end: local_max.y as usize,
        };
        range_y.flat_map(move |y| range_x.clone().map(move |x| (x, y)))
    }
}
