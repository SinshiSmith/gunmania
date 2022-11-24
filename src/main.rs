use std::time::Duration;

use bevy::{asset::LoadState, prelude::*, sprite::MaterialMesh2dBundle};

fn main() {
    App::new()
        .init_resource::<RpgSpriteHandles>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_state(AppState::Setup)
        .add_system_set(SystemSet::on_enter(AppState::Setup).with_system(load_textures))
        .add_system_set(SystemSet::on_update(AppState::Setup).with_system(check_textures))
        .add_system_set(SystemSet::on_update(AppState::Setup).with_system(player_setup))
        .add_system_set(SystemSet::on_enter(AppState::Finished).with_system(setup))
        .add_system(animate_sprite)
        .add_system(flip)
        .add_system(shoot)
        .add_system(auto_move)
        .run();
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    Setup,
    Finished,
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

fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}

#[derive(Component)]
struct Player;

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
    let zombie_handle = asset_server.get_handle("textures/rpg/chars/zombie/Idle1.png");
    let zombie_index = texture_atlas.get_texture_index(&zombie_handle).unwrap();
    let atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        Speed(-0.2),
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
        AnimationTimer(Timer::from_seconds(0.16, TimerMode::Repeating)),
    ));
}

fn flip(mut query: Query<&mut TextureAtlasSprite, With<Player>>, buttons: Res<Input<KeyCode>>) {
    if buttons.just_pressed(KeyCode::F) {
        for mut sprite in &mut query {
            sprite.flip_x = !sprite.flip_x;
        }
    }
}

#[derive(Component)]
struct Speed(f32);

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

fn auto_move(mut query: Query<(&mut Transform, &Speed)>) {
    let mut timer = Timer::from_seconds(1.0, TimerMode::Repeating);

    for (mut transform, speed) in &mut query {
        timer.tick(Duration::from_secs_f32(1.0));
        if timer.just_finished() {
            transform.translation.x += speed.0;
        }
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
