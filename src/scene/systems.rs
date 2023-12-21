use bevy::{
    prelude::*,
    window::{WindowCreated, WindowResized},
};

use super::SceneResizeEvent;

pub fn setup(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(0.0, 100.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 20000.0,
            ..default()
        },
        ..default()
    });
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 30.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

pub fn window_events_listener(
    mut resize_events_reader: EventReader<WindowResized>,
    mut create_events_reader: EventReader<WindowCreated>,
    mut resize_events_writer: EventWriter<SceneResizeEvent>,
    window_query: Query<(Entity, &Window)>,
) {
    let mut window_entity = None;
    for resize_event in resize_events_reader.read() {
        window_entity = Some(resize_event.window);
    }
    if window_entity.is_none() {
        for create_event in create_events_reader.read() {
            window_entity = Some(create_event.window);
        }
    }
    if window_entity.is_none() {
        return;
    }
    let window_entity = window_entity.unwrap();

    for (_, window) in window_query
        .iter()
        .filter(|(entity, _)| *entity == window_entity)
    {
        resize_events_writer.send(SceneResizeEvent {
            size: Vec2::new(window.width(), window.height()),
        });
    }
}
