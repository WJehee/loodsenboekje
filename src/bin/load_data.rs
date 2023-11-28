use loodsenboekje::model::entry::create_entry;
use simplelog::{LevelFilter, ConfigBuilder, TermLogger, TerminalMode, ColorChoice};

#[derive(Debug, serde::Deserialize)]
struct Record {
    how: String,
    who: String,
}

#[tokio::main]
async fn main() {
    let _ = TermLogger::init(
        LevelFilter::Info,
        ConfigBuilder::new()
        .add_filter_allow("loodsenboekje".to_string())
        .build(),
        TerminalMode::Mixed,
        ColorChoice::Auto
        );

    let mut reader = csv::Reader::from_path("loodsenboekje.csv").expect("to find the file");
    for result in reader.deserialize() {
        let record: Record = result.unwrap();
        match create_entry(&record.how, &record.who).await {
            Ok(_) => (),
            Err(e) => eprintln!("{e}"),
        }
    }
}
