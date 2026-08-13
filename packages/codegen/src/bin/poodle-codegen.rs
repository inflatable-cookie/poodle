//! `poodle-codegen` CLI — write mode (`ir:build`) and read-only check mode
//! (`ir:check`).
//!
//! ```text
//! poodle-codegen <FIXTURE> --out <DIR> [--check] [--target <ID>]
//! poodle-codegen --author-shell <OUT> [--check]
//! ```
//!
//! Fixture mode runs the registered targets over one serialized `IrModel`.
//! `--check` regenerates in memory and byte-compares against the committed
//! files under `--out`, failing on drift without writing (ruling R3). The
//! check branch is structurally incapable of writing: it calls
//! [`poodle_codegen::check_outputs`], which contains no write call, and the
//! write path lives in a separate function the check branch never reaches.
//! `--target` restricts emission to one target (e.g. the scene-scoped
//! `shell-scene`, which renders into the consuming web packages and is not
//! part of the default set).
//!
//! Author mode (card 035 R1) serializes the Rust-authored shell model —
//! [`poodle_codegen::models::preview_shell::shell_model`] — to the JSON
//! fixture the pipeline consumes, after a validate round trip: the emitted
//! bytes are parsed and validated before they are written or compared, so
//! the committed fixture can never be invalid IR. `--check` byte-compares
//! against the committed fixture without writing.
//!
//! Exit codes: 0 clean, 1 drift/validation/IO failure, 2 usage error.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use poodle_codegen::{
    check_outputs, generate, load_and_validate, models, targets, write_outputs, CodegenError,
};

fn usage() -> String {
    "usage: poodle-codegen <FIXTURE> --out <DIR> [--check] [--target <ID>]\n\
     \n\
     FIXTURE  repo-relative path of the serialized IrModel JSON (also carried\n\
              into every generated header as the authored source path).\n\
     --out    directory committed artifacts are written to / compared against;\n\
              each target owns a subdirectory below it.\n\
     --check  compare instead of write; never mutates the worktree.\n\
     --target restrict emission to one target id (default: every registered
              target; the scene-scoped `shell-scene` and `shell-rust`
              targets are select-only).
     \n\
     usage: poodle-codegen --author-shell <OUT> [--check]\n\
     \n\
     --author-shell  serialize the Rust-authored shell model\n\
              (packages/codegen/src/models/preview_shell.rs) to OUT — the\n\
              fixture the pipeline consumes — after a validate round trip;\n\
              with --check, byte-compare instead of write."
        .to_owned()
}

struct Args {
    fixture: Option<PathBuf>,
    out: Option<PathBuf>,
    author_shell: Option<PathBuf>,
    target: Option<String>,
    check: bool,
}

fn parse_args() -> Result<Args, String> {
    let mut out = None;
    let mut author_shell = None;
    let mut target = None;
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
            "--author-shell" => {
                author_shell = Some(PathBuf::from(
                    args.next()
                        .ok_or("--author-shell requires an output path")?,
                ));
            }
            "--target" => {
                target = Some(
                    args.next()
                        .ok_or("--target requires a target id (see `effigy tasks` for the list)")?,
                );
            }
            "--help" | "-h" => return Err(usage()),
            flag if flag.starts_with("--") => return Err(format!("unknown flag '{flag}'")),
            _ => positional.push(PathBuf::from(arg)),
        }
    }

    let author_mode = author_shell.is_some();

    let fixture = match (author_mode, positional.len()) {
        (true, 0) => None,
        (true, _) => {
            return Err(
                "author mode takes no positional FIXTURE argument; run author mode alone"
                    .to_owned(),
            );
        }
        (false, 1) => Some(positional.pop().expect("length checked above")),
        (false, 0) => return Err("expected a FIXTURE argument or --author-*".to_owned()),
        (false, _) => return Err("expected exactly one FIXTURE argument".to_owned()),
    };

    if author_mode {
        if out.is_some() {
            return Err("--out is not valid with --author-*".to_owned());
        }
        if target.is_some() {
            return Err("--target is not valid with --author-*".to_owned());
        }
    } else if out.is_none() {
        return Err("--out is required".to_owned());
    }

    Ok(Args {
        fixture,
        out,
        author_shell,
        target,
        check,
    })
}

fn run(args: &Args) -> Result<(), CodegenError> {
    if let Some(out_path) = &args.author_shell {
        return if args.check {
            check_author_shell(out_path)
        } else {
            write_author_shell(out_path)
        };
    }
    run_emit(
        args.fixture.as_ref().expect("emit mode carries a fixture"),
        args,
    )
}

/// Serializes the Rust-authored shell model to the fixture after a validate
/// round trip (card 035 R1): the bytes written are exactly the bytes the
/// pipeline's `load_and_validate` will accept.
fn author_shell_document() -> Result<String, CodegenError> {
    let model = models::preview_shell::shell_model();
    let document = serde_json::to_string_pretty(&model).map_err(|error| CodegenError::Gate {
        message: format!("cannot serialize the authored shell model: {error}"),
    })?;
    // Validate the serialized form, not just the in-memory model: the
    // fixture is the pipeline's input, and it must pass `load_and_validate`.
    let round_tripped: poodle_ir::IrModel =
        serde_json::from_str(&document).map_err(|error| CodegenError::Gate {
            message: format!("authored shell model does not round-trip as JSON: {error}"),
        })?;
    let findings = round_tripped.validate();
    if !findings.is_empty() {
        return Err(CodegenError::Invalid {
            path: PathBuf::from("packages/codegen/fixtures/shell-model.json"),
            findings,
        });
    }
    Ok(format!("{document}\n"))
}

fn write_author_shell(out_path: &Path) -> Result<(), CodegenError> {
    let document = author_shell_document()?;
    if let Some(parent) = out_path.parent() {
        fs::create_dir_all(parent).map_err(|error| CodegenError::Write {
            path: parent.to_path_buf(),
            source: error,
        })?;
    }
    fs::write(out_path, &document).map_err(|error| CodegenError::Write {
        path: out_path.to_path_buf(),
        source: error,
    })?;
    println!(
        "Authored shell model ({} bytes, IR schema {}).",
        document.len(),
        poodle_ir::IR_SCHEMA_VERSION
    );
    Ok(())
}

/// Read-only twin of [`write_author_shell`]: regenerate in memory and
/// byte-compare against the committed fixture. No write call exists on this
/// path.
fn check_author_shell(out_path: &Path) -> Result<(), CodegenError> {
    let document = author_shell_document()?;
    let committed = fs::read_to_string(out_path).map_err(|error| CodegenError::Read {
        path: out_path.to_path_buf(),
        source: error,
    })?;
    if committed == document {
        println!("Authored shell model is current.");
        Ok(())
    } else {
        Err(CodegenError::Gate {
            message: format!(
                "authored shell model is stale under {}: the committed fixture differs from \
                 `packages/codegen/src/models/preview_shell.rs`; run `effigy ir:build`",
                out_path.display()
            ),
        })
    }
}

fn run_emit(fixture: &Path, args: &Args) -> Result<(), CodegenError> {
    let model = load_and_validate(fixture)?;

    let selected = match &args.target {
        Some(id) => vec![targets::by_id(id).ok_or_else(|| {
            let known = targets::selectable()
                .iter()
                .map(|target| target.id().to_owned())
                .collect::<Vec<String>>();
            CodegenError::UnknownTarget {
                id: id.clone(),
                known,
            }
        })?],
        None => targets::all(),
    };
    let out = args.out.as_ref().expect("emit mode always carries --out");

    let mut any_written = false;

    for target in selected {
        let source_path = fixture_source_path(fixture);
        let files = generate(&model, &source_path, target)?;
        let root = out.join(target.output_root());

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
            eprintln!("{message}");
            eprintln!();
            eprintln!("{}", usage());
            return ExitCode::from(2);
        }
    };

    match run(&args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
