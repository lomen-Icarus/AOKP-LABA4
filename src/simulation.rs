//! Модуль симуляции: каждый кадр читает клавиатуру, учитывает время кадра
//! и изменяет камеру, затем переносит её положение в сущность камеры.

use bevy::prelude::*;

use crate::camera::CameraRig;

/// Скорость вращения камеры, градусы в секунду.
const ROTATION_SPEED_DEG_PER_SEC: f32 = 90.0;

/// Скорость приближения/удаления камеры, условных единиц в секунду.
const ZOOM_SPEED_UNITS_PER_SEC: f32 = 25.0;

/// Управление камерой. pressed, а не just_pressed: камера движется всё время,
/// пока клавиша удерживается. Сдвиг = скорость × время кадра.
pub fn simulate_camera(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut camera: ResMut<CameraRig>,
) {
    let delta_time = time.delta().as_secs_f32();

    // Защита от нулевого или отрицательного времени кадра.
    if delta_time <= 0.0 {
        return;
    }

    // Поворот влево/вправо.
    if keyboard.pressed(KeyCode::ArrowLeft) {
        camera.rotate_left_right(ROTATION_SPEED_DEG_PER_SEC * delta_time);
    }
    if keyboard.pressed(KeyCode::ArrowRight) {
        camera.rotate_left_right(-ROTATION_SPEED_DEG_PER_SEC * delta_time);
    }

    // Поворот вверх/вниз.
    if keyboard.pressed(KeyCode::ArrowUp) {
        camera.rotate_up_down(ROTATION_SPEED_DEG_PER_SEC * delta_time);
    }
    if keyboard.pressed(KeyCode::ArrowDown) {
        camera.rotate_up_down(-ROTATION_SPEED_DEG_PER_SEC * delta_time);
    }

    // Приближение/удаление: цифровой блок и основной ряд клавиатуры.
    let zoom_in = keyboard.pressed(KeyCode::NumpadAdd) || keyboard.pressed(KeyCode::Equal);
    let zoom_out = keyboard.pressed(KeyCode::NumpadSubtract) || keyboard.pressed(KeyCode::Minus);

    if zoom_in {
        // Приближение уменьшает радиус.
        camera.zoom_in_out(-ZOOM_SPEED_UNITS_PER_SEC * delta_time);
    }
    if zoom_out {
        // Удаление увеличивает радиус.
        camera.zoom_in_out(ZOOM_SPEED_UNITS_PER_SEC * delta_time);
    }
}

/// Записывает Transform из ресурса камеры в сущность с компонентом Camera3d.
/// Фильтр With<Camera3d> не даёт задеть Transform торов.
pub fn apply_camera_transform(
    camera: Res<CameraRig>,
    mut query: Query<&mut Transform, With<Camera3d>>,
) {
    for mut transform in &mut query {
        *transform = camera.transform();
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use bevy::time::TimeUpdateStrategy;

    use super::*;

    fn test_app(step_ms: u64) -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<ButtonInput<KeyCode>>()
            .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_millis(
                step_ms,
            )))
            .insert_resource(CameraRig::new(Vec3::new(10.0, 15.0, 17.5)))
            .add_systems(Update, (simulate_camera, apply_camera_transform).chain());
        app.world_mut()
            .spawn((Camera3d::default(), Transform::default()));
        app.update();
        app
    }

    /// Удерживает клавишу в течение одной секунды.
    fn hold_for_one_second(app: &mut App, key: KeyCode, step_ms: u64) {
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(key);
        for _ in 0..(1000 / step_ms) {
            app.update();
        }
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .release(key);
    }

    fn horizontal_angle(position: Vec3) -> f32 {
        position.z.atan2(position.x).to_degrees().rem_euclid(360.0)
    }

    #[test]
    fn rotation_speed_is_90_degrees_per_second_at_any_fps() {
        for step_ms in [20, 50, 100] {
            let mut app = test_app(step_ms);
            let before = horizontal_angle(app.world().resource::<CameraRig>().position());
            hold_for_one_second(&mut app, KeyCode::ArrowLeft, step_ms);
            let after = horizontal_angle(app.world().resource::<CameraRig>().position());
            assert!((after - before - 90.0).abs() < 0.01, "шаг {step_ms} мс");
        }
    }

    #[test]
    fn zoom_speed_is_25_units_per_second() {
        let mut app = test_app(50);
        let before = app.world().resource::<CameraRig>().position().length();
        hold_for_one_second(&mut app, KeyCode::Minus, 50);
        let after = app.world().resource::<CameraRig>().position().length();
        assert!(
            (after - before - 25.0).abs() < 0.01,
            "удаление на 25 единиц за секунду"
        );
    }

    #[test]
    fn camera_entity_follows_rig() {
        let mut app = test_app(50);
        hold_for_one_second(&mut app, KeyCode::ArrowUp, 50);
        let expected = app.world().resource::<CameraRig>().transform();
        let transform = *app
            .world_mut()
            .query_filtered::<&Transform, With<Camera3d>>()
            .single(app.world())
            .unwrap();
        assert!(
            transform
                .translation
                .abs_diff_eq(expected.translation, 1e-4)
        );
        assert!(transform.rotation.abs_diff_eq(expected.rotation, 1e-4));
    }
}
