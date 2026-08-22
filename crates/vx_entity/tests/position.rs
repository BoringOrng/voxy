use bevy::prelude::*;
use vx_entity::{EntityPlugin, VxEntity};
use vx_world::chunk::{Chunk, ChunkPos};

#[test]
fn entity_syncs_chunk_pos() {
    let mut app = App::new();

    app.add_plugins((MinimalPlugins, EntityPlugin)).add_systems(
        Update,
        |mut entities: Query<&mut Transform, With<VxEntity>>| {
            for mut transform in &mut entities {
                transform.translation += Chunk::SIZE.as_vec3();
            }
        },
    );

    let entity_id = app.world_mut().spawn(VxEntity).id();

    assert_eq!(
        *app.world().get::<ChunkPos>(entity_id).unwrap(),
        ChunkPos::new(IVec3::ZERO),
    );

    app.update();

    assert_eq!(
        *app.world().get::<ChunkPos>(entity_id).unwrap(),
        ChunkPos::new(IVec3::ONE),
    );
}
