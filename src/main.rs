extern crate binance;
use binance::api::*;
use binance::market::*;
use binance::model::*;

use bevy::prelude::*;

mod camera_dragging;
use camera_dragging::{camera_dragging_system, CameraDraggingState};

struct TickTimer(Timer);

fn main() {
    let mut window_desc = WindowDescriptor::default();
    window_desc.width = 1600;
    window_desc.height = 900;
    window_desc.title = "Bevy Rider".to_string();

    App::build()
        .add_resource(window_desc)
        .add_plugins(DefaultPlugins)
        .add_resource(TickTimer(Timer::from_seconds(2.0, true)))
        .add_startup_system(setup.system())
        .add_system(camera_dragging_system.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>) {

    let camera_entity = commands
        .spawn(Camera2dComponents::default())
        .current_entity()
        .unwrap();
    commands.insert_resource(CameraDraggingState::new(camera_entity));

    spawn_candles(&mut commands, &mut materials);
}

fn spawn_candles(commands: &mut Commands, materials: &mut ResMut<Assets<ColorMaterial>>) {

    let market: Market = Binance::new(None, None);

    let KlineSummaries::AllKlineSummaries(klines) =
        match market.get_klines("ETHUSDT", "15m", 200, None, None) {
            Ok(answer) => answer,
            Err(e) => panic!("Error: {}", e),
        };
    
    //Pos x is initiated at center of screen width. Should be replaced by (window_width/2)
    let mut candle_position_x = -500.0;
    let mut candle_height;

    let mut range_high = 0.0;
    let mut range_low = 10000.0;

    for kline in &klines {
        if kline.high > range_high {
            range_high = kline.high;
        }
        if kline.low < range_low {
            range_low = kline.low;
        }
    }

    let scale = 900.0 / (range_high - range_low);
    let absolute_low = range_low * scale;

    println!("High: {}, Low: {}, scale: {}, absolute_low: {}", range_high, range_low, scale, absolute_low);

    for kline in klines {

        candle_height = (kline.high - kline.low) * scale;

        println!("kline.high {}, kline.low {}", kline.high, kline.low);
        println!("candle_height {}, candle_height scaled {}", (kline.high - kline.low), candle_height);
        /*
            The candles are not drawing correctly. Scaling likely the cause
        */
        if kline.open > kline.close {
            commands.spawn(SpriteComponents {
                material: materials.add(Color::rgb(0.9, 0.0, 0.0).into()), //red
                transform: Transform::from_translation(Vec3::new(
                    candle_position_x,
                    ((kline.low * scale) - absolute_low - 450.0) as f32,
                    0.0,
                )),
                sprite: Sprite::new(Vec2::new(9.0, candle_height.abs() as f32)),
                ..Default::default()
            });
        } else {
            commands.spawn(SpriteComponents {
                material: materials.add(Color::rgb(0.0, 0.9, 0.0).into()), //green 
                transform: Transform::from_translation(Vec3::new(
                    candle_position_x,
                    ((kline.low * scale) - absolute_low - 450.0) as f32,
                    0.0,
                )),
                sprite: Sprite::new(Vec2::new(9.0, candle_height.abs() as f32)),
                ..Default::default()
            });
        }
        candle_position_x += 11.0;
    }
}
