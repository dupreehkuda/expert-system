use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Response {
    question: String,
    decision: String,
}

#[derive(Serialize, Deserialize)]
pub struct Node {
    question: Option<String>,
    decision: Option<String>,
    pub yes_node: Option<Box<Node>>,
    pub no_node: Option<Box<Node>>,
}

impl Node {
    pub fn go_yes(&mut self) -> Response {
        if let Some(yes_node) = self.yes_node.take() {
            *self = *yes_node;
        }

        self.return_response()
    }

    pub fn go_no(&mut self) -> Response {
        if let Some(no_node) = self.no_node.take() {
            *self = *no_node;
        }

        self.return_response()
    }

    pub fn return_response(&self) -> Response {
        Response {
            decision: self.decision.clone().unwrap_or("".to_string()),
            question: self.question.clone().unwrap_or("".to_string())
        }
    }
}
