pub trait summerize {
    fn summary(&self) -> String {
        String::from("We have no summary for you")
    }

    fn summerize_author(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl summerize for NewsArticle {
    fn summary(&self) -> String {
        format!("{}, by {} {}", self.headline, self.author, self.location)
    }

    fn summerize_author(&self) -> String {
        format!("{}", self.author)
    }
}
