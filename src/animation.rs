use bevy::prelude::*;

use crate::AnimationTimer;
pub struct CustomAnimationPlugin;

impl Plugin for CustomAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(loop_animation);
    }
}

#[derive(Component)]
pub struct Animate(pub usize, pub usize);

fn loop_animation(
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlasSprite, &Animate)>,
    time: Res<Time>,
) {
    for (mut timer, mut sprite, Animate(start, finish)) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            if sprite.index == *finish {
                sprite.index = *start;
            } else {
                sprite.index += 1;
            }
        }
    }
}
