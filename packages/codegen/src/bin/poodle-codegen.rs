//! `poodle-codegen` CLI — write mode (`ir:build`) and read-only check mode
//! (`ir:check`).
//!
//! ```text
//! poodle-codegen <FIXTURE> --out <DIR> [--check]
//! ```
//!
//! `--check` regenerates in memory and byte-compares against the committed
//! files under `--out`, failing on drift without writing (ruling R3). The
//! check branch is structurally incapable of writing: it calls
//! [`poodle_codegen::check_outputs`], which contains no write call, and the
//! write path lives in a separate function the check branch never reaches.
//!
//! Exit codes: 0 clean, 1 drift/validation/IO failure, 2 usage error.

use std::env;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use poodle_codegen::{
    check_outputs, generate, load_and_validate, targets, write_outputs, CodegenError,
};

fn usage() -> String {
    "usage: poodle-codegen <FIXTURE> --out <DIR> [--check]\n\
     \n\
     FIXTURE  repo-relative path of the serialized IrModel JSON (also carried\n\
              into every generated header as the authored source path).\n\
     --out    directory committed artifacts are written to / compared against;\n\
              each target owns a subdirectory below it.\n\
     --check  compare instead of write; never mutates the worktree."
        .to_owned()
}

struct Args {
    fixture: PathBuf,
    out: PathBuf,
    check: bool,
}

fn parse_args() -> Result<Args, String> {
    let mut out = None;
    let mut check = false;
    let mut positional = Vec::new();

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--check" => check = true,
            "--out" => {
                out = Some(PathBuf::from(
                    args.next().ok_or("--out requires a directory argument")?,
                ));
            }
            "--help" | "-h" => return Err(usage()),
            flag if flag.starts_with("--") => return Err(format!("unknown flag '{flag}'")),
            _ => positional.push(PathBuf::from(arg)),
        }
    }

    if positional.len() != 1 {
        return Err("expected exactly one FIXTURE argument".to_owned());
    }
    let fixture = positional.pop().expect("length checked above");
    let out = out.ok_or("--out is required")?;

    Ok(Args {
        fixture,
        out,
        check,
    })
}

fn run(args: &Args) -> Result<(), CodegenError> {
    let model = load_and_validate(&args.fixture)?;

    let targets = targets::all();
    let mut any_written = false;

    for target in targets {
        let source_path = fixture_source_path(&args.fixture);
        let files = generate(&model, &source_path, target)?;
        let root = args.out.join(target.output_root());

        if args.check {
            let report = check_outputs(&root, &files)?;
            if !report.is_clean() {
                return Err(CodegenError::Gate {
                    message: format!(
                        "generated {} artifacts are stale under {}:\n{}",
                        target.id(),
                        root.display(),
                        report.message()
                    ),
                });
            }
            println!(
                "Verified {} files (target: {}, IR schema {}).",
                files.len(),
                target.id(),
                poodle_ir::IR_SCHEMA_VERSION
            );
        } else {
            write_outputs(&root, &files)?;
            any_written = true;
            println!(
                "Generated {} files (target: {}, IR schema {}).",
                files.len(),
                target.id(),
                poodle_ir::IR_SCHEMA_VERSION
            );
        }
    }

    if args.check && !any_written {
        println!("All generated artifacts are current.");
    }
    Ok(())
}

/// The source path carried into headers: the fixture argument verbatim.
/// The selector passes a repo-relative path, so the header is portable
/// across machines — no absolute path is ever derived here.
fn fixture_source_path(fixture: &Path) -> String {
    fixture.to_string_lossy().into_owned()
}

fn main() -> ExitCode {
    let args = match parse_args() {
        Ok(args) => args,
        Err(message) => {
            eprintln!("error: {message}\n\n{}", usage());
            return ExitCode::from(2);
        }
    };

    match run(&args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}
