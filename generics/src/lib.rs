pub trait Summary {
    fn summarize(&self) -> &String;
}

pub struct NewsArticle {
    pub text: String
}

impl Summary for NewsArticle {
    fn summarize(&self) -> &String {
        &self.text
    }
}
