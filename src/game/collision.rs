use std::{collections::VecDeque, ops::Range};

use macroquad::math::{Circle, Rect, Vec2};
use nonmax::NonMaxU16;

use crate::{
    ecs::{
        component::{Component, ComponentPool},
        entity::Entity,
    },
    game::transform::Transform,
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

#[derive(Debug, Clone, Copy)]
pub enum Shape {
    Circle { radius: f32 },
}

/// A collider component.
#[derive(Debug, Clone, Copy)]
pub struct Collider {
    /// Shape and position of the collider.
    pub shape: Shape,
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
    pub fn update(
        &mut self,
        transforms: &ComponentPool<Transform>,
        colliders: &ComponentPool<Collider>,
        params: CollisionGridParams,
    ) {
        self.params = params;
        let cell_count = params.resolution.0.strict_mul(params.resolution.1);

        // Pass 1: count cells.
        self.cell_ranges.clear();
        self.cell_ranges.resize(cell_count, Range::default());
        for (entity, &collider) in colliders.iter() {
            let transform = transforms
                .get(entity)
                .expect("All colliders must have a Transform component!");
            for (x, y) in self.overlapping_cells(transform.position, collider.shape) {
                let index = self.cell_index(x, y);
                self.cell_ranges[index].end += 1;
            }
        }
        // `self.cell_ranges[i]` is `0..count`, where `count` is number of entities in the cell

        // Build ranges.
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

        // Pass 2: fill entities.
        self.entities.clear();
        self.entities.reserve(total);
        for (entity, &collider) in colliders.iter() {
            let transform = transforms
                .get(entity)
                .expect("All colliders must have a Transform component!");
            for (x, y) in self.overlapping_cells(transform.position, collider.shape) {
                let index = self.cell_index(x, y);
                let range = &mut self.cell_ranges[index];
                self.entities.spare_capacity_mut()[range.end as usize].write(entity);
                range.end += 1;
            }
        }
        // `self.cell_ranges[i]` is `start..end` where `start` and `end` indicate the positions
        // of the corresponding entities within the spare capacity of `self.entities`.

        // SAFETY: This is safe since `total` is the total number of entities
        // and we filled in the entities into the spare capacity in "pass 2".
        unsafe {
            self.entities.set_len(total);
        }
    }
    pub fn collisions(
        &self,
        transforms: &ComponentPool<Transform>,
        colliders: &ComponentPool<Collider>,
        entity: Entity,
    ) -> impl Iterator<Item = Entity> {
        let collider = colliders
            .get(entity)
            .copied()
            .expect("Entity must have a collider!");
        let transform = transforms.get(entity).copied().unwrap();
        let circle = match collider.shape {
            Shape::Circle { radius } => Circle {
                x: transform.position.x,
                y: transform.position.y,
                r: radius,
            },
        };
        self.overlapping_cells(transform.position, collider.shape)
            .flat_map(|(x, y)| self.cell(x, y).iter().copied())
            .filter(move |entity| {
                let other_collider = colliders.get(*entity).unwrap();
                let other_transform = transforms.get(*entity).unwrap();
                if !collider.monitoring.overlaps(other_collider.monitorable) {
                    return false;
                }
                let other_circle = match collider.shape {
                    Shape::Circle { radius } => Circle {
                        x: other_transform.position.x,
                        y: other_transform.position.y,
                        r: radius,
                    },
                };
                if !other_circle.overlaps(&circle.clone()) {
                    return false;
                }
                true
            })
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
        position: Vec2,
        shape: Shape,
    ) -> impl Iterator<Item = (usize, usize)> + 'static {
        let Shape::Circle { radius } = shape;
        let min = position - Vec2::splat(radius);
        let max = position + Vec2::splat(radius);
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
