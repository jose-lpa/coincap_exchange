use clap::Parser;
use duckdb::{Connection, Result, params};
use log::{debug, error, info, warn};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "coincap_exchange",
    version = "0.1",
    author = "José L. Patiño <jose.lpa@gmail.com>"
)]
struct Cli {
    #[arg(
        short,
        long,
        value_name = "FILE",
        help = "Sets the output file to store the exchange data."
    )]
    file: PathBuf,

    #[arg(
        long,
        value_name = "API_URL",
        default_value = "https://rest.coincap.io/v3/exchanges",
        help = "Sets the CoinCap API URL to fetch exchange data."
    )]
    api_url: String,

    #[arg(
        short = 'k',
        long,
        value_name = "API_KEY",
        env = "COINCAP_API_KEY",
        help = "Sets the API key for authentication if required by the CoinCap API."
    )]
    api_key: String,

    #[arg(
        short,
        long,
        value_name = "DEBUG_LEVEL",
        default_value_t = 0,
        help = "Sets the debug level (0-3)."
    )]
    debug: u8,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let log_level = match cli.debug {
        3 => log::LevelFilter::Debug,
        2 => log::LevelFilter::Info,
        1 => log::LevelFilter::Warn,
        0 => log::LevelFilter::Error,
        other => {
            eprintln!("Unknown verbosity '{}', defaulting to 'info'", other);
            log::LevelFilter::Info
        }
    };

    env_logger::Builder::new()
        .filter_level(log_level)
        .format_timestamp(None) // cleaner output; remove line to show timestamps
        .init();

    let file_str = cli.file.to_string_lossy().into_owned();

    debug!("File: {:?}", file_str);
    debug!("API URL: {}", cli.api_url);

    debug!("Connecting to DuckDB...");
    let connection = Connection::open_in_memory()?;

    debug!("Setting up `httpfs` extension...");
    connection.execute("INSTALL httpfs;", [])?;
    connection.execute("LOAD httpfs;", [])?;

    // Disable ETag checks to avoid errors caused by live API responses changing
    // between DuckDB's internal multi-pass HTTP reads of the same endpoint.
    connection.execute("SET unsafe_disable_etag_checks = true;", [])?;

    debug!("Setting up HTTP authentication...");
    connection.execute(
        "CREATE SECRET http_auth (TYPE http, BEARER_TOKEN ?)",
        params![cli.api_key],
    )?;

    debug!("Executing COPY command to fetch and store exchange data...");
    let sql = format!(
        "COPY (
            WITH exchange_data as (
                SELECT UNNEST(data) as data
                FROM read_json('{api_url}')
            )
            SELECT 
                regexp_replace(json_extract(data, '$.exchangeId')::VARCHAR, '\"', '', 'g') AS id,
                regexp_replace(json_extract(data, '$.name')::VARCHAR, '\"', '', 'g') AS name,
                json_extract(data, '$.rank')::INTEGER AS rank,
                json_extract(data, '$.percentTotalVolume')::DOUBLE AS percentTotalVolume,
                json_extract(data, '$.volumeUsd')::DOUBLE AS volumeUsd,
                json_extract(data, '$.tradingPairs')::INTEGER AS tradingPairs,
                json_extract(data, '$.socket')::BOOLEAN AS socket,
                regexp_replace(json_extract(data, '$.exchangeUrl')::VARCHAR, '\"', '', 'g') AS exchangeUrl,
                json_extract(data, '$.updated')::BIGINT AS updated
            FROM exchange_data
        ) TO '{file_str}' (FORMAT csv, HEADER, DELIMITER ',')",
        api_url = cli.api_url,
        file_str = file_str
    );

    connection.execute(&sql, [])?;

    info!("Data successfully fetched and stored in '{}'", file_str);
    Ok(())
}
