pub struct Post {
    state: Option<Box<State>>,
    content: String,
    approvable: bool,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
            approvable: false,
        }
    }

    pub fn add_text(&mut self, text: &str) -> Result<(), String> {
        if self.state() == "draft" {
            self.content.push_str(text);
            Ok(())
        } else {
            Err(format!(
                "can't add text to the post in {} state",
                self.state()
            ))
        }
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)
    }

    pub fn state(&self) -> &'static str {
        self.state.as_ref().unwrap().state()
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }

    pub fn approve(&mut self) {
        if self.approvable {
            if let Some(s) = self.state.take() {
                self.state = Some(s.approve());
            }
        } else {
            self.approvable = true;
        }
    }

    pub fn reject(&mut self) {
        self.approvable = false;

        if let Some(s) = self.state.take() {
            self.state = Some(s.reject());
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<State>;
    fn approve(self: Box<Self>) -> Box<State>;
    fn reject(self: Box<Self>) -> Box<State>;
    fn state(&self) -> &'static str;

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        let _ = post;
        ""
    }
}

struct Draft;

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<State> {
        Box::new(PendingReview {})
    }

    fn state(&self) -> &'static str {
        "draft"
    }

    fn approve(self: Box<Self>) -> Box<State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<State> {
        self
    }
}

struct PendingReview;

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<State> {
        self
    }

    fn state(&self) -> &'static str {
        "pending"
    }

    fn approve(self: Box<Self>) -> Box<State> {
        Box::new(Published {})
    }

    fn reject(self: Box<Self>) -> Box<State> {
        Box::new(Draft {})
    }
}

struct Published;

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }

    fn state(&self) -> &'static str {
        "published"
    }

    fn approve(self: Box<Self>) -> Box<State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<State> {
        self
    }
}
