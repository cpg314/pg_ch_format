use pg_ch_format::format_one;

use camino::Utf8Path as Path;
#[test]
fn test() -> anyhow::Result<()> {
    let data = Path::new("tests");
    for f in glob::glob(data.join("original").join("*.sql").as_str())? {
        let f = camino::Utf8PathBuf::try_from(f?)?;
        println!("Testing {}", f);
        assert_eq!(
            format_one(&std::fs::read_to_string(&f)?, None)?.trim(),
            std::fs::read_to_string(data.join("formatted").join(f.file_name().unwrap()))?.trim()
        );
    }
    Ok(())
}
