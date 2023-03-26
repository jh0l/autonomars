use std::f32::consts::PI;

use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .insert_resource(Msaa::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_map)
        .add_startup_system(setup_physics)
        .add_system(print_ball_altitude)
        .run();
}

fn setup_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // load a texture and retrieve its aspect ratio
    let texture_handle = asset_server.load("map.png");
    let w = 2387.0;
    let h = 1362.0;
    let aspect = h / w;

    // create a new quad mesh. this is what we will apply the texture to
    let quad_width = 8.0;
    let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
        quad_width,
        quad_width * aspect,
    ))));

    // this material renders the texture normally
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle.clone()),
        alpha_mode: AlphaMode::Blend,
        ..default()
    });
    let scale = 2.;
    // textured quad - normal
    commands.spawn(PbrBundle {
        mesh: quad_handle.clone(),
        material: material_handle,
        transform: Transform::from_xyz(0., 0., 0.)
            .with_rotation(Quat::from_rotation_x(-PI / 2.))
            .with_scale(Vec3 {
                x: scale,
                y: scale,
                z: scale,
            }),
        ..default()
    });
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(10., 10., 5.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 9000.0,
            range: 100.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 8.0, 8.0),
        ..default()
    });
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 9000.0,
            range: 100.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(-8.0, 8.0, -8.0),
        ..default()
    });
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 9000.0,
            range: 1000.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 8.0, -8.0),
        ..default()
    });

    /* create the 2d plane of the map */
}

fn setup_physics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(100.0, 0.1, 100.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)));

    /* Create the bouncing ball with a PbrBundle using a sphere mesh */

    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    /* Create the bouncing ball with a ball PbrBundle using a sphere mesh */
    // commands
    //     .spawn(RigidBody::Dynamic)
    //     .insert(Collider::ball(0.5))
    //     .insert(Restitution::coefficient(0.7))
    //     .insert(PbrBundle {
    //         // sphere mesh shape
    //         mesh: sphere_mesh.clone(),
    //         material: debug_material.clone(),
    //         ..default()
    //     })
    //     .insert(TransformBundle::from(Transform::from_xyz(0.0, 5.0, 0.0)));

    /* create bouncing box */

    let drop_height = 0.7;

    let scale = 0.5;
    let box_x = 1.2 * scale;
    let box_y = 0.5 * scale;
    let box_z = 1.7 * scale;

    let box_mesh = meshes.add(
        shape::Box {
            max_x: box_x,
            max_y: box_y,
            max_z: box_z,
            min_x: -box_x,
            min_y: -box_y,
            min_z: -box_z,
        }
        .try_into()
        .unwrap(),
    );
    let cuboid = commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::cuboid(box_x, box_y, box_z))
        .insert(CollisionGroups::new(Group::GROUP_2, Group::GROUP_2))
        .insert(Restitution::coefficient(0.0))
        .insert(PbrBundle {
            // sphere mesh shape
            mesh: box_mesh.clone(),
            material: debug_material.clone(),
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(
            0.0,
            drop_height,
            0.0,
        )))
        .id();

    let drive_axle_width = 0.4;
    let drive_axle_offset = 0.6;
    let drive_axle_height = -0.1;

    let wheel_size = 0.4;
    // make sphere mesh with handle
    let sphere_mesh = meshes.add(
        shape::UVSphere {
            radius: wheel_size * 0.99,
            sectors: 16,
            stacks: 16,
        }
        .try_into()
        .unwrap(),
    );

    /* create a RevoluteJoint based sphere wheel for the box */
    let right_joint = RevoluteJointBuilder::new(Vec3::X)
        .local_anchor1(Vec3::new(
            drive_axle_width,
            drive_axle_height,
            drive_axle_offset,
        ))
        .local_anchor2(Vec3::new(0.0, 0.0, 0.0));
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(wheel_size))
        .insert(CollisionGroups::new(Group::GROUP_3, Group::GROUP_3))
        .insert(Friction::coefficient(0.7))
        .insert(PbrBundle {
            // sphere mesh shape
            mesh: sphere_mesh.clone(),
            material: debug_material.clone(),
            ..default()
        })
        .insert(ImpulseJoint::new(cuboid, right_joint))
        .insert(TransformBundle::from(Transform::from_xyz(
            0.0,
            drop_height,
            0.0,
        )));

    let left_joint = RevoluteJointBuilder::new(Vec3::X)
        .local_anchor1(Vec3::new(
            -drive_axle_width,
            drive_axle_height,
            drive_axle_offset,
        ))
        .local_anchor2(Vec3::new(0.0, 0.0, 0.0));
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(wheel_size))
        .insert(CollisionGroups::new(Group::GROUP_4, Group::GROUP_4))
        .insert(Friction::coefficient(0.7))
        .insert(PbrBundle {
            // sphere mesh shape
            mesh: sphere_mesh.clone(),
            material: debug_material.clone(),
            ..default()
        })
        .insert(ImpulseJoint::new(cuboid, left_joint))
        .insert(TransformBundle::from(Transform::from_xyz(
            0.0,
            drop_height,
            0.0,
        )));
}

fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
    for transform in positions.iter() {
        println!("Ball altitude: {}", transform.translation.y);
    }
}

/// Creates a colorful test pattern
fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
    )
}
