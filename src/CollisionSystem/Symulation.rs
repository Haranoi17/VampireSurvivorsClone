use std::vec;

use crate::MathUtilities::{Position, Vector};

use super::{
    Circle, Collidable, Collider, CollisionInfo, CollisionMask, CollisionShape, Rectangle,
};

#[derive(Debug, Clone, Copy)]
pub struct Pair<T> {
    pub first: T,
    pub second: T,
}

pub struct SymulationCollisionInfo {
    pub collided_objects_indices: Pair<usize>,
    pub info: CollisionInfo,
    pub masks: Pair<CollisionMask>,
}

pub struct WordSymulation {
    pub symulation_collisions_info: Vec<SymulationCollisionInfo>,
}

impl WordSymulation {
    pub fn new() -> Self {
        Self {
            symulation_collisions_info: vec![],
        }
    }

    pub fn collision_detection(&mut self, collidables: &Vec<&mut dyn Collidable>) {
        for i in 0..collidables.len() - 1 {
            for j in i + 1..collidables.len() {
                let first = collidables.get(i).unwrap();
                let second = collidables.get(j).unwrap();
                let did_collide = Collider::collide(first.get_collider(), second.get_collider());

                match did_collide {
                    Some(collision_info) => {
                        let collided_objects_indices = Pair {
                            first: i,
                            second: j,
                        };

                        let collided_objects_masks = Pair {
                            first: first.get_mask(),
                            second: second.get_mask(),
                        };

                        let symulation_collision_info = SymulationCollisionInfo {
                            collided_objects_indices: collided_objects_indices,
                            info: collision_info,
                            masks: collided_objects_masks,
                        };

                        self.symulation_collisions_info
                            .push(symulation_collision_info);
                    }
                    None => { /* do nothing */ }
                }
            }
        }
    }

    pub fn clear_collisions(&mut self) {
        self.symulation_collisions_info.clear();
    }

    pub fn react_to_collisionss(&mut self, collidables: &mut Vec<&mut dyn Collidable>) {
        WordSymulation::react_to_collisions(
            collidables,
            &self.symulation_collisions_info
        );
    }

    fn react_to_collisions(
        collidables: &mut Vec<&mut dyn Collidable>,
        symulation_collisions_info: &Vec<SymulationCollisionInfo>
    ) {
        for symulation_collision_info in symulation_collisions_info {
            let collision_pair = symulation_collision_info.collided_objects_indices;
            let first = collidables.get_mut(collision_pair.first).unwrap();
            
            first.react_to_collision(symulation_collision_info.info, symulation_collision_info.masks.first);
        }

        for symulation_collision_info in symulation_collisions_info {
            let collision_pair = symulation_collision_info.collided_objects_indices;
            let second = collidables.get_mut(collision_pair.second).unwrap();
            
            second.react_to_collision(symulation_collision_info.info.symetrical(), symulation_collision_info.masks.second);
        }

    }
}
