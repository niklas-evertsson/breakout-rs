use bevy::{core::FixedTimestep, prelude::*};

mod audio;
use audio::*;
mod ball;
use ball::*;
mod brick;
use brick::*;
mod collision_system;
use collision_system::*;
mod components;
use components::*;
mod events;
use events::*;
mod paddle;
use paddle::*;
mod score;
use score::*;
mod velocity_system;
use velocity_system::*;
mod walls;
use walls::*;

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const TIME_STEP: f32 = 1.0 / 60.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(Score::new())
        .add_startup_system(setup)
        .add_event::<CollisionEvent>()
        .add_system(bevy::window::exit_on_window_close_system)
        .add_system(update_score_ui)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(collision_system)
                .with_system(move_paddle.before(collision_system))
                .with_system(apply_velocity.before(collision_system))
                .with_system(play_collision_sound.after(collision_system)),
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    let ball_collision_sound = asset_server.load(BALL_COLLISION_SOUND);
    commands.insert_resource(CollisionSound(ball_collision_sound));

    let paddle_y = BOTTOM_WALL + PADDLE_HEIGHT;
    commands
        .spawn()
        .insert(Paddle)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, paddle_y, 0.0),
                scale: PADDLE_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: PADDLE_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(Collider);

    commands
        .spawn()
        .insert(Ball)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                scale: BALL_SIZE,
                translation: BALL_START_POSITION,
                ..default()
            },
            sprite: Sprite {
                color: BALL_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(Velocity(BALL_START_DIRECTION.normalize() * BALL_SPEED));

    commands.spawn_bundle(TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: SCORE_TEXT.to_string(),
                    style: TextStyle {
                        font: asset_server.load(SCORE_TEXT_FONT),
                        font_size: SCORE_FONT_SIZE,
                        color: TEXT_COLOR,
                    },
                },
                TextSection {
                    value: String::new(),
                    style: TextStyle {
                        font: asset_server.load(SCORE_NUMBER_FONT),
                        font_size: SCORE_FONT_SIZE,
                        color: SCORE_COLOR,
                    },
                },
            ],
            ..default()
        },
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: SCORE_TEXT_PADDING,
                left: SCORE_TEXT_PADDING,
                ..default()
            },
            ..default()
        },
        ..default()
    });

    commands.spawn_bundle(WallBundle::new(WallLocation::Left));
    commands.spawn_bundle(WallBundle::new(WallLocation::Right));
    commands.spawn_bundle(WallBundle::new(WallLocation::Bottom));
    commands.spawn_bundle(WallBundle::new(WallLocation::Top));

    assert!(BRICK_SIZE.x > 0.0);
    assert!(BRICK_SIZE.y > 0.0);

    let total_width_of_bricks = (RIGHT_WALL - LEFT_WALL) - 2. * BRICKS_OUTER_PADDING;
    let bottom_edge_of_bricks = paddle_y + BRICKS_BOTTOM;
    let total_height_of_bricks = TOP_WALL - bottom_edge_of_bricks - BRICKS_TOP;

    assert!(total_width_of_bricks > 0.0);
    assert!(total_height_of_bricks > 0.0);

    let n_columns = (total_width_of_bricks / (BRICK_SIZE.x + BRICK_INNER_PADDING)).floor() as usize;
    let n_rows = (total_height_of_bricks / (BRICK_SIZE.y + BRICK_INNER_PADDING)).floor() as usize;
    let n_vertical_gaps = n_columns - 1;

    let center_of_bricks = (LEFT_WALL + RIGHT_WALL) / 2.0;
    let left_edge_of_bricks = center_of_bricks
        - (n_columns as f32 / 2.0 * BRICK_SIZE.x)
        - n_vertical_gaps as f32 / 2.0 * BRICK_INNER_PADDING;

    let offset_x = left_edge_of_bricks + BRICK_SIZE.x / 2.0;
    let offset_y = bottom_edge_of_bricks + BRICK_SIZE.y / 2.0;

    for row in 0..n_rows {
        for column in 0..n_columns {
            let brick_position = Vec2::new(
                offset_x + column as f32 * (BRICK_SIZE.x + BRICK_INNER_PADDING),
                offset_y + row as f32 * (BRICK_SIZE.y + BRICK_INNER_PADDING),
            );

            commands
                .spawn()
                .insert(Brick)
                .insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: BRICK_COLOR,
                        ..default()
                    },
                    transform: Transform {
                        translation: brick_position.extend(0.0),
                        scale: Vec3::new(BRICK_SIZE.x, BRICK_SIZE.y, 1.0),
                        ..default()
                    },
                    ..default()
                })
                .insert(Collider);
        }
    }
}
