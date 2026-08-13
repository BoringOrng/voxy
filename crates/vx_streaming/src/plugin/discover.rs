use bevy::prelude::*;
use vx_world::chunk::{ChunkMap, ChunkPos};

use crate::{anchor, geometry::AxisSplit, queue};

type ChangedAnchors<'w, 's> = Query<
    'w,
    's,
    (
        &'static ChunkPos,
        &'static GlobalTransform,
        &'static mut anchor::LastPos,
    ),
    Changed<ChunkPos>,
>;

#[expect(
    clippy::needless_pass_by_value,
    reason = "
        `Res<ChunkMap>` and `Res<crate::Config>` must be passed by value as
         is required by bevy
    "
)]
pub fn discover_chunks(
    chunk_map: Res<ChunkMap>,
    stream_config: Res<crate::Config>,
    mut chunk_queue: ResMut<crate::Queue>,
    mut anchors: ChangedAnchors,
    all_anchors: Query<&ChunkPos, With<crate::Anchor>>,
    mut commands: Commands,
) {
    if anchors.is_empty() {
        return;
    }

    let load_radius = stream_config.load_radius();
    let unload_radius = stream_config.unload_radius();

    for (&chunk_pos, transform, mut last) in &mut anchors {
        let new_pos = *chunk_pos;

        let Some(old_pos) = last.0 else {
            last.0 = Some(new_pos);

            enqueue_box(
                new_pos,
                load_radius,
                &chunk_map,
                &mut chunk_queue,
                transform.forward(),
            );

            continue;
        };

        match AxisSplit::box_diff(new_pos, old_pos, load_radius) {
            Some(cells) => {
                for pos in cells {
                    let pos = ChunkPos::new(pos);

                    if !chunk_map.contains_key(&pos) {
                        chunk_queue.push(queue::Item::new(
                            ChunkPos::new(new_pos),
                            transform.forward(),
                            pos,
                        ));
                    }
                }
            }
            None => enqueue_box(
                new_pos,
                load_radius,
                &chunk_map,
                &mut chunk_queue,
                transform.forward(),
            ),
        }

        match AxisSplit::box_diff(old_pos, new_pos, load_radius) {
            Some(vacated) => {
                for pos in vacated {
                    let still_wanted = all_anchors
                        .iter()
                        .any(|&p| p.chebyshev_distance(pos) < load_radius);

                    if !still_wanted {
                        chunk_queue.remove(&ChunkPos::new(pos));
                    }
                }
            }
            None => chunk_queue.retain(|&pos| {
                all_anchors
                    .iter()
                    .any(|&p| p.chebyshev_distance(*pos) < load_radius)
            }),
        }

        match AxisSplit::box_diff(old_pos, new_pos, unload_radius) {
            Some(vacated) => {
                for pos in vacated {
                    let still_kept = all_anchors
                        .iter()
                        .any(|&p| p.chebyshev_distance(pos) < unload_radius);

                    if !still_kept && let Some(&entity) = chunk_map.get(&ChunkPos::new(pos)) {
                        commands.entity(entity).despawn();
                    }
                }
            }
            None => {
                for (pos, &entity) in chunk_map.iter() {
                    let keep = all_anchors
                        .iter()
                        .any(|&p| p.chebyshev_distance(**pos) < unload_radius);

                    if !keep {
                        commands.entity(entity).despawn();
                    }
                }
            }
        }

        last.0 = Some(new_pos);
    }
}

fn enqueue_box(
    center: IVec3,
    radius: u32,
    chunk_map: &ChunkMap,
    queue: &mut crate::Queue,
    forward_dir: Dir3,
) {
    let min = center - UVec3::splat(radius).as_ivec3();
    let max = center + UVec3::splat(radius).as_ivec3();

    for x in min.x..=max.x {
        for y in min.y..=max.y {
            for z in min.z..=max.z {
                let pos = ChunkPos::new(IVec3::new(x, y, z));
                if !chunk_map.contains_key(&pos) {
                    queue.push(queue::Item::new(ChunkPos::new(center), forward_dir, pos));
                }
            }
        }
    }
}
