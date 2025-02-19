use clap::Parser;

use log::{debug, info, LevelFilter};
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use postgres::{Client, NoTls};

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const LOG_PATTERN: &str = "{d(%Y-%m-%d %H:%M:%S)}|{l:5}|>> {m}{n}";

#[derive(Parser, Debug)]
#[command(
    name = "pgvacuum",
    about = "PostgreSQL vacuum tool by Code Solver.",
    version = "1.0"
)]
struct Params {
    /// Path to the PostgreSQL configuration file
    #[arg(short = 'H', long, default_value = "localhost")]
    host: String,

    /// PostgreSQL port
    #[arg(short = 'p', long, default_value = "5432")]
    port: u16,

    /// PostgreSQL SCAP database
    #[arg(short = 'd', long, default_value = "scap")]
    database: String,

    /// SCAP DBA username
    #[arg(short = 'u', long, default_value = "scap_admin")]
    username: String,

    /// SCAP DBA password
    #[arg(short = 'P', long, default_value = "scap_admin")]
    password: String,

    /// Log file
    #[arg(short='l', long, default_value=None)]
    logfile: Option<String>,

    /// Verbose mode
    #[arg(short = 'v', long)]
    verbose: bool,
}

fn setup_logger(logfile: &Option<String>, verbose: &bool) {
    let mut builder = Config::builder();
    let mut root = Root::builder();

    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(LOG_PATTERN)))
        .build();
    builder = builder.appender(Appender::builder().build("stdout", Box::new(stdout)));
    root = root.appender("stdout");

    if let Some(logfile) = logfile {
        let file = FileAppender::builder()
           .encoder(Box::new(PatternEncoder::new(LOG_PATTERN)))
            .build(logfile)
            .unwrap();
        builder = builder.appender(Appender::builder().build("file", Box::new(file)));
        root = root.appender("file");
    }

    let level = if *verbose {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };
    let config = builder.build(root.build(level)).unwrap();

    log4rs::init_config(config).unwrap();
}

fn execute_vacuum(params: &Params) -> Result<(), Box<dyn std::error::Error>> {
    let dburl = format!(
        "postgresql://{}:{}@{}:{}/{}",
        params.username, params.password, params.host, params.port, params.database,
    );
    let mut client = Client::connect(&dburl, NoTls)?;
    debug!("Connected to database...");

    let sql: &str = if params.verbose {
        "VACUUM (VERBOSE, FULL)"
    } else {
        "VACUUM (FULL)"
    };
    client.execute(sql, &[])?;

    debug!("VACUUM executed...");

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let params = Params::parse();

    setup_logger(&params.logfile, &params.verbose);

    info!("=== {} - {} starting ===", APP_NAME, VERSION);
    debug!("Host....: {}", params.host);
    debug!("Port....: {}", params.port);
    debug!("Database: {}", params.database);
    debug!("Username: {}", params.username);

    if let Err(e) = execute_vacuum(&params) {
        eprintln!("Error executing VACUUM: {}", e);
        std::process::exit(1);
    }

    info!("=== {} - {} finished ===", APP_NAME, VERSION);

    Ok(())
}
