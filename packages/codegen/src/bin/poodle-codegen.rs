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
    check_outputs, conformance, generate, load_and_validate, machine_interfaces, models, targets,
    write_outputs, CodegenError,
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
              with --check, byte-compare instead of write.\n\
     \n\
     usage: poodle-codegen --author-specimens <OUT> [--check]\n\
     \n\
     --author-specimens  serialize the Rust-authored display-specimen\n\
              scenes (packages/codegen/src/models/display_specimens.rs) to\n\
              OUT — the fixture the specimen targets consume — after a\n\
              validate round trip; with --check, byte-compare instead of\n\
              write.\n\
     \n\
     usage: poodle-codegen --machine-interfaces <FILE> --out <DIR> --target <machine-ts|machine-rust> [--check]\n\
     \n\
     --machine-interfaces  schema of machine states, events, effects, and\n\
               context types (spec 064 mechanism 1). Not an IrModel; the\n\
               machine targets are select-only.\n\
     \n\
     usage: poodle-codegen --conformance <INTERFACE> --cases <CORPUS> --out <DIR> --target <conformance-rust|conformance-cases> [--check]\n\
     \n\
     --conformance  serialized portable interface (spec 066), authored in\n\
               TypeScript and emitted as JSON by the conformance serializer.\n\
               `conformance-rust` renders the portable declaration into the\n\
               consuming crate's generated tree; `conformance-cases` copies\n\
               the interface and corpus JSON into a native preview's\n\
               generated tree. Select-only, like the machine targets."
        .to_owned()
}

struct Args {
    fixture: Option<PathBuf>,
    out: Option<PathBuf>,
    author_shell: Option<PathBuf>,
    author_specimens: Option<PathBuf>,
    machine_interfaces: Option<PathBuf>,
    conformance_interface: Option<PathBuf>,
    conformance_cases: Option<PathBuf>,
    target: Option<String>,
    check: bool,
}

fn parse_args() -> Result<Args, String> {
    let mut out = None;
    let mut author_shell = None;
    let mut author_specimens = None;
    let mut machine_interfaces = None;
    let mut conformance_interface = None;
    let mut conformance_cases = None;
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
            "--author-specimens" => {
                author_specimens = Some(PathBuf::from(
                    args.next()
                        .ok_or("--author-specimens requires an output path")?,
                ));
            }
            "--machine-interfaces" => {
                machine_interfaces = Some(PathBuf::from(
                    args.next()
                        .ok_or("--machine-interfaces requires a schema path")?,
                ));
            }
            "--conformance" => {
                conformance_interface = Some(PathBuf::from(
                    args.next()
                        .ok_or("--conformance requires an interface path")?,
                ));
            }
            "--cases" => {
                conformance_cases = Some(PathBuf::from(
                    args.next()
                        .ok_or("--cases requires a corpus path")?,
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

    let author_mode = author_shell.is_some() || author_specimens.is_some();
    let machine_mode = machine_interfaces.is_some();
    let conformance_mode = conformance_interface.is_some() || conformance_cases.is_some();

    if conformance_mode && (author_mode || machine_mode) {
        return Err("run --conformance separately from --author-* and --machine-interfaces".to_owned());
    }

    if machine_mode {
        if !positional.is_empty() {
            return Err(
                "machine-interface mode takes no positional FIXTURE argument".to_owned(),
            );
        }
        if out.is_none() {
            return Err("--out is required with --machine-interfaces".to_owned());
        }
        match target.as_deref() {
            Some(targets::machine_ts::ID) | Some(targets::machine_rust::ID) => {}
            Some(id) => {
                return Err(format!(
                    "machine-interface --target must be machine-ts or machine-rust, not '{id}'"
                ));
            }
            None => {
                return Err(
                    "--target is required with --machine-interfaces (machine-ts or machine-rust)"
                        .to_owned(),
                );
            }
        }
        return Ok(Args {
            fixture: None,
            out,
            author_shell: None,
            author_specimens: None,
            machine_interfaces,
            conformance_interface: None,
            conformance_cases: None,
            target,
            check,
        });
    }

    if conformance_mode {
        let (interface, cases) = match (conformance_interface, conformance_cases) {
            (Some(interface), Some(cases)) => (interface, cases),
            _ => {
                return Err(
                    "--conformance requires both --conformance <interface> and --cases <corpus>"
                        .to_owned(),
                );
            }
        };
        if !positional.is_empty() {
            return Err("conformance mode takes no positional FIXTURE argument".to_owned());
        }
        if out.is_none() {
            return Err("--out is required with --conformance".to_owned());
        }
        match target.as_deref() {
            Some(targets::conformance_rust::ID) | Some(targets::conformance_cases::ID) => {}
            Some(id) => {
                return Err(format!(
                    "conformance --target must be conformance-rust or conformance-cases, not '{id}'"
                ));
            }
            None => {
                return Err(
                    "--target is required with --conformance (conformance-rust or conformance-cases)"
                        .to_owned(),
                );
            }
        }
        return Ok(Args {
            fixture: None,
            out,
            author_shell: None,
            author_specimens: None,
            machine_interfaces: None,
            conformance_interface: Some(interface),
            conformance_cases: Some(cases),
            target,
            check,
        });
    }

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
        author_specimens,
        machine_interfaces,
        conformance_interface: None,
        conformance_cases: None,
        target,
        check,
    })
}

fn run(args: &Args) -> Result<(), CodegenError> {
    if let Some(out_path) = &args.author_shell {
        let model = models::preview_shell::shell_model();
        return if args.check {
            check_author_model(out_path, &model, "shell")
        } else {
            write_author_model(out_path, &model, "shell")
        };
    }
    if let Some(out_path) = &args.author_specimens {
        let model = models::display_specimens::display_specimens_model();
        return if args.check {
            check_author_model(out_path, &model, "display-specimens")
        } else {
            write_author_model(out_path, &model, "display-specimens")
        };
    }
    if let Some(schema) = &args.machine_interfaces {
        return run_machine_interfaces(schema, args);
    }
    if let (Some(interface), Some(cases)) = (&args.conformance_interface, &args.conformance_cases) {
        return run_conformance(interface, cases, args);
    }
    run_emit(
        args.fixture.as_ref().expect("emit mode carries a fixture"),
        args,
    )
}

fn run_machine_interfaces(schema: &Path, args: &Args) -> Result<(), CodegenError> {
    let document = machine_interfaces::load_and_validate(schema)?;
    let source_path = schema.to_string_lossy().into_owned();
    let target_id = args
        .target
        .as_deref()
        .expect("parse requires --target in machine-interface mode");
    let (files, output_root) = match target_id {
        targets::machine_ts::ID => (
            targets::machine_ts::render(&document, &source_path),
            targets::machine_ts::OUTPUT_ROOT,
        ),
        targets::machine_rust::ID => (
            targets::machine_rust::render(&document, &source_path),
            targets::machine_rust::OUTPUT_ROOT,
        ),
        other => {
            return Err(CodegenError::UnknownTarget {
                id: other.to_owned(),
                known: vec![
                    targets::machine_ts::ID.to_owned(),
                    targets::machine_rust::ID.to_owned(),
                ],
            });
        }
    };
    let out = args.out.as_ref().expect("parse requires --out");
    let root = out.join(output_root);

    if args.check {
        let report = check_outputs(&root, &files)?;
        if !report.is_clean() {
            return Err(CodegenError::Gate {
                message: format!(
                    "generated machine-interface artifacts are stale under {} (target {target_id}):\n{}",
                    root.display(),
                    report.message()
                ),
            });
        }
        println!(
            "Verified {} files (target: {target_id}, machine interface schema {}).",
            files.len(),
            document.schema_version
        );
    } else {
        write_outputs(&root, &files)?;
        println!(
            "Generated {} files (target: {target_id}, machine interface schema {}).",
            files.len(),
            document.schema_version
        );
    }
    Ok(())
}

/// Conformance mode (spec 066): renders the portable declaration into a
/// consuming crate (`conformance-rust`) or copies the serialized interface
/// and corpus into a native preview (`conformance-cases`). Same check/write
/// split as the other modes; `--check` never writes.
fn run_conformance(
    interface_path: &Path,
    cases_path: &Path,
    args: &Args,
) -> Result<(), CodegenError> {
    let interface = conformance::load_interface(interface_path)?;
    conformance::load_cases(cases_path)?;
    let source_path = interface_path.to_string_lossy().into_owned();
    let target_id = args
        .target
        .as_deref()
        .expect("parse requires --target in conformance mode");
    let (files, output_root) = match target_id {
        targets::conformance_rust::ID => (
            targets::conformance_rust::render(&interface, &source_path)?,
            targets::conformance_rust::OUTPUT_ROOT,
        ),
        targets::conformance_cases::ID => (
            targets::conformance_cases::render(interface_path, cases_path)?,
            targets::conformance_cases::OUTPUT_ROOT,
        ),
        other => {
            return Err(CodegenError::UnknownTarget {
                id: other.to_owned(),
                known: vec![
                    targets::conformance_rust::ID.to_owned(),
                    targets::conformance_cases::ID.to_owned(),
                ],
            });
        }
    };
    let out = args.out.as_ref().expect("parse requires --out");
    let root = out.join(output_root);

    if args.check {
        let report = check_outputs(&root, &files)?;
        if !report.is_clean() {
            return Err(CodegenError::Gate {
                message: format!(
                    "generated conformance artifacts are stale under {} (target {target_id}):\n{}",
                    root.display(),
                    report.message()
                ),
            });
        }
        println!(
            "Verified {} files (target: {target_id}, interface {}).",
            files.len(),
            interface.id
        );
    } else {
        write_outputs(&root, &files)?;
        println!(
            "Generated {} files (target: {target_id}, interface {}).",
            files.len(),
            interface.id
        );
    }
    Ok(())
}

/// Serializes a Rust-authored model to the fixture after a validate round
/// trip (card 035 R1): the bytes written are exactly the bytes the
/// pipeline's `load_and_validate` will accept.
fn author_document(model: &poodle_ir::IrModel, label: &str) -> Result<String, CodegenError> {    let document = serde_json::to_string_pretty(model).map_err(|error| CodegenError::Gate {
        message: format!("cannot serialize the authored {label} model: {error}"),
    })?;
    // Validate the serialized form, not just the in-memory model: the
    // fixture is the pipeline's input, and it must pass `load_and_validate`.
    let round_tripped: poodle_ir::IrModel =
        serde_json::from_str(&document).map_err(|error| CodegenError::Gate {
            message: format!("authored {label} model does not round-trip as JSON: {error}"),
        })?;
    let findings = round_tripped.validate();
    if !findings.is_empty() {
        return Err(CodegenError::Invalid {
            path: PathBuf::from(format!("packages/codegen/fixtures/{label}-model.json")),
            findings,
        });
    }
    Ok(format!("{document}\n"))
}

fn write_author_model(out_path: &Path, model: &poodle_ir::IrModel, label: &str) -> Result<(), CodegenError> {
    let document = author_document(model, label)?;
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
        "Authored {label} model ({} bytes, IR schema {}).",
        document.len(),
        poodle_ir::IR_SCHEMA_VERSION
    );
    Ok(())
}

/// Read-only twin of [`write_author_model`]: regenerate in memory and
/// byte-compare against the committed fixture. No write call exists on this
/// path.
fn check_author_model(out_path: &Path, model: &poodle_ir::IrModel, label: &str) -> Result<(), CodegenError> {
    let document = author_document(model, label)?;
    let committed = fs::read_to_string(out_path).map_err(|error| CodegenError::Read {
        path: out_path.to_path_buf(),
        source: error,
    })?;
    if committed == document {
        println!("Authored {label} model is current.");
        Ok(())
    } else {
        Err(CodegenError::Gate {
            message: format!(
                "authored {label} model is stale under {}: the committed fixture differs from \
                 the model source; run `effigy ir:build`",
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
