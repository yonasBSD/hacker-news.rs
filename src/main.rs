use clap::{Parser, ValueEnum};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use serde::Deserialize;
use std::error::Error;

// --- Data Models ---

#[derive(Parser, Debug)]
#[command(author, version, about = "A stylish HN CLI fetcher")]
struct Args {
    /// Sort mode: 'latest' for new stories, 'hottest' for top stories
    #[arg(short, long, value_enum, default_value_t = SortMode::Hottest)]
    sort: SortMode,

    /// Number of results to return
    #[arg(short, long, default_value_t = 30)]
    count: usize,
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum, Debug)]
enum SortMode {
    Latest,
    Hottest,
}

#[derive(Deserialize, Debug)]
struct Story {
    title: String,
    url: Option<String>,
    score: i32,
    by: String,
}

// --- Logic ---

/// Fetches details for a single story.
/// Comments: Using ureq 3.x body_mut() pattern.
fn get_story_details(id: u32) -> Result<Story, Box<dyn Error>> {
    let url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id);
    let mut response = ureq::get(&url).call()?;
    let story: Story = response.body_mut().read_json()?;
    Ok(story)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // Visual header
    println!("\n{}", " 🧡 Hacker News CLI ".on_cyan().black().bold());

    let endpoint = match args.sort {
        SortMode::Hottest => "topstories",
        SortMode::Latest => "newstories",
    };

    let list_url = format!("https://hacker-news.firebaseio.com/v0/{}.json", endpoint);

    // 1. Fetch story IDs
    let mut list_response = ureq::get(&list_url).call()?;
    let story_ids: Vec<u32> = list_response.body_mut().read_json()?;
    let limit = args.count.min(story_ids.len());
    let target_ids = &story_ids[..limit];

    // 2. Set up Progress Bar
    // Comments: indicatif helps manage user expectations during blocking I/O
    let pb = ProgressBar::new(limit as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")?
        .progress_chars("#>-"));

    let mut stories = Vec::new();

    // 3. Fetch stories sequentially
    for &id in target_ids {
        if let Ok(story) = get_story_details(id) {
            stories.push(story);
        }
        pb.inc(1);
    }

    pb.finish_and_clear();

    // 4. Pretty Print Results
    for (i, story) in stories.iter().enumerate() {
        let index = format!("{:>2}.", i + 1).dimmed();
        let score = format!("[{:^4}]", story.score).yellow().bold();
        let title = story.title.white().bold();
        let author = format!("by {}", story.by).bright_black();

        println!("{} {} {}", index, score, title);

        if let Some(url) = &story.url {
            println!("      {} {}", "🔗".dimmed(), url.cyan().underline());
        }
        println!("      {}\n", author);
    }

    println!("{}", "Done!".green().bold());
    Ok(())
}
