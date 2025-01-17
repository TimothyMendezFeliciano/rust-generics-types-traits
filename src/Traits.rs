pub trait Summary {
    fn summarize(&self) -> String {
        String::from("Default Implementation. Read more...")
    }
}

pub trait Display {
    fn display(&self) -> String {
        String::from("Displaying myself.")
    }
}

pub struct Comment {
    pub content: String,
    pub author: String,
}

impl Summary for Comment {}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!(
            "{}, by {} ({})",
            self.headline,
            self.author,
            self.location
        )
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!(
            "{}: {}", self.username, self.content
        )
    }
}

fn returns_summarizable() -> impl Summary { // Returns a type that implements the Summary trait without naming the concrete type.
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("writing simple gibberish about horses"),
        reply: false,
        retweet: false
    };
}

