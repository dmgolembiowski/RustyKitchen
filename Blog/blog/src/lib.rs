
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    // Takes a mutable reference to `self` because
    // we're changing the Post instance that we're calling
    //  `add_text` on.
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
        /* We then call `push_str` on the `String` in
         * `content` and pass the `text` arg to add to the
         * saved `content`. This behavior doesn't depend on the state
         * that the post is in, so it's not part of the state pattern.
         * 
         * The `add_text` method doesn't interact with the `state`
         * field at all, but it is part of the behavior we want to have.
         * */
        // Note that still want content to be an empty str since it 
        // should still be `PendingReview`
    }

    pub fn content(&self) -> &str {
        self.state.as_ref()
            .unwrap()
            .content(&self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}


/// Defines the behavior shared by different post states,
/// and the `Draft`, `PendingReview`, and `Published` states
/// will all implement the `State` trait.
trait State {
    /* Note: this top bit with:
     *      `self: Box<Self>`
     * means that the method is only valid when called on
     * a `Box` holding the type. It also takes ownership of
     * `Box<Self>`, so as to invalidate the old state so the 
     * state value of the `Post` can transform into a new state.
     */
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str { "" }
}

struct Draft {}

struct PendingReview {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self 
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self 
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    
    /// If we call `approve` on a `Draft`, it will have no
    /// effect because it will return `self`.
    /// When we call `approve` on a `PendingReview`, it re-
    /// turns a new, boxed instance of the `Published` struct.
    ///
    /// The `Published` struct implements the `State` trait,
    /// and for both the `request_review` method and the 
    /// `approve` method, it returns itself, because the post
    /// should stay in the `Published` state in those cases.
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
