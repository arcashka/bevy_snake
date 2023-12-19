use bevy::{
    prelude::*,
    render::{camera::CameraProjection, primitives::Frustum},
    window::{WindowCreated, WindowResized},
};

use super::SceneResizeEvent;

pub fn setup(mut commands: Commands) {
    let projection = Projection::Orthographic(OrthographicProjection {
        far: 1000.,
        near: -1000.,
        scale: 1.0,
        ..Default::default()
    });
    let transform = Transform::default();
    let view_projection = projection.get_projection_matrix() * transform.compute_matrix().inverse();
    let frustum = Frustum::from_view_projection_custom_far(
        &view_projection,
        &transform.translation,
        &transform.back(),
        projection.far(),
    );
    commands.spawn(Camera3dBundle {
        projection,
        frustum,
        ..default()
    });
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 3.0,
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
