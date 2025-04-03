use clap::Parser;
use gstest_kits::{
    bam_diff::bam_diff,
    cli::{Cli, Commands},
};

fn main() {
    let cli_param = Cli::parse();

    match &cli_param.command {
        Commands::BamDiff(bam_diff_args) => {
            bam_diff(bam_diff_args);
        }
    }
}
