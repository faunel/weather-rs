use super::config;
use clap::{Parser, Subcommand};

/// Сommand line weather forecast
#[derive(Parser)]
#[command(name = "git")]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
/// command line argument processing
pub enum Commands {
    /// Change API key for provider
    #[command(arg_required_else_help = true)]
    Conf { provider: config::Providers },
    /// Receiving the weather forecast at the address
    #[command(arg_required_else_help = true)]
    Get { address: String },
    /// Getting the provider by default
    #[command(arg_required_else_help = true)]
    Default { provider: config::Providers },
}
