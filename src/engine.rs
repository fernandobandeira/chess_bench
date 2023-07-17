use std::{
    io::{BufRead, BufReader, Write},
    process::{Child, ChildStdin, ChildStdout, Command, Stdio},
};

pub struct Engine {
    pub process: Child,
    pub stdin: ChildStdin,
    pub stdout: BufReader<ChildStdout>,
}

impl Engine {
    pub fn new(program: &str) -> Engine {
        // Start a process and send "uci" to the stdin of the process
        // Then read the output of the process
        let mut process = Command::new(program)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start Engine");
    
        let stdin = process.stdin.take().unwrap();
        let stdout = BufReader::new(process.stdout.take().unwrap());
    
        let mut engine = Engine {
            process,
            stdin,
            stdout,
        };
        engine.configure();
    
        return engine;
    }

    pub fn calculate_move(&mut self, fen: &str) -> String {
        // Send "position fen <fen>" to the engine
        self.send_command(&format!("position fen {}\n", fen));
        // Send "go movetime 3000" to the engine
        self.send_command("go movetime 3000\n");
        // Wait till the engine sends "bestmove <move>"
        let response = self.wait_for_response("bestmove ");
        // Return the move
        return response[9..13].to_string();
    }

    pub fn stop(&mut self) {
        // Send "stop" to the engine
        self.send_command("quit\n");

        self.process.kill().expect("Failed to kill Engine");
    }

    fn configure(&mut self) {
        // Send "uci" command to the engine
        self.send_command("uci\n");
        // Wait till the engine sends "uciok" to indicate that it has finished
        // loading and has sent all the info we need
        self.wait_for_response("uciok\n");

        // Send "isready" to check if the engine is ready
        self.send_command("isready\n");
        // Wait till the engine sends "readyok"
        self.wait_for_response("readyok\n");

        // Send "ucinewgame" to indicate that we are starting a new game
        self.send_command("ucinewgame\n");
        // Send "isready" to check if the engine is ready
        self.send_command("isready\n");
        // Wait till the engine sends "readyok"
        self.wait_for_response("readyok\n");
    }

    fn send_command(&mut self, command: &str) {
        self.stdin.write_all(command.as_bytes()).unwrap();
        self.stdin.flush().unwrap();
    }

    fn wait_for_response(&mut self, response: &str) -> String {
        let mut buffer = String::new();
        while self.stdout.read_line(&mut buffer).unwrap() > 0 {
            if buffer.starts_with(response) {
                break;
            }
            buffer.clear();
        }

        return buffer;
    }
}
