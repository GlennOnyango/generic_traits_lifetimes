mod aggregator;

use std::collections::btree_map::IterMut;

use aggregator::{summerize, NewsArticle};

//local struct

struct Tweet {
    news_feed: String,
    retweet: bool,
    new_tweet: bool,
    reply: bool,
}

impl summerize for Tweet {
    fn summerize_author(&self) -> String {
        format!("@{}", self.news_feed)
    }
}

struct Races {
    color: String,
}

fn main() {
    let news = NewsArticle {
        headline: String::from("Money grabbed"),
        location: String::from("Nairobi"),
        author: String::from("Glenn Tedd"),
        content: String::from(
            "asdasdsd asdasdqwvolhfa  iohqhbkb q oliwn qwy  lfnuobqwg flqbqqffpfhq",
        ),
    };

    println!("News summarry {}", news.summary());

    let tweet = Tweet {
        news_feed: String::from("People are fighting over tattoos"),
        retweet: false,
        new_tweet: true,
        reply: false,
    };

    println!("{}", tweet.summary());

    fn notify(item: &impl summerize) {
        println!("Breaking news {}", item.summary());
    }

    notify(&news);
}
