use clap::{Parser, Subcommand};
use super::nft::NftId;

#[derive(Parser, Debug)]
#[command(
    name = "",
    version, 
    about = "Mementor lets you create hilarious memes, mint them as exclusive MEME NFTs, and earn MEME coins in return! 🎨🚀🔥", 
    long_about = None
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(about = "Search for meme templates")]
    Search {
        #[arg(help = "Query used to search for meme templates")]
        query: String,
        #[arg(default_value_t = 1, help = "Optional page number (default = 1)")]
        page: usize,
    },
    #[command(about = "Generate a meme from a template")]
    Gen {
        #[arg(help = "Numeric id of the meme template as returned by '/meme search'")]
        id: u32,
        #[arg(help = "Captions, separated by white-space, to be displayed on the image (use single-quotes, ie: 'some caption', to preserve white-spaces)")]
        captions: Vec<String>,
    },
    #[command(about = "Use AI to create a story and suggest captions for generating a meme from a template")]
    Suggest {
        #[arg(help = "Numeric id of the meme template as returned by '/meme search'")]
        id: u32,
        #[arg(help = "The story mood to use, e.g.: happy, sad, weird, silly, etc (default happy)")]
        mood: Option<String>,
        #[arg(help = "The story topic, e.g.: crypto, cats, cars, etc (default crypto)")]
        topic: Option<String>,
    },
    #[command(about = "Post a meme previously created with '/'meme gen'")]
    Post {
        #[arg(help = "Optional alphanumeric id of the meme returned by '/meme gen' (default: last meme generated)")]
        id: Option<String>
    },
    #[command(subcommand, about = "Sub-commands of the **Mementor Wallet**")]
    Wallet (Wallet),
    #[command(subcommand, about = "Sub-commands of the **Mementor NFT collection**")]
    Nft (Nft),
}

#[derive(Subcommand, Debug)]
pub enum Wallet {
    #[command(about = "Display your ICP balance in the Mementor Wallet")]
    Balance,
    #[command(about = "Display your ICP address in the Mementor Wallet")]
    Address,
    #[command(about = "Withdraw ICP from your account in the Mementor Wallet")]
    Withdraw {
        #[arg(help = "Amount to withdraw in decimal format (ie: 1.25)")]
        amount: f32,
        #[arg(help = "Optional destination account address in hex format (default: your OC wallet)")]
        to: Option<String>,
    },
    #[command(about = "Display logs of ICP transactions")]
    Logs {
        #[arg(default_value_t = 1, help = "Optional page number (default = 1)")]
        page: usize,
    },
}

#[derive(Subcommand, Debug)]
pub enum Nft {
    #[command(about = "Mint a NFT, previously posted with '/meme post'")]
    Mint {
        #[arg(help = "Optional alphanumeric id of the meme returned by '/meme gen' (default: last meme posted)")]
        id: Option<String>,
    },
    #[command(about = "Print the balance of a principal")]
    Balance {
        #[arg(short, long, help = "Optional principal of the owner (default = your OC user id")]
        owner: Option<String>,
    },
    #[command(about = "Display the tokens of a principal")]
    Tokens {
        #[arg(short, long, help = "Optional principal of the owner (default = your OC user id)")]
        owner: Option<String>,
        #[arg(default_value_t = 1, help = "Optional page number (default = 1)")]
        page: usize,
    },
    #[command(about = "Transfer a token to another principal")]
    Transfer {
        #[arg(help = "Token id, as returned by '/meme nft tokens'")]
        id: NftId,
        #[arg(help = "Principal of the recipient")]
        to: String,
    },
    #[command(about = "Display logs of NFT transactions")]
    Logs {
        #[arg(default_value_t = 1, help = "Optional page number (default = 1)")]
        page: usize,
    },
    #[command(about = "Print the status of the NFT collection")]
    Status,
}
