# pg_ch_format

The goals of that small tool are to:

- Allow running [`pg_format`](https://github.com/darold/pgFormatter/) or [`clickhouse-format`](https://clickhouse.com/docs/en/operations/utilites/clickhouse-format) on SQL files, detecting the dialect (Postgres or Clickhouse) automatically or through file extensions or annotations.
- Add a check mode to both, to verify in CI that files are properly formatted. See [this github issue](https://github.com/darold/pgFormatter/issues/137).

## Usage

```
Format Postgres or Clickhouse SQL files.

OUTPUT FORMAT
If the input is stdin (`-`), the formatted contents will be output on stdout.
Otherwise, a diff for the misformatted files is shown.

EXIT CODES
0: Success
1: Program error
2: Unformatted files

Usage: pg_ch_format [OPTIONS] <INPUT>

Arguments:
  <INPUT>
          Folder, filename, or `-` to read from stdin

Options:
  -i, --inplace
          Rewrite the formatted files..

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Dialect detection

This is for now very primitive. You can also specify the dialect manually with

```sql
-- dialect=Clickhouse
-- or
-- dialect=Postgres
```

in your SQL file.

## Known issues

- A bug in `clickhouse-format` removes some inline comments (https://github.com/ClickHouse/ClickHouse/issues/62256)
