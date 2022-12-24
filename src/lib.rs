use std::ops::Range;

use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};

pub struct BevyRhythmPlugin;

impl Plugin for BevyRhythmPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(ActiveState::Inactive);
        app.add_system_set(SystemSet::on_enter(ActiveState::Active).with_system(Self::on_enter));
        app.add_system_set(SystemSet::on_update(ActiveState::Active).with_system(Self::on_update));
    }
}

impl BevyRhythmPlugin {
    pub fn on_enter(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        // plane
        /* commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        });*/
        // cube
        /* commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        });*/
        // light
        commands.spawn(PointLightBundle {
            point_light: PointLight {
                intensity: 300000.0,
                range: 100000.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(15.0, 8.0, 4.0),
                scale: Vec3::new(1000.0, 1000.0, 1000.0),
                ..default()
            },
            ..default()
        });
        // camera
        commands.spawn(Camera3dBundle {
            transform: Transform {
                translation: Vec3::new(15.0, 0.0, 5.0),
                rotation: Quat::from_rotation_x(1.2),
                scale: Vec3::new(5.0, 1.0, 1.0),
            },
            camera: Camera {
                priority: 1,
                ..default()
            },
            ..default()
        });
        let space = 43.0 / 6.0;
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(1.0, 100.0)))),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.1),
            ..default()
        });

        ////////////////////////////////
        let five: u16 = 5;
        let one: u16 = 1;
        for count in one..five {
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(space, 100.0)))),

                    material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
                    transform: Transform::from_xyz(space * f32::from(count), 0.0, 0.1),
                    ..default()
                },
                RhythmPart(count),
            ));
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(1.0, 100.0)))),
                    material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
                    transform: Transform::from_xyz((space * f32::from(count)) + space, 0.0, 0.1),
                    ..default()
                },
                RhythmPart(count),
            ));
        }
    }
    pub fn on_update() {}
}

#[derive(Component)]
pub struct RhythmPart(u16);
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ActiveState {
    Inactive,
    Active,
}
