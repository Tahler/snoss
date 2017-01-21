use std::io;
use std::io::BufRead;
use std::io::Write;

#[derive(Debug)]
struct Shell<R: BufRead, W: Write> {
    prompt: String,
    input: R,
    output: W,
}

impl<R: BufRead, W: Write> Shell<R, W> {
    pub fn new(prompt: String, input: R, output: W) -> Self {
        Shell {
            prompt: prompt,
            input: input,
            output: output,
        }
    }

    pub fn start(&mut self) {
        loop {
            self.print_prompt();
            let cmd = self.read_line();
            // eval
            // let (stdout, stderr) = self.eval()
            // print
            // loop
        }
    }

    fn print_prompt(&mut self) {
        self.output.write_all(self.prompt.as_bytes());
    }

    fn read_line(&mut self) -> String {
        let mut input_text = String::new();
        self.input
            .read_line(&mut input_text)
            .expect("failed to read from stdin");
        let trimmed = input_text.trim();
        trimmed.to_string()
    }
}
