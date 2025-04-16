use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Initializes the directory.
    Init,

    /// Creates a new profile.
    Create { profile: String },

    /// Switch to a specified profile.
    Switch {
        profile: String,

        #[arg(long, short, help = "Override the existing env without checks.")]
        force: bool,
    },

    /// Deletes a profile - cannot be undone.
    Remove { profile: String },

    /// Lists available profiles.
    List,

    /// Display the current active profile.
    Current,
}
