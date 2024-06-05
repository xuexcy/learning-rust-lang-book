pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        return Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
    pub fn add_text(&mut self, text: &str) {
        if let Some(s) = self.state.take() {
            s.add_text(self, text);
            self.state = Some(s);
        }
    }
    fn internal_add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn content(&self) -> &str {
        // Post 只存储 content，什么时候能获取content由 State 决定
        // 所以将 Post 作为参数传给 State 来获取content
        // 而不是将判断逻辑(使用match语法)写在 Post 内
        return self.state.as_ref().unwrap().content(self);
    }
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }
    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject());
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        return ""
    }
    fn add_text(&self, post: &mut Post, text: &str) {
    }
}

struct Draft {

}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        return Box::new(PendingReview {approve_cnt: 0});
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        return self;
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        return self;
    }
    fn add_text(&self, post: &mut Post, text: &str) {
        post.internal_add_text(text);
    }
}

struct PendingReview {
    approve_cnt: u32,
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        return Box::new(PendingReview {
            approve_cnt: 0,
        });
        // return self;
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        let approve_cnt = self.approve_cnt + 1;
        match approve_cnt {
            1 => Box::new(PendingReview { approve_cnt : approve_cnt}),
            _ => Box::new(Published {}),
        }
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        return Box::new(Draft{});
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        return self;
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        return self;
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        return self;
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        return &post.content;
    }
}

pub struct PostV2 {
    content: String,
}
pub struct DraftPost {
    content: String,
}

impl PostV2 {
    pub fn new() -> DraftPost {
        return DraftPost {
            content: String::new(),
        };
    }
    pub fn content(&self) -> &str {
        return &self.content;
    }
}
impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn request_review(self) -> PendingReviewPost {
        return PendingReviewPost {
            content: self.content,
        };
    }
}
pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> PostV2 {
        return PostV2 {
            content: self.content,
        };
    }
}
