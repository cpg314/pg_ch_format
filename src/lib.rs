use tracing::*;

#[derive(Debug)]
enum Dialect {
    Postgres,
    Clickhouse,
}
impl Dialect {
    fn command(&self) -> duct::Expression {
        match self {
            Dialect::Postgres => duct::cmd!("pg_format", "-w", "100"),
            Dialect::Clickhouse => duct::cmd!(
                "clickhouse-format",
                "--multiquery",
                "--comments",
                "--max_line_length=100"
            ),
        }
    }
}

// TODO: This is for now very primitive
fn guess_dialect(contents: &str) -> Dialect {
    for dialect in [Dialect::Postgres, Dialect::Clickhouse] {
        if contents
            .to_lowercase()
            .contains(&format!("-- dialect={:?}", dialect).to_lowercase())
        {
            return dialect;
        }
        if ["MergeTree", "ENGINE", "UInt", "Float32", "Float64"]
            .iter()
            .any(|c| contents.contains(c))
        {
            return Dialect::Clickhouse;
        }
    }
    Dialect::Postgres
}
pub fn format_one(contents: &str) -> anyhow::Result<String> {
    let dialect = guess_dialect(contents);
    debug!(?dialect, "Guessed dialect");
    let out = dialect
        .command()
        .stdin_bytes(contents.as_bytes())
        .stdout_capture()
        .run()?
        .stdout;
    Ok(String::from_utf8(out)?)
}
