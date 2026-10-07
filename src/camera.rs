//! Модуль направленной камеры. Камера смотрит в центр сцены и вращается
//! вокруг него. Состояние хранится в сферических координатах (расстояние,
//! горизонтальный угол, угол возвышения), позиция пересчитывается из них.
//! Камера не читает клавиатуру и не знает о времени: её двигают другие системы.

use bevy::prelude::*;

/// Минимальный и максимальный угол возвышения камеры.
const MIN_ANGLE_Y_DEG: f32 = 5.0;
const MAX_ANGLE_Y_DEG: f32 = 85.0;

/// Минимальное и максимальное расстояние до точки наблюдения.
const MIN_RADIUS: f32 = 5.0;
const MAX_RADIUS: f32 = 100.0;

/// Направленная камера. Камера одна, поэтому её состояние хранится в ресурсе.
/// Поля закрыты: менять камеру можно только через методы, которые держат
/// сферические параметры и позицию согласованными и в допустимых диапазонах.
#[derive(Resource)]
pub struct CameraRig {
    /// Расстояние от камеры до точки наблюдения.
    radius: f32,

    /// Горизонтальный угол поворота камеры вокруг оси Oy, в градусах.
    angle_x_deg: f32,

    /// Угол возвышения камеры над горизонтальной плоскостью, в градусах.
    angle_y_deg: f32,

    /// Текущая позиция камеры в глобальной системе координат.
    position: Vec3,
}

impl CameraRig {
    /// Конструктор: создаёт камеру в заданной декартовой позиции.
    pub fn new(position: Vec3) -> Self {
        let mut camera = Self {
            radius: 0.0,
            angle_x_deg: 0.0,
            angle_y_deg: 0.0,
            position: Vec3::ZERO,
        };

        camera.set_position(position);
        camera
    }

    /// Устанавливает позицию камеры по декартовым координатам,
    /// пересчитывая внутренние сферические параметры.
    pub fn set_position(&mut self, position: Vec3) {
        let length = position.length();

        // Защита от нулевой или почти нулевой позиции.
        if length <= f32::EPSILON {
            self.radius = MIN_RADIUS;
            self.angle_x_deg = 0.0;
            self.angle_y_deg = MIN_ANGLE_Y_DEG;
            self.recalculate_position();
            return;
        }

        // Расстояние до точки наблюдения ограничивается допустимым диапазоном.
        self.radius = length.clamp(MIN_RADIUS, MAX_RADIUS);

        // Вертикальный угол определяется через отношение высоты к расстоянию.
        let vertical_ratio = (position.y / length).clamp(-1.0, 1.0);
        self.angle_y_deg = vertical_ratio
            .asin()
            .to_degrees()
            .clamp(MIN_ANGLE_Y_DEG, MAX_ANGLE_Y_DEG);

        // Горизонтальный угол определяется через проекцию позиции
        // на горизонтальную плоскость.
        let horizontal_length = Vec3::new(position.x, 0.0, position.z).length();

        self.angle_x_deg = if horizontal_length <= f32::EPSILON {
            // Камера строго над центром сцены — угол считается нулевым.
            0.0
        } else {
            position.z.atan2(position.x).to_degrees().rem_euclid(360.0)
        };

        // Пересчитываем позицию, чтобы убрать возможные ошибки округления.
        self.recalculate_position();
    }

    /// Текущая позиция камеры в глобальной системе координат.
    pub fn position(&self) -> Vec3 {
        self.position
    }

    /// Поворот влево/вправо: меняется только горизонтальный угол.
    /// rem_euclid возвращает угол в диапазон 0..360°.
    pub fn rotate_left_right(&mut self, degrees: f32) {
        self.angle_x_deg = (self.angle_x_deg + degrees).rem_euclid(360.0);
        self.recalculate_position();
    }

    /// Поворот вверх/вниз: меняется угол возвышения в пределах 5..85°.
    pub fn rotate_up_down(&mut self, degrees: f32) {
        self.angle_y_deg = (self.angle_y_deg + degrees).clamp(MIN_ANGLE_Y_DEG, MAX_ANGLE_Y_DEG);
        self.recalculate_position();
    }

    /// Приближение/удаление: меняется расстояние в пределах 5..100.
    pub fn zoom_in_out(&mut self, distance: f32) {
        self.radius = (self.radius + distance).clamp(MIN_RADIUS, MAX_RADIUS);
        self.recalculate_position();
    }

    /// Transform для сущности камеры: камера стоит в своей позиции и смотрит
    /// в центр сцены (аналог gluLookAt).
    pub fn transform(&self) -> Transform {
        Transform::from_translation(self.position).looking_at(Vec3::ZERO, Vec3::Y)
    }

    /// Пересчёт позиции из сферических параметров.
    fn recalculate_position(&mut self) {
        let angle_x_rad = self.angle_x_deg.to_radians();
        let angle_y_rad = self.angle_y_deg.to_radians();

        // Радиус горизонтальной окружности, по которой камера движется на текущей высоте.
        let horizontal_radius = self.radius * angle_y_rad.cos();

        self.position = Vec3::new(
            horizontal_radius * angle_x_rad.cos(),
            self.radius * angle_y_rad.sin(),
            horizontal_radius * angle_x_rad.sin(),
        );
    }
}
