use bevy::prelude::*;
use clap::Parser as _;
use vx_client::{VxClientPlugins, player::LocalPlayerPlugin};
use vx_server::VxServerPlugins;

mod cli;

fn main() -> AppExit {
    let args = cli::Args::parse();

    let mut app = App::new();

    if args.headless() {
        app.add_plugins(VxServerPlugins);
    } else {
        app.add_plugins(VxClientPlugins.set(LocalPlayerPlugin::new(args.username().clone())));
    }

    app.run()
}
