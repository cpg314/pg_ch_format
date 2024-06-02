use tracing::*;

lazy_static::lazy_static! {
    static ref DIALECT_RE: regex::Regex = regex::Regex::new(r#"--\s*dialect\s*=\s*(\S*)"#).unwrap();
}

#[derive(Debug, Clone, Copy, PartialEq, clap::ValueEnum)]
pub enum Dialect {
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
fn guess_dialect(contents: &str) -> anyhow::Result<Dialect> {
    if let Some(cap) = DIALECT_RE.captures(&contents.to_lowercase()) {
        let dialect = cap.get(1).unwrap().as_str().to_lowercase();
        match dialect.as_str() {
            "clickhouse" => return Ok(Dialect::Clickhouse),
            "postgres" => return Ok(Dialect::Postgres),
            _ => {
                anyhow::bail!("Unknown dialect '{}'", dialect);
            }
        }
    }
    if ["MergeTree", "ENGINE", "UInt", "Float32", "Float64"]
        .iter()
        .any(|c| contents.contains(c))
    {
        return Ok(Dialect::Clickhouse);
    }
    warn!("Falling back to Postgres dialect");
    Ok(Dialect::Postgres)
}
pub fn format_one(contents: &str, dialect: Option<Dialect>) -> anyhow::Result<String> {
    let dialect = if let Some(dialect) = dialect {
        dialect
    } else {
        guess_dialect(contents)?
    };
    debug!(?dialect, "Guessed dialect");
    let out = dialect
        .command()
        .stdin_bytes(contents.as_bytes())
        .stdout_capture()
        .run()?
        .stdout;
    Ok(String::from_utf8(out)?)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn dialect() {
        for s in [
            "--dialect=Clickhouse",
            "-- dialect =  Clickhouse",
            "-- dialect=clickhouse",
        ] {
            assert_eq!(guess_dialect(s).unwrap(), Dialect::Clickhouse);
        }
    }
}
