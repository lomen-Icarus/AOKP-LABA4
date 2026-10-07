# Лабораторная работа №4: направленная камера (Rust / Bevy 0.19)

Камера смотрит в центр сцены и вращается вокруг него. Сцена — четыре тора
из лабораторной №3. FPS выводится в заголовок окна: `Lab04 [60 FPS]`.

## Запуск

```
cargo run --locked
```

Первая сборка идёт 10–15 минут: Bevy компилируется целиком.

| Клавиша | Действие |
|---|---|
| ← / → | Поворот вокруг центра, 90° в секунду |
| ↑ / ↓ | Подъём и опускание, угол возвышения от 5° до 85° |
| + / − | Приближение и удаление, 25 единиц в секунду, расстояние от 5 до 100 |

«+» и «−» работают и на цифровом блоке, и в основном ряду (клавиши «=» и «−»).

## Модули

| Файл | Назначение |
|---|---|
| `src/main.rs` | Точка входа: окно, ресурсы, системы, сцена |
| `src/camera.rs` | Ресурс `CameraRig`: сферические координаты, повороты, приближение, Transform |
| `src/simulation.rs` | Системы `simulate_camera` и `apply_camera_transform` |
| `src/display.rs` | Ресурс `FpsCounter`, системы `update_fps` и `update_window_title` |
| `src/data.rs` | Начальная позиция камеры и список объектов |
| `src/graphic_object.rs` | Модуль графического объекта из лабораторной №3, без изменений |

Отчёт: `docs/otchet_lab04.docx`. Разбор с вопросами для защиты: `docs/razbor_lab04.md`.

## Проверка

```
cargo fmt --check
cargo clippy --locked -- -D warnings
```

## Требования

Rust 1.95 или новее. Windows: установщик rustup и Visual Studio Build Tools.
Linux: X11 или XWayland и драйвер Vulkan; на Ubuntu
`sudo apt install mesa-vulkan-drivers libvulkan1 libxkbcommon-x11-0`.
