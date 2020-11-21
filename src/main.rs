
use std::path::PathBuf;

use clap::Clap;

use anagma::config::Config;

#[derive(Clap)]
pub(crate) struct Opts {
    pub(crate) item_path: PathBuf,
    #[clap(long, alias = "config")]
    pub(crate) config_file: Option<PathBuf>,
}

fn main() {
    let opts = Opts::parse();

    let config = opts.config_file.map(|fp| Config::from_file(&fp).unwrap()).unwrap_or_else(Default::default);

    println!("{:?}", config.selection);
    println!("{:?}", config.sorter);
    println!("{:?}", config.sourcer);

    let block = anagma::get_with_config(&opts.item_path, &config);

    println!("{:?}", block);
}
