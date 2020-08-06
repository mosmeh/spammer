use anyhow::Result;
use rand::distributions::Alphanumeric;
use rand::Rng;
use rayon::prelude::*;
use std::env;
use std::fs::{self, File};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION")
)]
struct Opt {
    /// Directory to create
    dir: PathBuf,

    /// Number of files to create
    #[structopt(short, default_value = "100")]
    n: usize,

    /// Filename length
    #[structopt(short, long, default_value = "64")]
    len: usize,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    fs::create_dir_all(&opt.dir)?;
    env::set_current_dir(&opt.dir)?;

    (0..opt.n).into_par_iter().for_each(|_| {
        let filename: String = rand::thread_rng()
            .sample_iter(Alphanumeric)
            .take(opt.len)
            .collect();
        let _ = File::create(filename);
    });

    Ok(())
}
