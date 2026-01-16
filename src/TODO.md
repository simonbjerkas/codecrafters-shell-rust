# TODO:
- History function prints rather thyan returns which would break the pipeline. 
  - The termion use needs to print newlines approprialetly, right now it it prints newlines from the same cursor position as where the last line ended
  - Then return ExecResult::Res in the history builtin and it should all be good to go
  
## Done :D
Could probably be rewriteen and cleaned up, but as a first rust project, I am satisfied with the result.
