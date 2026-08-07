use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Run voxy as a headless server
    #[arg(long, short = 'H', default_value_t)]
    headless: bool,

    /// When running as a client, run with this username
    #[arg(long, short, default_value_t, conflicts_with("headless"))]
    username: String,

    /// What world seed to build the world with
    #[arg(long, short)]
    seed: Option<u64>,
}

impl Args {
    pub const fn headless(&self) -> bool {
        self.headless
    }

    pub const fn username(&self) -> &String {
        &self.username
    }

    pub const fn seed(&self) -> Option<u64> {
        self.seed
    }
}
