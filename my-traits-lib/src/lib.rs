use std::fmt::Display;

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("Read from: `{}`...", self.summarize_author())
    }
}
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    // oveloading
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// ----------------------------------------------------
// Using Traits To return a type that implements a trait
// WARN: We can only use it with a single type (not allowed to return more than one type
// ----------------------------------------------------
pub fn return_summarizabl() -> impl Summary {
    SocialPost {
        username: "hamo".to_string(),
        content: "hamo".to_string(),
        reply: true,
        repost: false,
    }
}

// ----------------------------------------------------
// Using Traits as parameters
// ----------------------------------------------------
pub fn notify(item: &impl Summary) {
    println!("Urget news!! {}", item.summarize());
}

// ----------------------------------------------------
// Another way of using Traits as parameters with generics
// ----------------------------------------------------
pub fn notify_generic<T: Summary>(item: &T) {
    println!("Urget news!! {}", item.summarize());
}

// ----------------------------------------------------
// Multible trait bounds
// ----------------------------------------------------
pub fn notify_multi(item: &(impl Summary + Display)) {
    println!("Urget news!! {}", item.summarize());
}

// ----------------------------------------------------
// Multible trait bounds as generics
// ----------------------------------------------------
pub fn notify_multi_generic<T: Summary + Display>(item: &T) {
    println!("Urget news!! {}", item.summarize());
}

// ----------------------------------------------------
// Multible trait bounds as generics with `where` clause
// ----------------------------------------------------
pub fn notify_multi_generic_where<T>(item: &T)
where
    T: Summary + Display,
{
    3.to_string();
    println!("Urget news!! {}", item.summarize());
}

// ----------------------------------------------------
// Using traits to constrain method that implements this trait
// ----------------------------------------------------
pub struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    pub fn new(&self, x: T, y: T) -> Self {
        Self { x, y }
    }
}

// types that implements Display + PartialOrd will have the `cmp_diplay` method
impl<T: Display + PartialOrd> Pair<T> {
    pub fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
