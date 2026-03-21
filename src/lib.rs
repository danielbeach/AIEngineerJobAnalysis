use std::collections::HashMap;
use std::fs;
use std::path::Path;

use colored::Colorize;

// Common English stopwords + generic job-description filler with no skill signal
const STOPWORDS: &[&str] = &[
    "a", "ability", "able", "about", "above", "across", "after", "again",
    "against", "ago", "all", "also", "although", "am", "among", "an", "and",
    "any", "apply", "are", "around", "as", "at", "be", "because", "been",
    "before", "being", "between", "both", "bring", "build", "building",
    "built", "business", "but", "by", "can", "candidate", "candidates",
    "capabilities", "collaborate", "collaboration", "come", "company",
    "compensation", "competitive", "complex", "contribute", "could",
    "customers", "day", "deliver", "department", "design", "develop",
    "development", "did", "directly", "do", "does", "doing", "don", "down",
    "driven", "drive", "driving", "during", "each", "effectively",
    "employer", "end", "engineer", "engineering", "ensure", "environment",
    "environments", "equal", "even", "existing", "experience", "experienced",
    "expertise", "fast", "few", "flexible", "focus", "focused", "following",
    "for", "from", "full", "further", "get", "global", "great", "grow",
    "growth", "had", "has", "have", "having", "he", "help", "helping",
    "her", "here", "hers", "herself", "high", "him", "himself", "hire",
    "hiring", "his", "how", "ideal", "identify", "if", "impact", "implement",
    "in", "including", "individual", "into", "involved", "is", "it", "its",
    "itself", "job", "join", "key", "knowledge", "large", "lead",
    "leadership", "leading", "learn", "learning", "level", "like", "looking",
    "make", "manage", "management", "may", "me", "members", "mission",
    "more", "most", "must", "my", "myself", "need", "needs", "new", "no",
    "nor", "not", "of", "off", "on", "once", "one", "only", "open",
    "opportunity", "or", "organization", "other", "our", "ours", "ourselves",
    "out", "over", "own", "part", "passion", "passionate", "people",
    "performance", "platform", "platforms", "plus", "position", "practices",
    "prefer", "preferred", "product", "products", "provide", "providing",
    "quality", "real", "required", "requirements", "research", "role",
    "same", "set", "she", "should", "skills", "so", "software", "solutions",
    "solve", "solving", "some", "start", "strong", "success", "such",
    "support", "take", "team", "teams", "technical", "than", "that", "the",
    "their", "theirs", "them", "themselves", "then", "there", "these",
    "they", "this", "those", "through", "time", "to", "too", "tools",
    "top", "type", "under", "understand", "unique", "until", "up", "use",
    "used", "using", "value", "values", "very", "was", "we", "were",
    "what", "when", "where", "which", "while", "who", "whom", "why",
    "will", "with", "within", "work", "working", "world", "would", "years",
    "you", "youll", "youre", "your", "yours", "yourself", "yourselves",
    // Additional HR/generic noise
    "applications", "benefits", "best", "engineers", "ensuring", "familiarity",
    "features", "modern", "process", "related", "remote", "services",
    "stakeholders",
];

/// Load all `.txt` files from `dir`, returning their contents as a `Vec<String>`.
pub fn load_texts(dir: &Path) -> std::io::Result<Vec<String>> {
    let mut paths: Vec<_> = fs::read_dir(dir)?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().map_or(false, |ext| ext == "txt"))
        .collect();
    paths.sort();
    paths.iter().map(fs::read_to_string).collect()
}

/// Split text into lowercase tokens, keeping only alphanumeric chars and hyphens.
fn tokenize(text: &str) -> impl Iterator<Item = String> + '_ {
    text.split_whitespace().filter_map(|raw| {
        let word: String = raw
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-')
            .collect();
        // Keep words longer than 2 chars that aren't purely numeric
        if word.len() > 2 && !word.chars().all(|c| c.is_ascii_digit()) {
            Some(word)
        } else {
            None
        }
    })
}

fn is_stopword(word: &str) -> bool {
    STOPWORDS.contains(&word)
}

/// Count word frequencies across all texts, filtering out stopwords.
pub fn count_words(texts: &[String]) -> HashMap<String, usize> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for text in texts {
        for word in tokenize(text) {
            if !is_stopword(&word) {
                *counts.entry(word).or_insert(0) += 1;
            }
        }
    }
    counts
}

/// Return the top `n` words sorted by frequency descending (ties broken alphabetically).
pub fn top_words(counts: &HashMap<String, usize>, n: usize) -> Vec<(String, usize)> {
    let mut pairs: Vec<(String, usize)> = counts
        .iter()
        .map(|(k, v)| (k.clone(), *v))
        .collect();
    pairs.sort_unstable_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    pairs.truncate(n);
    pairs
}

/// Print a ranked bar-chart of word frequencies to stdout.
pub fn display_word_count(words: &[(String, usize)]) {
    if words.is_empty() {
        println!("No words found.");
        return;
    }

    let max_count = words[0].1;
    let bar_width: usize = 40;
    let col_width = words.iter().map(|(w, _)| w.len()).max().unwrap_or(10).max(10);

    println!();
    println!(
        "  {:>4}  {:<width$}  {:>6}  {}",
        "RANK".bold(),
        "WORD".bold(),
        "COUNT".bold(),
        "FREQUENCY".bold(),
        width = col_width
    );
    println!("  {}", "─".repeat(col_width + bar_width + 20));

    for (i, (word, count)) in words.iter().enumerate() {
        let bar_len = if max_count > 0 {
            (count * bar_width) / max_count
        } else {
            0
        };
        let bar = "█".repeat(bar_len);

        // Pre-pad the word so ANSI codes don't break column alignment
        let padded = format!("{:<width$}", word, width = col_width);

        println!(
            "  {:>4}  {}  {:>6}  {}",
            (i + 1).to_string().bright_white(),
            padded.bright_cyan().bold(),
            count,
            bar.bright_green(),
        );
    }
    println!();
}

/// Render an ASCII word cloud to stdout.
///
/// Words are grouped into four tiers by frequency rank and displayed with
/// varying prominence (case, colour, and density per line).
pub fn display_word_cloud(words: &[(String, usize)]) {
    if words.is_empty() {
        println!("No words to display.");
        return;
    }

    let n = words.len();
    let term_width: usize = 100;

    // Build (visible_len, colored_string) for each word based on frequency tier.
    let entries: Vec<(usize, String)> = words
        .iter()
        .enumerate()
        .map(|(i, (word, _))| {
            let pct = i as f64 / n as f64;

            if pct < 0.10 {
                // Tier 1: ALL CAPS, bold bright red
                let dw = word.to_uppercase();
                let visible = dw.len() + 4;
                let colored = format!("  {}  ", dw).bright_red().bold().to_string();
                (visible, colored)
            } else if pct < 0.25 {
                // Tier 2: Title Case, bold yellow
                let dw = title_case(word);
                let visible = dw.len() + 4;
                let colored = format!("  {}  ", dw).bright_yellow().bold().to_string();
                (visible, colored)
            } else if pct < 0.55 {
                // Tier 3: Title Case, green
                let dw = title_case(word);
                let visible = dw.len() + 4;
                let colored = format!("  {}  ", dw).bright_green().to_string();
                (visible, colored)
            } else {
                // Tier 4: lowercase, cyan
                let visible = word.len() + 4;
                let colored = format!("  {}  ", word).cyan().to_string();
                (visible, colored)
            }
        })
        .collect();

    // Print header
    println!();
    let header = " AI ENGINEER SKILLS — WORD CLOUD ";
    let h_pad = term_width.saturating_sub(header.len()) / 2;
    println!("{}{}", " ".repeat(h_pad), header.on_bright_blue().bold().white());
    println!();

    // Tier layout: (words_in_tier, words_per_line)
    let t1 = (n as f64 * 0.10).ceil() as usize;
    let t2 = (n as f64 * 0.15).ceil() as usize;
    let t3 = (n as f64 * 0.30).ceil() as usize;
    let tier_layout: &[(usize, usize)] = &[(t1, 1), (t2, 2), (t3, 3), (n, 5)];

    let mut idx = 0;
    for &(tier_count, per_line) in tier_layout {
        let tier_end = (idx + tier_count).min(n);
        while idx < tier_end {
            let line_end = (idx + per_line).min(tier_end);
            let slice = &entries[idx..line_end];
            let total_visible: usize = slice.iter().map(|(len, _)| len).sum();
            let padding = term_width.saturating_sub(total_visible) / 2;
            print!("{}", " ".repeat(padding));
            for (_, colored) in slice {
                print!("{}", colored);
            }
            println!();
            idx = line_end;
        }
    }

    println!();
    println!(
        "  Legend:  {}  Top 10%   {}  Top 25%   {}  Top 55%   {}  Rest",
        "██".bright_red().bold(),
        "██".bright_yellow().bold(),
        "██".bright_green(),
        "██".cyan(),
    );
    println!();
}

fn title_case(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().to_string() + chars.as_str(),
    }
}
