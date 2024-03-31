use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Clone, Debug, ValueEnum, PartialEq, Eq)]
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

#[derive(Clone, Debug, ValueEnum, PartialEq, Eq)]
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

    #[arg(short, long, value_name = "FILE")]
    /// The output file to write the report
    /// If not provided, the report will be printed to the console
    pub output_file: Option<PathBuf>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn report_type() -> impl Strategy<Value = ReportType> {
        prop_oneof![
            Just(ReportType::All),
            Just(ReportType::PlayerRank),
            Just(ReportType::MeanDeath),
        ]
    }

    fn report_format() -> impl Strategy<Value = ReportFormat> {
        prop_oneof![Just(ReportFormat::Html), Just(ReportFormat::Text)]
    }

    proptest! {
    #[test]
        fn verify_cmd_default(
            log_file in "\\w+"
        ) {
            let cmd = Cli::parse_from(&["test", &log_file]);
            assert_eq!(cmd.log_file, PathBuf::from(&log_file));
            assert_eq!(cmd.report_type, ReportType::All);
            assert_eq!(cmd.report_format, ReportFormat::Text);
            assert_eq!(cmd.output_file, None);
        }
    }

    proptest! {
    #[test]
        fn verify_cmd_default_flag_like_log_file(
            log_file in "--\\PC*"
        ) {
            let cmd = Cli::try_parse_from(&["test", &log_file]);
            assert!(cmd.is_err());
        }
    }

    #[test]
    fn verify_cmd_default_empty_log_file() {
        let cmd = Cli::try_parse_from(&["test", ""]);
        assert!(cmd.is_err());
    }

    proptest! {
    #[test]
        fn verify_cmd_with_report_type(
            log_file in "\\w+",
            report_type in report_type(),
        ) {
            let arg_text = match report_type {
                ReportType::All => {
                    "all"
                }
                ReportType::PlayerRank => {
                    "player-rank"
                }
                ReportType::MeanDeath => {
                    "mean-death"
                }
            };
            let cmd = Cli::parse_from(&["test", &log_file, "--report-type", arg_text]);
            assert_eq!(cmd.log_file, PathBuf::from(&log_file));
            assert_eq!(cmd.report_type, report_type);
            assert_eq!(cmd.report_format, ReportFormat::Text);
            assert_eq!(cmd.output_file, None);

            let cmd = Cli::parse_from(&["test", &log_file, "-r", arg_text]);
            assert_eq!(cmd.log_file, PathBuf::from(&log_file));
            assert_eq!(cmd.report_type, report_type);
            assert_eq!(cmd.report_format, ReportFormat::Text);
            assert_eq!(cmd.output_file, None);
        }
    }

    proptest! {
    #[test]
        fn verify_cmd_with_report_format(
            log_file in "\\w+",
            report_format in report_format(),
        ) {
            let arg_text = match report_format {
                ReportFormat::Html => {
                    "html"
                }
                ReportFormat::Text => {
                    "text"
                }
            };
            let cmd = Cli::parse_from(&["test", &log_file, "--report-format", arg_text]);
            assert_eq!(cmd.log_file, PathBuf::from(&log_file));
            assert_eq!(cmd.report_type, ReportType::All);
            assert_eq!(cmd.report_format, report_format);
            assert_eq!(cmd.output_file, None);

            let cmd = Cli::parse_from(&["test", &log_file, "-f", arg_text]);
            assert_eq!(cmd.log_file, PathBuf::from(&log_file));
            assert_eq!(cmd.report_type, ReportType::All);
            assert_eq!(cmd.report_format, report_format);
            assert_eq!(cmd.output_file, None);
        }
    }

    proptest! {
    #[test]
        fn verify_cmd_with_output_file(
            log_file in "\\w+",
            output_file in "\\w+"
        ) {
            let cmd = Cli::parse_from(&["test", &log_file, "--output-file", &output_file]);
            assert_eq!(cmd.log_file, PathBuf::from(&log_file));
            assert_eq!(cmd.report_type, ReportType::All);
            assert_eq!(cmd.report_format, ReportFormat::Text);
            assert_eq!(cmd.output_file, Some(PathBuf::from(&output_file)));

            let cmd = Cli::parse_from(&["test", &log_file, "-o", &output_file]);
            assert_eq!(cmd.log_file, PathBuf::from(&log_file));
            assert_eq!(cmd.report_type, ReportType::All);
            assert_eq!(cmd.report_format, ReportFormat::Text);
            assert_eq!(cmd.output_file, Some(PathBuf::from(&output_file)));
        }
    }

    proptest! {
        #[test]
        fn verify_cmd_with_output_file_flag_like_log_file(
            log_file in "\\w+",
            output_file in "--\\PC*"
        ) {
            let cmd = Cli::try_parse_from(&["test", &log_file, "--output-file", &output_file]);
            assert!(cmd.is_err());
        }
    }

    proptest! {
        #[test]
        fn verify_cmd_with_empty_output_file(
            log_file in "\\w+",
        ) {
            let cmd = Cli::try_parse_from(&["test", &log_file, "--output-file", ""]);
            assert!(cmd.is_err());
        }
    }

    proptest! {
    #[test]
        fn verify_cmd_full(
            log_file in "\\w+",
            report_type in report_type(),
            report_format in report_format(),
            output_file in "\\w+"
        ) {
            let type_text = match report_type {
                ReportType::All => {
                    "all"
                }
                ReportType::PlayerRank => {
                    "player-rank"
                }
                ReportType::MeanDeath => {
                    "mean-death"
                }
            };

            let format_text = match report_format {
                ReportFormat::Html => {
                    "html"
                }
                ReportFormat::Text => {
                    "text"
                }
            };

            let cmd = Cli::parse_from(
                &["test", &log_file, "--report-type", type_text, "--report-format", format_text, "--output-file", &output_file]
            );
            assert_eq!(cmd.log_file, PathBuf::from(&log_file));
            assert_eq!(cmd.report_type, report_type);
            assert_eq!(cmd.report_format, report_format);
            assert_eq!(cmd.output_file, Some(PathBuf::from(&output_file)));

            let cmd = Cli::parse_from(
                &["test", &log_file, "-r", type_text, "-f", format_text, "-o", &output_file]
            );
            assert_eq!(cmd.log_file, PathBuf::from(&log_file));
            assert_eq!(cmd.report_type, report_type);
            assert_eq!(cmd.report_format, report_format);
            assert_eq!(cmd.output_file, Some(PathBuf::from(&output_file)));
        }
    }
}
