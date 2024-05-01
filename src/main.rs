use std::io::Read;
use std::path::Path;

use camino::Utf8PathBuf;
use clap::Parser;
use metadata::LevelFilter;
use rayon::prelude::*;
use tracing::*;

use pg_ch_format::format_one;

#[derive(Parser)]
#[clap(version, about, verbatim_doc_comment)]
/// Format Postgres or Clickhouse SQL files.
///
/// OUTPUT FORMAT
/// If the input is stdin (`-`), the formatted contents will be output on stdout.
/// Otherwise, a diff for the misformatted files is shown.
///
/// EXIT CODES
/// 0: Success
/// 1: Program error
/// 2: Unformatted files
struct Flags {
    /// Rewrite the formatted files..
    #[clap(long, short)]
    inplace: bool,
    /// Folder, filename, or `-` to read from stdin.
    input: Utf8PathBuf,
    #[clap(long, short)]
    debug: bool,
}

fn process_file(f: impl AsRef<Path>, args: &Flags) -> anyhow::Result<bool> {
    let f = f.as_ref();
    info!("Processing {:?}", f);
    let original = std::fs::read_to_string(f)?;
    let mut formatted = format_one(&original)?;
    if !formatted.ends_with("\n\n") {
        if !formatted.ends_with('\n') {
            formatted.push('\n');
        }
        formatted.push('\n');
    }
    if formatted != original {
        warn!(
            "Difference in {:?}\n{}",
            f,
            prettydiff::diff_lines(&original, &formatted).format_with_context(
                Some(prettydiff::text::ContextConfig {
                    context_size: 5,
                    skipping_marker: "..."
                }),
                true
            )
        );
        if args.inplace {
            std::fs::write(f, &formatted)?;
        }
    }
    Ok(formatted == original)
}
fn main() {
    if let Err(e) = main_impl() {
        error!("{:?}", e);
        std::process::exit(1);
    }
}
fn main_impl() -> anyhow::Result<()> {
    let args = Flags::parse();

    tracing_subscriber::fmt()
        .with_max_level(if args.debug {
            LevelFilter::DEBUG
        } else {
            LevelFilter::INFO
        })
        .with_writer(std::io::stderr)
        .init();
    if args.inplace {
        warn!("Changes will be applied (if any)");
    }

    let well_formatted = if args.input == "-" {
        anyhow::ensure!(
            !args.inplace,
            "The --inplace flag is not supported with stdin input"
        );
        let stdin = std::io::stdin();
        let mut stdin = stdin.lock();
        let mut data = vec![];
        stdin.read_to_end(&mut data)?;
        drop(stdin);
        let data = String::from_utf8(data)?;
        println!("{}", format_one(&data)?);
        return Ok(());
    } else if args.input.is_file() {
        process_file(&args.input, &args)?
    } else {
        info!("Collecting files");
        let files: Vec<_> = glob::glob(args.input.join("**").join("*.sql").as_str())?
            .filter_map(|f| f.ok())
            .collect();
        info!("{} files to process", files.len());
        let results = files
            .into_par_iter()
            .map(|f| process_file(f, &args))
            .collect::<anyhow::Result<Vec<bool>>>()?;
        results.into_iter().all(std::convert::identity)
    };

    if !well_formatted && args.inplace {
        warn!("Wrote changes");
    }
    if !well_formatted {
        error!("Not all files are properly formatted");
        std::process::exit(2);
    }
    info!("Done");
    Ok(())
}
