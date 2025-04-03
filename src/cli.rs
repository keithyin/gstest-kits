use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about=None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    BamDiff(BamDiffArgs),
}

#[derive(Args)]
pub struct BamDiffArgs {
    #[arg(short = 'a', help = "bam a")]
    pub a_bam: String,

    #[arg(short = 'b', help = "bam b")]
    pub b_bam: String,

    #[arg(long = "check-seq")]
    pub check_seq: bool,
    #[arg(long = "check-qual")]
    pub check_qual: bool,
}
