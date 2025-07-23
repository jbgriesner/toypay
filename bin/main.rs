use anyhow::Result;
use csv::{ReaderBuilder, WriterBuilder};
use std::io;
use toypay::{cli, ToyEngine};

fn main() -> Result<()> {
    let file_path = cli::parse_args()?;

    let mut reader = ReaderBuilder::new()
        .trim(csv::Trim::All)
        .flexible(true)
        .from_path(&file_path)?;

    let mut engine = ToyEngine::new();

    for result in reader.deserialize() {
        if let Ok(transaction) = result {
            let _ = engine.dispatch(transaction);
        }
    }

    let mut writer = WriterBuilder::new().from_writer(io::stdout());
    for record in engine.get_all_accounts() {
        writer.serialize(record)?;
    }
    writer.flush()?;

    Ok(())
}
