use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Response {
    question: String,
    decision: String,
    answers: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Node {
    question: Option<String>,
    decision: Option<String>,
    pub yes_node: Option<Box<Node>>,
    pub no_node: Option<Box<Node>>,
    pub likely_yes_node: Option<Box<Node>>,
    pub likely_no_node: Option<Box<Node>>,
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

    pub fn go_likely_yes(&mut self) -> Response {
        if let Some(likely_yes_node) = self.likely_yes_node.take() {
            *self = *likely_yes_node;
        }

        self.return_response()
    }

    pub fn go_likely_no(&mut self) -> Response {
        if let Some(likely_no_node) = self.likely_no_node.take() {
            *self = *likely_no_node;
        }

        self.return_response()
    }

    pub fn return_response(&self) -> Response {
        let mut result: Vec<String> = Vec::new();

        if self.yes_node.is_some() {
            result.push("Да".to_string())
        }

        if self.no_node.is_some() {
            result.push("Нет".to_string())
        }

        if self.likely_yes_node.is_some() {
            result.push("Скорее да".to_string())
        }

        if self.likely_no_node.is_some() {
            result.push("Скорее нет".to_string())
        }

        Response {
            decision: self.decision.clone().unwrap_or("".to_string()),
            question: self.question.clone().unwrap_or("".to_string()),
            answers: result,
        }
    }
}
