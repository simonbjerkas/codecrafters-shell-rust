use anyhow::Result;

use super::{ShellCommand, ShellCtx};

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
                    .history
                    .iter()
                    .enumerate()
                    .map(|(idx, entry)| format!("    {}  {}", idx + 1, entry))
                    .collect();

                let skip = entries.len() - skip.parse::<usize>().unwrap();
                entries.into_iter().skip(skip).collect()
            }
            Some(flag) if flag == "-r" => {
                let path = args.next().unwrap();
                ctx.set_histfile(path.to_string())?;
                return Ok(None);
            }
            Some(_) => Vec::new(),
            None => ctx
                .history
                .iter()
                .enumerate()
                .map(|(idx, entry)| format!("    {}  {}", idx + 1, entry))
                .collect(),
        };

        println!("    {}", hist.join("\n    "));

        Ok(None)
    }
}
