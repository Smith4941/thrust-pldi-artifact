//! `scalability`: the lines of code of the RQ3 programs (Table tab:scalability).
//!
//! A program's code lines are the lines of its items that are not blank, not a `//` comment, not
//! an attribute and not a spec line by the rules of `creusot`, where an `assert!` call is code.
//! The output has one row per program `*.rs` of PROGRAMS_DIR, named by its file stem:
//! `id,code_lines`.

use std::error::Error;

use syn::spanned::Spanned;
use syn::visit::Visit;

use crate::creusot::Spec;
use crate::{blank, lines, parse};

/// The rows of code-lines.csv, the programs `*.rs` of `dir` by name.
pub fn run(dir: &str) -> Result<(), Box<dyn Error>> {
    let mut programs: Vec<std::path::PathBuf> =
        std::fs::read_dir(dir)?.map(|e| e.map(|e| e.path())).collect::<Result<_, _>>()?;
    programs.retain(|p| p.extension().is_some_and(|e| e == "rs"));
    programs.sort();
    println!("id,code_lines");
    for path in programs {
        let (source, file) = parse(&path.to_string_lossy())?;
        let mut spec = Spec::new(&file);
        spec.visit_file(&file);
        let text: Vec<&str> = source.lines().collect();
        let mut code: Vec<usize> = file.items.iter().flat_map(|i| lines(i.span())).collect();
        code.sort();
        code.dedup();
        code.retain(|&n| !blank(text[n - 1]));
        code.retain(|n| !spec.lines.contains(n) && !spec.attributes.contains(n));
        let id = path.file_stem().unwrap_or_default().to_string_lossy();
        println!("{id},{}", code.len());
    }
    Ok(())
}
