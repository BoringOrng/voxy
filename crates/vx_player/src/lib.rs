use bevy::{
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    prelude::*,
};
use vx_entity::VxEntity;

#[derive(Clone, Copy, Debug, Default, Component)]
#[component(on_add, on_remove)]
#[require(VxEntity, Name, vx_streaming::Anchor)]
pub struct Player;

impl Player {
    #[expect(
        clippy::needless_pass_by_value,
        reason = "`DeferredWorld` must be passed by value as is required by bevy"
    )]
    fn on_add(world: DeferredWorld, ctx: HookContext) {
        let player_name = world
            .get::<Name>(ctx.entity)
            .expect("all players should have a name");

        info!("Player spawned: `{player_name}`");
    }

    #[expect(
        clippy::needless_pass_by_value,
        reason = "`DeferredWorld` must be passed by value as is required by bevy"
    )]
    fn on_remove(world: DeferredWorld, ctx: HookContext) {
        let player_name = world
            .get::<Name>(ctx.entity)
            .expect("all players should have a name");

        info!("Player removed: `{player_name}`");
    }
}
