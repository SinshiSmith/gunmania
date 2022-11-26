use crate::player::PlayerPlugin;
use animation::CustomAnimationPlugin;
use bevy::{asset::LoadState, prelude::*, sprite::collide_aabb::collide};
use bevy_inspector_egui::Inspectable;
use inspector::CustomInspectorPlugin;
use zombie::{Enemy, ZombiePlugin};

mod animation;
mod inspector;
mod player;
mod zombie;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    Setup,
    Finished,
}

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
        .add_plugin(ZombiePlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(CustomAnimationPlugin)
        .add_system(bevy::window::close_on_esc)
        .add_state(AppState::Setup)
        .add_system_set(SystemSet::on_enter(AppState::Setup).with_system(load_textures))
        .add_system_set(SystemSet::on_update(AppState::Setup).with_system(check_textures))
        .add_system_set(SystemSet::on_enter(AppState::Finished).with_system(setup_camera))
        .add_system(collision_detection)
        .add_system(auto_move)
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

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
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
                commands.entity(ent).despawn();
                target_combat_stats.health -= combat_stats.damage;
            }
        }
    }
}
