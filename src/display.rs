//! Модуль вспомогательной визуальной информации: подсчёт кадров в секунду
//! и вывод значения в заголовок окна. Сцену и камеру не изменяет.

use std::time::Duration;

use bevy::prelude::*;

/// Счётчик кадров в секунду.
#[derive(Resource, Default)]
pub struct FpsCounter {
    /// Количество кадров, накопленных за текущий интервал измерения.
    frames: u32,

    /// Время, прошедшее с начала текущего интервала измерения.
    elapsed: Duration,

    /// Последнее вычисленное значение FPS.
    fps: f32,
}

/// Подсчёт FPS: каждый вызов — один кадр. Раз в полсекунды FPS пересчитывается
/// как число кадров, делённое на фактически прошедшее время.
pub fn update_fps(time: Res<Time>, mut counter: ResMut<FpsCounter>) {
    counter.frames += 1;
    counter.elapsed += time.delta();

    let elapsed_secs = counter.elapsed.as_secs_f32();

    if elapsed_secs >= 0.5 {
        counter.fps = (counter.frames as f32 / elapsed_secs).max(0.0);
        counter.frames = 0;
        counter.elapsed = Duration::ZERO;
    }
}

/// Вывод FPS в заголовок окна. Заголовок меняется, только если
/// округлённое значение изменилось; прошлое значение хранится в Local.
pub fn update_window_title(
    mut windows: Query<&mut Window>,
    counter: Res<FpsCounter>,
    mut last_fps: Local<u32>,
) {
    let fps = counter.fps.round().max(0.0) as u32;

    if fps != *last_fps {
        *last_fps = fps;

        for mut window in &mut windows {
            window.title = format!("Lab04 [{fps} FPS]");
        }
    }
}
