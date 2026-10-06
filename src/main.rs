//! Лабораторная работа №4: направленная камера, система симуляции,
//! подсчёт FPS и модульная структура проекта (Rust / Bevy 0.19).
//! Главный файл подключает модули, создаёт окно, регистрирует ресурсы
//! и системы и создаёт сцену.

mod camera;
mod data;
mod display;
mod graphic_object;
mod simulation;

use bevy::prelude::*;

use camera::CameraRig;
use data::{initial_camera_position, scene_objects};
use display::{FpsCounter, update_fps, update_window_title};
use graphic_object::spawn_graphic_object;
use simulation::{apply_camera_transform, simulate_camera};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Lab04".into(),
                resolution: (1000, 700).into(),
                ..default()
            }),
            ..default()
        }))
        // Фон и рассеянный свет — как в ЛР №3.
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(GlobalAmbientLight {
            color: Color::WHITE,
            brightness: 250.0,
            affects_lightmapped_meshes: true,
        })
        .insert_resource(CameraRig::new(initial_camera_position()))
        .insert_resource(FpsCounter::default())
        .add_systems(Startup, setup_scene)
        // Порядок: ввод и изменение камеры, перенос Transform в сущность камеры,
        // затем подсчёт FPS и заголовок окна.
        .add_systems(
            Update,
            (
                simulate_camera,
                apply_camera_transform,
                update_fps,
                update_window_title,
            )
                .chain(),
        )
        .run();
}

/// Начальная сцена: графические объекты из модуля data, источник света
/// и камера, начальный Transform которой берётся из ресурса CameraRig.
fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    camera_rig: Res<CameraRig>,
) {
    for object in scene_objects() {
        spawn_graphic_object(&mut commands, &mut meshes, &mut materials, object);
    }

    commands.spawn((
        DirectionalLight {
            illuminance: 3_000.0,
            ..default()
        },
        Transform::from_xyz(5.0, 5.0, 7.5).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    println!("Позиция камеры: {:?}", camera_rig.position());
    commands.spawn((Camera3d::default(), camera_rig.transform()));
}
