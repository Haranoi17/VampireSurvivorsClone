use super::{Animation, AnimationPlayer};

#[test]
fn test_loading_animation_from_directory() {
    let test_animation_directory = "resources/Animations/Player/idle";
    let animation = Animation::new(test_animation_directory);
    println!("{:?}", animation.frames);
}

#[test]
fn test_loading_animations_for_animation_player() {
    let mut animation_player = AnimationPlayer::new();
    let animations_path = String::from("resources/Animations/Player");
    animation_player.initialize(animations_path);
}