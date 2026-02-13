use std::error::Error;

use clap::{Parser, ValueEnum};
use serde::Deserialize;

// --- Data Models ---

#[derive(Parser, Debug)]
#[command(author, version, about = "HN CLI fetcher using ureq 3.x")]
pub struct Args {
    /// Sort mode: 'latest' for new stories, 'hottest' for top stories
    #[arg(short, long, value_enum, default_value_t = SortMode::Hottest)]
    pub sort: SortMode,

    /// Number of results to return
    #[arg(short, long, default_value_t = 30)]
    pub top: usize,
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum, Debug)]
pub enum SortMode {
    Latest,
    Hottest,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Story {
    pub title: String,
    pub url: Option<String>,
    pub score: i32,
    pub by: String,
}

// --- Logic ---

/// Fetches individual story details from the HN Firebase API.
/// Uses ureq 3.x response handling.
fn get_story_details(id: u32) -> Result<Story, Box<dyn Error>> {
    let url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id);
    let mut response = ureq::get(&url).call()?;
    let story: Story = response.body_mut().read_json()?;
    Ok(story)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // Map the internal SortMode to the API endpoint string
    let endpoint = match args.sort {
        SortMode::Hottest => "topstories",
        SortMode::Latest => "newstories",
    };

    let list_url = format!("https://hacker-news.firebaseio.com/v0/{}.json", endpoint);

    println!("--- Fetching {} stories from {} ---", args.top, endpoint);

    // Fetch the list of IDs from HN
    let mut list_response = ureq::get(&list_url).call()?;
    let story_ids: Vec<u32> = list_response.body_mut().read_json()?;

    // Ensure we don't try to take more stories than the API returned
    let limit = args.top.min(story_ids.len());
    let target_ids = &story_ids[..limit];

    for (i, &id) in target_ids.iter().enumerate() {
        match get_story_details(id) {
            Ok(story) => {
                println!(
                    "{:>2}. [{:^4}] {}\n    Link: {}",
                    i + 1,
                    story.score,
                    story.title,
                    story.url.as_deref().unwrap_or("No URL")
                );
                println!("    User: {}\n", story.by);
            },
            Err(e) => eprintln!("Error fetching story {}: {}", id, e),
        }
    }

    Ok(())
}

// --- Tests ---

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that the Story struct can be correctly deserialized from JSON.
    #[test]
    fn test_story_deserialization() {
        let json = r#"{
            "by": "dhouston",
            "descendants": 71,
            "id": 8863,
            "kids": [8952, 9224],
            "score": 111,
            "time": 1175714200,
            "title": "My YC app: Sample",
            "type": "story",
            "url": "http://www.getdropbox.com/u/2/screencast.html"
        }"#;

        let story: Story = serde_json::from_str(json).unwrap();
        assert_eq!(story.title, "My YC app: Sample");
        assert_eq!(story.score, 111);
        assert_eq!(story.by, "dhouston");
    }

    /// Test that the CLI argument defaults work as expected.
    #[test]
    fn test_arg_defaults() {
        // Mocking the command line arguments
        let args = Args::try_parse_from(&["test_bin"]).unwrap();
        assert_eq!(args.top, 30);
        assert_eq!(args.sort, SortMode::Hottest);
    }

    /// Test custom CLI arguments for top and sort mode.
    #[test]
    fn test_arg_customization() {
        let args = Args::try_parse_from(&["test_bin", "--top", "5", "--sort", "latest"]).unwrap();
        assert_eq!(args.top, 5);
        assert_eq!(args.sort, SortMode::Latest);
    }

    /// Smoke test for the HN API.
    /// Note: This requires internet access and checks if the endpoint is still
    /// alive.
    #[test]
    fn test_api_endpoint_alive() {
        let url = "https://hacker-news.firebaseio.com/v0/topstories.json";
        let response = ureq::get(url).call();
        assert!(response.is_ok(), "The HN API should be reachable");
    }
}
