use aijobs::{count_words, display_word_cloud, display_word_count, load_texts, top_words};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "aijobs",
    about = "Analyze word frequencies across AI Engineer job descriptions",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show a ranked frequency bar-chart of the most common words
    WordCount {
        /// Directory containing job description .txt files
        #[arg(short, long, default_value = "src/ai_job_listings")]
        dir: PathBuf,

        /// Number of top words to display
        #[arg(short, long, default_value = "40")]
        top: usize,
    },

    /// Display a visual word cloud of the most frequent words
    WordCloud {
        /// Directory containing job description .txt files
        #[arg(short, long, default_value = "src/ai_job_listings")]
        dir: PathBuf,

        /// Number of top words to include in the cloud
        #[arg(short, long, default_value = "60")]
        top: usize,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::WordCount { dir, top } => {
            let texts = load_texts(&dir).unwrap_or_else(|e| {
                eprintln!("Error loading job listings from {:?}: {}", dir, e);
                std::process::exit(1);
            });
            println!("Loaded {} job description files.", texts.len());
            let counts = count_words(&texts);
            let words = top_words(&counts, top);
            display_word_count(&words);
        }

        Commands::WordCloud { dir, top } => {
            let texts = load_texts(&dir).unwrap_or_else(|e| {
                eprintln!("Error loading job listings from {:?}: {}", dir, e);
                std::process::exit(1);
            });
            println!("Loaded {} job description files.", texts.len());
            let counts = count_words(&texts);
            let words = top_words(&counts, top);
            display_word_cloud(&words);
        }
    }
}
