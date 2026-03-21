# AIJobs

A Rust CLI tool for analyzing word frequencies across AI Engineer job descriptions to identify the most commonly required skills.

## Features

- **Word Count** — ranked frequency bar-chart with visual bars
- **Word Cloud** — color-coded terminal word cloud grouped by frequency tier

Both commands filter out English stopwords and generic HR boilerplate so only meaningful skill/tech signal surfaces.

## Usage

```bash
# Ranked bar-chart of top 40 words
cargo run -- word-count

# Word cloud of top 60 words
cargo run -- word-cloud

# Custom options
cargo run -- word-count --top 20
cargo run -- word-cloud --top 80 --dir path/to/listings
```

### Subcommands

| Subcommand   | Flag          | Default                  | Description                        |
|--------------|---------------|--------------------------|------------------------------------|
| `word-count` | `--top`       | 40                       | Number of words to display         |
| `word-count` | `--dir`       | `src/ai_job_listings`    | Directory of `.txt` job files      |
| `word-cloud` | `--top`       | 60                       | Number of words to include         |
| `word-cloud` | `--dir`       | `src/ai_job_listings`    | Directory of `.txt` job files      |

## Word Cloud Legend

| Color      | Tier         |
|------------|--------------|
| 🔴 Red     | Top 10%      |
| 🟡 Yellow  | Top 25%      |
| 🟢 Green   | Top 55%      |
| 🔵 Cyan    | Rest         |

## Project Structure

```
src/
├── lib.rs                  # Core library (tokenization, counting, display)
├── main.rs                 # CLI entry point (clap)
└── ai_job_listings/        # 30 AI Engineer job description .txt files
```

## Dependencies

- [`clap`](https://github.com/clap-rs/clap) — CLI argument parsing
- [`colored`](https://github.com/colored-rs/colored) — terminal colors
