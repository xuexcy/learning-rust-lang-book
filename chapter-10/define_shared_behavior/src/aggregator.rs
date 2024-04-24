use std::fmt::Display;

pub trait Summary {
    fn summarize(&self) -> String {
        return String::from("Read more...");
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        return format!("{}, by {} ({})", self.headline, self.author, self.location);
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
        return format!("{}: {}", self.username, self.content);
    }
}

pub struct NewsArticleV2 {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticleV2 {}

pub fn notify(item: &impl Summary) {
    println!("breaking news! {}", item.summarize());
}

pub fn notify_v2<T: Summary>(item1: &T, item2: &T) {}

pub fn notify_v3<T: Summary + Display>(item1: &T, item2: &T) {}

pub fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    return 1;
}

fn returns_summarizable() -> impl Summary {
    return Tweet {
        username: String::From("horse_ebooks"),
        content: String::from("hi"),
        reply: false,
        retweet: false,
    };
}
