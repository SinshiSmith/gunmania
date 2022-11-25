use crate::player::PlayerPlugin;
use animation::{Animate, CustomAnimationPlugin};
use bevy::{asset::LoadState, prelude::*, sprite::collide_aabb::collide};
use bevy_inspector_egui::Inspectable;
use inspector::CustomInspectorPlugin;

mod animation;
mod inspector;
mod player;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    Setup,
    Finished,
}

#[derive(Component)]
pub struct Enemy;

#[derive(Component, Inspectable)]
pub struct CombatStats {
    health: isize,
    damage: isize,
}

fn main() {
    App::new()
        .init_resource::<RpgSpriteHandles>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(CustomInspectorPlugin)
        .add_system(bevy::window::close_on_esc)
        .add_plugin(PlayerPlugin)
        .add_plugin(CustomAnimationPlugin)
        .add_state(AppState::Setup)
        .add_system_set(SystemSet::on_enter(AppState::Setup).with_system(load_textures))
        .add_system_set(SystemSet::on_update(AppState::Setup).with_system(check_textures))
        .add_system_set(SystemSet::on_enter(AppState::Finished).with_system(setup))
        .add_system(collision_detection)
        .add_system(auto_move)
        .add_system(destroy_zombie)
        .run();
}

#[derive(Resource, Default)]
struct RpgSpriteHandles {
    handles: Vec<HandleUntyped>,
}

fn load_textures(mut rpg_sprite_handles: ResMut<RpgSpriteHandles>, asset_server: Res<AssetServer>) {
    rpg_sprite_handles.handles = asset_server
        .load_folder("textures/rpg/chars/zombie")
        .unwrap();
}

fn check_textures(
    mut state: ResMut<State<AppState>>,
    rpg_sprite_handles: ResMut<RpgSpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    if let LoadState::Loaded =
        asset_server.get_group_load_state(rpg_sprite_handles.handles.iter().map(|handle| handle.id))
    {
        state.set(AppState::Finished).unwrap();
    }
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn setup(
    mut commands: Commands,
    rpg_sprite_handles: Res<RpgSpriteHandles>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
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
    let zombie_handle = asset_server.get_handle("textures/rpg/chars/zombie/Walk1.png");
    let zombie_index = texture_atlas.get_texture_index(&zombie_handle).unwrap();
    let atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        Speed(-10.),
        SpriteSheetBundle {
            transform: Transform {
                translation: Vec3::new(150.0, 0.0, 0.0),
                scale: Vec3::splat(0.3),
                ..default()
            },
            sprite: TextureAtlasSprite {
                index: zombie_index,
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

#[derive(Component)]
struct Speed(f32);

fn auto_move(mut query: Query<(&mut Transform, &Speed)>, time: Res<Time>) {
    for (mut transform, speed) in &mut query {
        transform.translation.x += speed.0 * time.delta_seconds();
    }
}

fn collision_detection(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &CombatStats), Without<Enemy>>,
    mut target: Query<(&Transform, &mut CombatStats), With<Enemy>>,
) {
    for (ent, transform, combat_stats) in &query {
        for (target_transform, mut target_combat_stats) in &mut target {
            let collision = collide(
                transform.translation,
                Vec2::splat(5.0),
                target_transform.translation,
                Vec2::splat(32.0),
            );
            if collision.is_some() {
                info!("hit hit hit hit");
                commands.entity(ent).despawn();
                target_combat_stats.health -= combat_stats.damage;
            }
        }
    }
}

fn destroy_zombie(mut commands: Commands, zombie: Query<(Entity, &CombatStats), With<Enemy>>) {
    for (entity, combat_stats) in &zombie {
        if combat_stats.health <= 0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}
