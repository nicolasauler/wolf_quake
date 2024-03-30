use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Clone, Debug, ValueEnum)]
/// Type of report to generate:
/// - Report with player ranking and mean of death ranking
/// - Report with player ranking
/// - Report with mean of death ranking
pub enum ReportType {
    /// Player kill score ranking + mean of death ranking
    All,
    /// Only player kill score ranking
    PlayerRank,
    /// Only mean of death ranking
    MeanDeath,
}

#[derive(Clone, Debug, ValueEnum)]
/// Format of report to generate:
/// - Text table report in console
/// - Html table report
pub enum ReportFormat {
    /// HTML table report
    Html,
    /// Text console report with tabled crate
    Text,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
/// The CLI struct
/// Defines the declarative CLI interface using the `clap` crate
pub struct Cli {
    /// The path to the log file, required
    pub log_file: PathBuf,

    #[arg(short, long, value_enum, default_value = "all")]
    /// The type of report to generate
    /// - Report with player ranking and mean of death ranking
    /// - Report with player ranking
    /// - Report with mean of death ranking
    /// Default: all
    pub report_type: ReportType,

    #[arg(short = 'f', long, value_enum, default_value = "text")]
    /// The format of the report to generate
    /// - Text table report in console
    /// - Html table report
    /// Default: text
    pub report_format: ReportFormat,

    #[arg(short, long)]
    /// The output file to write the report
    /// If not provided, the report will be printed to the console
    pub output_file: Option<PathBuf>,
}
