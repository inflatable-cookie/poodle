fn main() {
    println!("cargo:rerun-if-changed=Cargo.lock");
    if std::env::var_os("CARGO_FEATURE_WINDOW_CAPTURE").is_none() {
        return;
    }

    let lockfile = std::fs::read_to_string("Cargo.lock").expect("read the preview Cargo.lock");
    let mut gpui = lockfile.split("[[package]]").filter(|entry| {
        entry
            .lines()
            .any(|line| line == r#"name = "gpui-unofficial""#)
    });
    let entry = gpui
        .next()
        .expect("gpui-unofficial in the preview Cargo.lock");
    assert!(gpui.next().is_none(), "ambiguous GPUI receipt identity");
    assert!(
        entry
            .lines()
            .any(|line| line
                == r#"source = "registry+https://github.com/rust-lang/crates.io-index""#),
        "GPUI receipts require a crates.io resolution"
    );
    let version = entry
        .lines()
        .find_map(|line| line.strip_prefix(r#"version = ""#)?.strip_suffix('"'))
        .expect("the locked GPUI version");
    println!("cargo:rustc-env=POODLE_CAPTURE_GPUI_VERSION={version}");
}
