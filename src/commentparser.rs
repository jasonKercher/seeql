pub struct CommentCommand {
    pub cmd: String,
    pub arg: String,
}

impl CommentCommand {
    pub fn get_commands(comment: &str) -> Vec<CommentCommand> {
        let mut comment = String::from(comment);
        let mut ret = Vec::new();

        loop {
            let index = comment.find("seeql-");
            if index.is_none() {
                break;
            }

            let start_index = index.unwrap() as usize + 6;
            comment = (&comment[start_index..]).to_string();
            let end_index = comment.find('\n');

            let end_index = match end_index {
                None => comment.len() - start_index,
                Some(x) => x as usize,
            };

            let line = &comment[..end_index].to_string();

            let cmd_end = match line.find(' ') {
                None => line.len(),
                Some(x) => x,
            };

            let cmd = &line[..cmd_end];

            let arg = if cmd_end == line.len() {
                ""
            } else {
                &line[cmd_end + 1..]
            };

            ret.push(CommentCommand {
                cmd: String::from(cmd),
                arg: String::from(arg),
            });

            comment = (&comment[end_index..]).to_string();
        }

        ret
    }
}
