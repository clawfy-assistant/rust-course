//! Lesson 08: Traits

/// Trait สำหรับพิมพ์ข้อมูล
pub trait Summary {
    fn summarize(&self) -> String;
    
    // default implementation
    fn summarize_author(&self) -> String {
        String::from("(read more...)")
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
        format!("{}, by {} ({})", self.headline, self.author, self.location)
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
        format!("{}: {}", self.username, self.content)
    }
}

/// ฟังก์ชันที่รับ trait
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

/// Trait bounds
pub fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

/// สร้าง trait ให้กับ external type
pub trait Displayable {
    fn display(&self);
}

impl Displayable for i32 {
    fn display(&self) {
        println!("The number is: {}", self);
    }
}

// TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_news_article_summary() {
        let article = NewsArticle {
            headline: String::from("Rust 1.70 Released"),
            location: String::from("Internet"),
            author: String::from("Rust Team"),
            content: String::from("New features..."),
        };
        
        assert!(article.summarize().contains("Rust 1.70"));
    }

    #[test]
    fn test_tweet_summary() {
        let tweet = Tweet {
            username: String::from("rustlang"),
            content: String::from("Hello Rustaceans!"),
            reply: false,
            retweet: false,
        };
        
        assert_eq!(tweet.summarize(), "rustlang: Hello Rustaceans!");
    }
}
