pub struct CommentCommand {
    pub command: String,
    pub argument: String,
}

impl CommentCommand {
    pub fn get_commands(comment: &str) -> Vec<CommentCommand> {
        if !comment.contains("seeql-") {
            return Vec::new();
        }

        Vec::new()
    }
}
