use std::{vec, };

use crate::MathUtilities::{Position, Vector};

use super::{Circle, Collidable, Collider, CollisionInfo, CollisionShape, CollisionMask};


#[derive(Debug)]
pub struct Pair<T>{
    pub first: T,
    pub second: T,
}

pub struct CollisionInfoWithMasks{
    pub info: CollisionInfo,
    pub masks: Pair<CollisionMask>,
}

pub struct WordSymulation {
    pub collided_pairs_indices: Vec<Pair<usize>>,
    pub collision_info: Vec<CollisionInfoWithMasks>,
}

impl WordSymulation {
    pub fn new()->Self{
        Self { collided_pairs_indices: vec![], collision_info: vec![] }
    }
    
    pub fn collision_detection(&mut self, collidables: &Vec<&mut dyn Collidable>) {
        for i in 0..collidables.len() - 1 {
            for j in i + 1..collidables.len() {
                let first = collidables.get(i).unwrap();
                let second = collidables.get(j).unwrap();
                let did_collide = Collider::collide(first.get_collider(), second.get_collider());

                if did_collide{
                    self.collided_pairs_indices.push(Pair{first: i, second: j});
                }
            }
        }
    }
    
    pub fn gather_collision_info(&mut self, collidables: &Vec<&mut dyn Collidable>){
        for collision_pair in &mut self.collided_pairs_indices{
            let first = collidables.get(collision_pair.first).unwrap();
            let second = collidables.get(collision_pair.second).unwrap();
            
            let new_info = WordSymulation::generate_collision_info(first.get_collider(), second.get_collider());
            let info_with_masks = CollisionInfoWithMasks{info: new_info, masks: Pair{first: first.get_mask(), second: second.get_mask()}};
            self.collision_info.push(info_with_masks); 
        }
    }

    pub fn clear_collisions(&mut self){
        self.collided_pairs_indices.clear();
        self.collision_info.clear();
    }

    pub fn react_to_collisionss(&mut self, collidables: &mut Vec<&mut dyn Collidable>){
        WordSymulation::react_to_collisions(collidables, &self.collided_pairs_indices, &self.collision_info);
    }

    fn react_to_collisions(collidables: &mut Vec<&mut dyn Collidable>, collided_pairs: &Vec<Pair<usize>>, collision_info_with_masks: &Vec<CollisionInfoWithMasks> ){
        for index in 0..collided_pairs.len(){
            let collision_pair = collided_pairs.get(index).unwrap();
            let first = collidables.get_mut(collision_pair.first).unwrap();

            let collision_info = collision_info_with_masks.get(index).unwrap().info;
            let masks = &collision_info_with_masks.get(index).unwrap().masks;
            first.react_to_collision(collision_info, masks.second);
        }

        for index in 0..collided_pairs.len(){
            let collision_pair = collided_pairs.get(index).unwrap();
            let second = collidables.get_mut(collision_pair.second).unwrap();

            let collision_info = collision_info_with_masks.get(index).unwrap().info;
            let masks = &collision_info_with_masks.get(index).unwrap().masks;
            second.react_to_collision(collision_info, masks.first);
        }
    }

    fn generate_collision_info(first: Collider, second: Collider) -> CollisionInfo {
        match (first.shape, second.shape) {
            (CollisionShape::Circle(first_circle), CollisionShape::Circle(second_circle)) => {
                WordSymulation::circle_circle_collision_info(
                    first_circle,
                    second_circle,
                    first.position,
                    second.position,
                )
            }

            (CollisionShape::Circle(circle), CollisionShape::Rectangle(rectangle)) => {
                let vector_between_positions = first.position - second.position;
                let outer_rectangle_circle = Collider::outer_rectangle_circle(rectangle);

                WordSymulation::circle_circle_collision_info(
                    circle,
                    outer_rectangle_circle,
                    first.position,
                    second.position,
                )
            }
            (CollisionShape::Rectangle(rectangle), CollisionShape::Circle(circle)) => {
                let vector_between_positions = first.position - second.position;
                let outer_rectangle_circle = Collider::outer_rectangle_circle(rectangle);

                WordSymulation::circle_circle_collision_info(
                    circle,
                    outer_rectangle_circle,
                    second.position,
                    first.position,
                )
            }
            (
                CollisionShape::Rectangle(first_rectangle),
                CollisionShape::Rectangle(second_rectangle),
            ) => {
                let first_outer_rectangle_circle =
                    Collider::outer_rectangle_circle(first_rectangle);
                let second_outer_rectangle_circle =
                    Collider::outer_rectangle_circle(second_rectangle);

                WordSymulation::circle_circle_collision_info(
                    first_outer_rectangle_circle,
                    second_outer_rectangle_circle,
                    first.position,
                    second.position,
                )
            }
        }
    }

    fn circle_circle_collision_info(
        first: Circle,
        second: Circle,
        first_position: Position,
        second_position: Position,
    ) -> CollisionInfo {
        let vector_between_positions = first_position - second_position;
        let distance = vector_between_positions.length();
        let sum_of_radii = first.radius + second.radius;

        let collision_direction = vector_between_positions.normal().unwrap_or(Vector::new(10.0, 0.0));

        let collision_depth_value = sum_of_radii - distance;
        let collision_depth = collision_direction * collision_depth_value;

        CollisionInfo::new(collision_direction, collision_depth)
    }
}
