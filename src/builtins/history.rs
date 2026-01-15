use anyhow::Result;

use super::{ShellCommand, ShellCtx, ShellError};

#[derive(Debug)]
pub struct History;

impl ShellCommand for History {
    fn name(&self) -> &'static str {
        "history"
    }

    fn execute(&self, args: &Vec<String>, ctx: &mut ShellCtx) -> Result<Option<String>> {
        let mut args = args.iter().to_owned();

        let hist: Vec<String> = match args.next() {
            Some(skip) if skip.parse::<usize>().is_ok() => {
                let entries: Vec<String> = ctx
                    .get_history()
                    .iter()
                    .enumerate()
                    .map(|(idx, entry)| format!("    {}  {}", idx + 1, entry))
                    .collect();

                let skip = entries.len() - skip.parse::<usize>().unwrap();
                entries.into_iter().skip(skip).collect()
            }
            Some(flag) if flag == "-r" => {
                let Some(path) = args.next() else {
                    return Err(ShellError::MissingArg.into());
                };
                ctx.set_read_history(path)?;
                return Ok(None);
            }
            Some(flag) if flag == "-w" => {
                let Some(path) = args.next() else {
                    return Err(ShellError::MissingArg.into());
                };
                ctx.set_write_history(path);
                return Ok(None);
            }
            Some(flag) if flag == "-a" => {
                let Some(path) = args.next() else {
                    return Err(ShellError::MissingArg.into());
                };
                ctx.set_append_history(path);
                return Ok(None);
            }
            Some(_) => Vec::new(),
            None => ctx
                .get_history()
                .iter()
                .enumerate()
                .map(|(idx, entry)| format!("    {}  {}", idx + 1, entry))
                .collect(),
        };

        println!("{}", hist.join("\n"));

        Ok(None)
    }
}
