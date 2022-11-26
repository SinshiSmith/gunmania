use bevy::prelude::*;

use crate::{animation::Animate, AnimationTimer, AppState, CombatStats, RpgSpriteHandles, Speed};

pub struct ZombiePlugin;

#[derive(Component)]
pub struct Enemy;

impl Plugin for ZombiePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Finished).with_system(spawn_zombie_on_enter),
        )
        .add_system(destroy_zombie);
    }
}

fn spawn_zombie_on_enter(
    rpg_sprite_handles: Res<RpgSpriteHandles>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Image>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    for handle in &rpg_sprite_handles.handles {
        let handle = handle.typed_weak();
        let Some(texture) = textures.get(&handle) else {
        warn!("{:?} did not resolve to an `Image` asset.", asset_server.get_handle_path(handle));
        continue;
    };

        texture_atlas_builder.add_texture(handle, texture);
    }
    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
    let atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn((
        Speed(-10.),
        SpriteSheetBundle {
            transform: Transform {
                translation: Vec3::new(150.0, 0.0, 0.0),
                scale: Vec3::splat(0.3),
                ..default()
            },
            sprite: TextureAtlasSprite {
                flip_x: true,
                ..default()
            },
            texture_atlas: atlas_handle,

            ..default()
        },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Animate(0, 5),
        Enemy,
        CombatStats {
            health: 100,
            damage: 1,
        },
    ));
}

fn destroy_zombie(mut commands: Commands, zombie: Query<(Entity, &CombatStats), With<Enemy>>) {
    for (entity, combat_stats) in &zombie {
        if combat_stats.health <= 0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}
