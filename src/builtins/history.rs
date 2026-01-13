use anyhow::Result;

use std::env;

use super::{ShellCommand, ShellCtx, writer};

#[derive(Debug)]
pub struct History;

impl ShellCommand for History {
    fn name(&self) -> &'static str {
        "history"
    }

    fn execute(&self, args: &Vec<String>, ctx: &mut ShellCtx) -> Result<Option<String>> {
        // let histfile = get_histfile();
        // let file = fs::File::open(histfile)?;
        // let reader = io::BufReader::new(file);

        // let take_last = args.first().and_then(|arg| arg.parse::<usize>().ok());

        // let hist: Vec<String> = match take_last {
        //     Some(skip) => {
        //         let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
        //         let skip = lines.len() - skip;
        //         lines.into_iter().skip(skip).collect()
        //     }
        //     None => reader.lines().collect::<Result<_, _>>()?,
        // };

        // println!("    {}", hist.join("\n    "));
        // Ok(None)

        let take_last = args.first().and_then(|arg| arg.parse::<usize>().ok());

        let entries: Vec<String> = ctx
            .history
            .iter()
            .enumerate()
            .map(|(idx, entry)| format!("    {}  {}", idx + 1, entry))
            .collect();

        let hist = match take_last {
            Some(skip) => {
                let skip = entries.len() - skip;
                entries.into_iter().skip(skip).collect()
            }
            None => entries,
        };

        println!("    {}", hist.join("\n    "));

        Ok(None)
    }
}

pub fn handle_history(line: &str) -> Result<()> {
    let histfile = get_histfile();

    writer::write_file(&histfile, line, &true)?;
    Ok(())
}

fn get_histfile() -> String {
    match env::var("HISTFILE") {
        Ok(hist) => hist,
        Err(_) => String::from("history.txt"),
    }
}
