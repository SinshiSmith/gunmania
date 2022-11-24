use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{AnimationTimer, AppState, Speed};
pub struct PlayerPlugin;

#[derive(Component)]
struct Player;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(AppState::Setup).with_system(player_setup))
            .add_system(shoot)
            .add_system(flip);
    }
}

fn player_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("textures/rpg/chars/player/player.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 6, 5, None, None);

    // let test = texture_atlas.textures.split_off(2);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn((
        Player,
        SpriteSheetBundle {
            transform: Transform {
                translation: Vec3::new(-100.0, 0.0, 0.0),
                scale: Vec3::splat(3.0),
                ..default()
            },
            texture_atlas: texture_atlas_handle,
            ..default()
        },
        AnimationTimer(Timer::from_seconds(0.16, TimerMode::Repeating)),
    ));
}

fn shoot(
    query: Query<(&Transform, &TextureAtlasSprite), With<Player>>,
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        for (transform, sprite) in &query {
            let direction = if sprite.flip_x { -1. } else { 1. };
            let bullet_x = transform.translation.x + (50.0 * direction);
            commands.spawn((
                Speed(20.0),
                MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                    transform: Transform {
                        translation: Vec3::new(bullet_x, 0.0, 0.0),
                        scale: Vec3::splat(20.),
                        ..default()
                    },
                    material: materials.add(ColorMaterial::from(Color::PURPLE)),
                    ..default()
                },
            ));
        }
    }
}

fn flip(mut query: Query<&mut TextureAtlasSprite, With<Player>>, buttons: Res<Input<KeyCode>>) {
    if buttons.just_pressed(KeyCode::F) {
        for mut sprite in &mut query {
            sprite.flip_x = !sprite.flip_x;
        }
    }
}
