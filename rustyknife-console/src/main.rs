use rustyknife_z::*;
use std::fs;
use std::io::{BufRead, Write};
use std::path::PathBuf;
use structopt::StructOpt;

// TODO reinstate
//     fn next_instr(&mut self, pc: Address, call_stack_depth: usize, instr: &Instruction) {
//         if self.trace {
//             eprintln!("{:6}  {}{:?}", pc, "  ".repeat(call_stack_depth), instr);
//         }
//     }

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
#[allow(dead_code)]
struct Options {
    #[structopt(name = "FILE", parse(from_os_str))]
    story_file: PathBuf,
    #[structopt(
        short = "t",
        long = "trace",
        help = "print Z-machine instructions to stderr as they are executed"
    )]
    trace: bool,
}

fn run() -> i32 {
    let opts = Options::from_args();

    let story_file = fs::read(&opts.story_file)
        .unwrap_or_else(|_| panic!("could not read game file {:?}", &opts.story_file));

    // TODO put behind a flag
    // let bytes = Bytes::from(data);
    // let mem = Memory::wrap(bytes)
    //     .expect(&format!("error in story file {:?}", &opts.story_file));
    // print!("{:}", mem.obj_table().to_tree_string().unwrap());

    let stdin = std::io::stdin();
    let mut input = stdin.lock();

    let mut z = ZMachine::new(story_file)
        .unwrap_or_else(|_| panic!("error in story file {:?}", &opts.story_file));

    let mut continuation = z.start();
    loop {
        match continuation {
            Ok(cont) => match cont {
                Continuation::Step(next) => {
                    continuation = next();
                }
                Continuation::UpdateStatusLine(status_line, next) => {
                    // 8.2.2.2
                    // If the object's short name exceeds the available room on the status line, the author
                    // suggests that an interpreter should break it at the last space and append an ellipsis
                    // "...". There is no guaranteed maximum length for location names but an interpreter
                    // should expect names of length up to at least 49 characters.
                    print!("{:49}  ", status_line.location);
                    match status_line.progress {
                        Progress::Score { score, turns } => {
                            println!("Score: {:3}  Turns: {:4}", score, turns)
                        }
                        Progress::Time { hours, minutes } => {
                            println!("Time:  {}:{:02}", hours, minutes)
                        }
                    }
                    continuation = next();
                }
                Continuation::Print(string, next) => {
                    // Note that stdout is typically line-buffered, but we flush it in read_line().
                    print!("{}", string);
                    continuation = next();
                }
                Continuation::ReadLine(next) => {
                    std::io::stdout().flush().unwrap();
                    let mut buf = String::new();
                    input.read_line(&mut buf).unwrap();
                    // Remove trailing newline.
                    buf.pop().expect("unexpected EOF on stdin");
                    continuation = next(&buf);
                }
                Continuation::Quit => {
                    return 0;
                }
            },
            Err(err) => {
                eprintln!("Interpreter error: {}", err);
                return 1;
            }
        }
    }
}

fn main() {
    std::process::exit(run());
}
