use poodle_conformance::{cases, Backend, Identity, Probe, Reading, Receipt, Status};
use poodle_node::{Node, NodeKind};
use serde_json::{json, Value};

#[derive(Default)]
struct Fake {
    node: Node,
    drop_label: bool,
    ring_as_bar: bool,
}

impl Backend for Fake {
    fn mount(&mut self, root: &Node, _: [f32; 2]) {
        self.node = root.children[0].clone();
    }

    fn read(&mut self, _: &str, probe: Probe) -> Reading {
        let observation = match probe {
            Probe::A11y("label") => {
                json!(self.node.a11y.label.as_deref().filter(|_| !self.drop_label))
            }
            Probe::Paint { point: [x, y], rgb } => {
                let NodeKind::ProgressRing { fraction } = self.node.kind else {
                    panic!("expected a progress ring fixture");
                };
                let radius = (x - 12.0).hypot(y - 12.0);
                let track = (10.0..=12.0).contains(&radius);
                let angle = (x - 12.0).atan2(12.0 - y).rem_euclid(std::f32::consts::TAU);
                json!(match rgb {
                    [255, 0, 0] if self.ring_as_bar => x <= 24.0 * fraction,
                    [255, 0, 0] => track && angle <= std::f32::consts::TAU * fraction,
                    [0, 0, 255] => track,
                    _ => false,
                })
            }
            _ => return Reading::Unsupported("fixture fake only implements two probes".into()),
        };
        Reading::Observed(observation)
    }
}

#[test]
fn missing_accessibility_property_changes_pass_to_fail() {
    let case = cases()
        .find(|case| case.id == "a11y.label")
        .expect("label fixture");
    let mut backend = Fake::default();
    assert_eq!(case.run(&mut backend).status, Status::Pass);
    backend.drop_label = true;
    let result = case.run(&mut backend);
    assert_eq!(result.status, Status::Fail);
    assert_eq!(result.observation[0]["actual"], Value::Null);
    assert_eq!(result.observation[0]["expected"], "Run report");
}

#[test]
fn a_bar_cannot_pass_for_a_quarter_ring() {
    let case = cases()
        .find(|case| case.id == "kind.ProgressRing")
        .expect("ring fixture");
    let mut backend = Fake::default();
    assert_eq!(case.run(&mut backend).status, Status::Pass);
    backend.ring_as_bar = true;
    let result = case.run(&mut backend);
    assert_eq!(result.status, Status::Fail);
    assert_eq!(result.observation[0]["actual"], false);
    assert_eq!(result.observation[0]["expected"], true);
}

#[test]
fn receipt_rejects_unknown_missing_duplicate_and_wrong_proof() {
    let identity = Identity {
        poodle_commit: "a".repeat(40),
        backend_commit: "b".repeat(40),
        lock_sha256: "c".repeat(64),
        platform: "test".into(),
    };
    let mut backend = Fake::default();
    let mut results = cases()
        .map(|case| poodle_conformance::ResultEntry {
            id: case.id,
            proof: case.proof,
            status: Status::NotRun,
            observation: json!("synthetic"),
        })
        .collect::<Vec<_>>();
    let activate = cases()
        .find(|case| case.id == "dispatch.activate")
        .expect("dispatch fixture");
    let dispatch = activate.run(&mut backend);
    assert_eq!(dispatch.status, Status::NotRun);
    assert!(activate.recorded.lock().expect("event recorder").is_empty());
    *results.last_mut().expect("dispatch result") = dispatch;
    let receipt = Receipt::new(identity, results).expect("canonical IDs");
    let json = receipt.to_json().expect("encode receipt");
    assert_eq!(
        Receipt::from_json(json.as_bytes()).expect("validate receipt"),
        receipt
    );

    let baseline: Value = serde_json::from_str(&json).expect("receipt JSON");
    for (change, reason) in [
        ("unknown", "unknown case"),
        ("missing", "missing case"),
        ("duplicate", "duplicate case"),
        ("proof", "wrong proof level"),
    ] {
        let mut altered = baseline.clone();
        let rows = altered["results"].as_array_mut().expect("receipt rows");
        match change {
            "unknown" => rows[0]["id"] = json!("unknown"),
            "missing" => {
                rows.pop();
            }
            "duplicate" => rows.push(rows[0].clone()),
            "proof" => rows[0]["proof"] = json!("dispatch"),
            _ => unreachable!("test case"),
        }
        let bytes = serde_json::to_vec(&altered).expect("serialize altered receipt");
        let error = Receipt::from_json(&bytes).expect_err("invalid case manifest accepted");
        assert!(error.to_string().contains(reason), "{error}");
    }
}

#[test]
fn digest_preserves_sha256_hex_encoding() {
    assert_eq!(
        poodle_conformance::digest(b"abc"),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
}
