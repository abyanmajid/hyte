use clap::{arg, command, Args, Parser, Subcommand};

#[derive(Parser)]
#[derive(Debug)]
pub struct CommandLines {
    #[command(subcommand)]
    pub subcommand: SubCommands,
    /// Increase logging verbosity
    #[arg(short('v'), long, action = clap::ArgAction::Count)]
    pub verbosity: u8,
    /// Enable debug output, another way to increase logging verbosity
    #[arg(
        long = "debug",
        env = "CRATE_DEBUG",
        value_name = "boolean",
        default_value_t = false
    )]
    pub debug: bool,
}

#[derive(Args, Debug)]
pub struct ServeArgs {
    /// IP address to listen on
    #[arg(
        short('i'),
        long = "ip",
        env = "API_IP",
        value_name = "address",
        default_value = "0.0.0.0"
    )]
    pub listener_ip: String,

    /// Test printing error message
    #[arg(
        short('t'),
        long = "test-error",
        value_name = "boolean",
        default_value_t = false
    )]
    pub test_err: bool,
}
