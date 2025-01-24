use std::path::PathBuf;

#[derive(clap::Parser)]
pub struct ArgsParser {
    #[command(subcommand)]
    pub cmds: Cmd,
    #[arg(short, long,)]
    pub resolver: Option<bool>,
}

#[derive(clap::Subcommand)]

pub enum Cmd {
    Tokenize { file: PathBuf },
    Parse { file: PathBuf },
    Evaluate  { file: PathBuf },
    Run  { file: PathBuf },
}
