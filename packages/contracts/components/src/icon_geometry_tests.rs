#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    const VECTOR_SOURCE: &str = include_str!("../../../core/src/icons/geometry-vectors.json");

    #[derive(Deserialize)]
    struct VectorDocument {
        #[serde(rename = "schemaVersion")]
        schema_version: u32,
        vectors: Vec<GeometryVector>,
    }

    #[derive(Deserialize)]
    struct GeometryVector {
        id: String,
        left: VectorInput,
        right: Option<VectorInput>,
        expect: VectorExpectation,
    }

    #[derive(Deserialize)]
    struct VectorInput {
        #[serde(rename = "viewBox")]
        view_box: [f64; 4],
        fill: String,
        stroke: String,
        #[serde(rename = "strokeWidth")]
        stroke_width: f64,
        #[serde(rename = "strokeLinecap")]
        stroke_linecap: String,
        #[serde(rename = "strokeLinejoin")]
        stroke_linejoin: String,
        nodes: Vec<(String, BTreeMap<String, String>)>,
    }

    #[derive(Deserialize)]
    struct VectorExpectation {
        left: GeometryExpectation,
        right: Option<GeometryExpectation>,
        pair: Option<PairExpectation>,
    }

    #[derive(Deserialize)]
    struct GeometryExpectation {
        status: String,
        code: Option<String>,
        #[serde(rename = "contourCount")]
        contour_count: Option<usize>,
        closed: Option<Vec<bool>>,
        #[serde(rename = "segmentCounts")]
        segment_counts: Option<Vec<usize>>,
        #[serde(rename = "canonicalPoints")]
        canonical_points: Option<Vec<Vec<[i64; 2]>>>,
        #[serde(rename = "wireDigest")]
        wire_digest: Option<String>,
    }

    #[derive(Deserialize)]
    struct ExpectedMapping {
        #[serde(rename = "leftIndex")]
        left_index: usize,
        #[serde(rename = "rightIndex")]
        right_index: usize,
        reversed: bool,
        offset: usize,
        #[serde(rename = "costMicros")]
        cost_micros: i64,
    }

    #[derive(Deserialize)]
    struct PairOracle {
        #[serde(rename = "leftDigest")]
        left_digest: String,
        #[serde(rename = "rightDigest")]
        right_digest: String,
        #[serde(rename = "pairDigest")]
        pair_digest: String,
        mappings: Vec<ExpectedMapping>,
        #[serde(rename = "costMicros")]
        cost_micros: i64,
    }

    #[derive(Deserialize)]
    struct PairExpectation {
        status: String,
        code: Option<String>,
        reversed: Option<Vec<bool>>,
        offsets: Option<Vec<usize>>,
        oracle: Option<PairOracle>,
    }

    fn input(value: VectorInput) -> IconGeometryInput {
        IconGeometryInput {
            view_box: value.view_box,
            fill: value.fill,
            stroke: value.stroke,
            stroke_width: value.stroke_width,
            stroke_linecap: value.stroke_linecap,
            stroke_linejoin: value.stroke_linejoin,
            nodes: value.nodes,
        }
    }

    fn points(geometry: &NormalizedIconGeometry) -> Vec<Vec<[i64; 2]>> {
        geometry
            .canonical
            .contours
            .iter()
            .map(|contour| {
                canonical_points(contour)
                    .expect("test geometry has canonical segments")
                    .into_iter()
                    .map(|point| [point.x, point.y])
                    .collect()
            })
            .collect()
    }

    fn assert_geometry(
        result: Result<NormalizedIconGeometry, GeometryError>,
        expected: &GeometryExpectation,
    ) {
        if expected.status == "rejected" {
            let error = result.expect_err("vector should reject");
            assert_eq!(error.code.as_str(), expected.code.as_deref().unwrap());
            return;
        }
        let geometry = result.expect("vector should normalize");
        assert_eq!(geometry.topology.closed, expected.closed.clone().unwrap());
        assert_eq!(
            geometry.topology.segment_counts,
            expected.segment_counts.clone().unwrap()
        );
        assert_eq!(
            geometry.canonical.contours.len(),
            expected.contour_count.unwrap()
        );
        if let Some(expected_points) = &expected.canonical_points {
            assert_eq!(&points(&geometry), expected_points);
        }
        if let Some(expected_digest) = &expected.wire_digest {
            assert_eq!(geometry_wire_digest(&geometry), *expected_digest);
        }
        assert!(geometry
            .sampled
            .contours
            .iter()
            .all(|contour| contour.points.len() == SAMPLE_COUNT));
    }

    #[test]
    fn shared_vectors_cover_both_normalization_and_pair_planning() {
        let document: VectorDocument = serde_json::from_str(VECTOR_SOURCE).expect("valid vectors");
        assert_eq!(document.schema_version, 1);
        for vector in document.vectors {
            let left = normalize_icon_geometry(&input(vector.left));
            assert_geometry(left.clone(), &vector.expect.left);
            let Some(right_input) = vector.right else {
                continue;
            };
            let right = normalize_icon_geometry(&input(right_input));
            assert_geometry(right.clone(), vector.expect.right.as_ref().unwrap());
            let pair = vector.expect.pair.as_ref().unwrap();
            if pair.status == "rejected" {
                let left = left.expect("left endpoint should normalize");
                let right = right.expect("right endpoint should normalize");
                let error = plan_icon_geometry_pair(&left, &right).expect_err("pair should reject");
                assert_eq!(
                    error.code.as_str(),
                    pair.code.as_deref().unwrap(),
                    "{}",
                    vector.id
                );
                continue;
            }
            let left = left.expect("left endpoint should normalize");
            let right = right.expect("right endpoint should normalize");
            let plan = plan_icon_geometry_pair(&left, &right).expect("pair should plan");
            if let Some(expected_reversed) = &pair.reversed {
                assert_eq!(
                    plan.contour_mappings
                        .iter()
                        .map(|mapping| mapping.reversed)
                        .collect::<Vec<_>>(),
                    *expected_reversed,
                    "{}",
                    vector.id
                );
            }
            if let Some(expected_offsets) = &pair.offsets {
                assert_eq!(
                    plan.contour_mappings
                        .iter()
                        .map(|mapping| mapping.offset)
                        .collect::<Vec<_>>(),
                    *expected_offsets,
                    "{}",
                    vector.id
                );
            }
            for mapping in &plan.contour_mappings {
                assert_eq!(
                    left.sampled.contours[mapping.left_index].closed,
                    right.sampled.contours[mapping.right_index].closed,
                    "{}",
                    vector.id
                );
            }
            let oracle = pair.oracle.as_ref().expect("missing exact pair oracle");
            assert_eq!(
                geometry_wire_digest(&left),
                oracle.left_digest,
                "{}",
                vector.id
            );
            assert_eq!(
                geometry_wire_digest(&right),
                oracle.right_digest,
                "{}",
                vector.id
            );
            assert_eq!(plan.cost_micros, oracle.cost_micros, "{}", vector.id);
            assert_eq!(
                pair_wire_digest(&left, &right, &plan),
                oracle.pair_digest,
                "{}",
                vector.id
            );
            assert_eq!(
                plan.contour_mappings
                    .iter()
                    .map(|mapping| {
                        (
                            mapping.left_index,
                            mapping.right_index,
                            mapping.reversed,
                            mapping.offset,
                            mapping.cost_micros,
                        )
                    })
                    .collect::<Vec<_>>(),
                oracle
                    .mappings
                    .iter()
                    .map(|mapping| {
                        (
                            mapping.left_index,
                            mapping.right_index,
                            mapping.reversed,
                            mapping.offset,
                            mapping.cost_micros,
                        )
                    })
                    .collect::<Vec<_>>(),
                "{}",
                vector.id
            );
            let planned = PlannedIconGeometryPair {
                left: &left,
                right: &right,
                plan: &plan,
            };
            assert_eq!(
                frame_at(&planned, 0.0)
                    .expect("left endpoint")
                    .contours
                    .into_iter()
                    .map(|contour| contour.points)
                    .collect::<Vec<_>>(),
                left.canonical
                    .contours
                    .iter()
                    .map(|contour| canonical_points(contour).unwrap())
                    .collect::<Vec<_>>()
            );
            assert_eq!(
                frame_at(&planned, 1.0)
                    .expect("right endpoint")
                    .contours
                    .into_iter()
                    .map(|contour| contour.points)
                    .collect::<Vec<_>>(),
                right
                    .canonical
                    .contours
                    .iter()
                    .map(|contour| canonical_points(contour).unwrap())
                    .collect::<Vec<_>>()
            );
            let reverse_plan = reverse_pair_plan(&plan);
            let reverse_pair = PlannedIconGeometryPair {
                left: &right,
                right: &left,
                plan: &reverse_plan,
            };
            assert_eq!(
                frame_at(&reverse_pair, 0.0)
                    .expect("reverse left endpoint")
                    .contours
                    .into_iter()
                    .map(|contour| contour.points)
                    .collect::<Vec<_>>(),
                right
                    .canonical
                    .contours
                    .iter()
                    .map(|contour| canonical_points(contour).unwrap())
                    .collect::<Vec<_>>()
            );
            assert_eq!(
                frame_at(&reverse_pair, 1.0)
                    .expect("reverse right endpoint")
                    .contours
                    .into_iter()
                    .map(|contour| contour.points)
                    .collect::<Vec<_>>(),
                left.canonical
                    .contours
                    .iter()
                    .map(|contour| canonical_points(contour).unwrap())
                    .collect::<Vec<_>>()
            );
            for progress in [0.25, 0.5, 0.75] {
                let forward = frame_at(&planned, progress).expect("forward frame");
                let reverse = frame_at(&reverse_pair, 1.0 - progress).expect("reverse frame");
                for mapping in &plan.contour_mappings {
                    let forward_points = &forward.contours[mapping.left_index].points;
                    let reverse_points = &reverse.contours[mapping.right_index].points;
                    for (index, forward_point) in forward_points.iter().enumerate() {
                        let reverse_index = if mapping.reversed {
                            modulo(
                                mapping.offset as isize - index as isize,
                                reverse_points.len(),
                            )
                        } else {
                            modulo(
                                mapping.offset as isize + index as isize,
                                reverse_points.len(),
                            )
                        };
                        assert_eq!(
                            reverse_points[reverse_index], *forward_point,
                            "{}",
                            vector.id
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn generated_registry_has_complete_lineage_and_candidate_gate() {
        assert_eq!(ICON_GEOMETRY_REGISTRY_SCHEMA_VERSION, 1);
        assert_eq!(ICON_GEOMETRY_NORMALIZER_VERSION, "1.0.0");
        assert_eq!(ICON_GEOMETRY_SOURCE_PACKAGE, "lucide-static");
        assert_eq!(ICON_GEOMETRY_SOURCE_VERSION, "1.31.0");
        assert_eq!(ICON_GEOMETRY_NOTICE_ID, "lucide-static-isc-feather-mit");
        assert!(ICON_GEOMETRY_REGISTRY.len() >= 8);
        assert!(ICON_GEOMETRY_REGISTRY
            .iter()
            .any(|pair| pair.status == GeneratedPairStatus::Candidate));
        assert!(ICON_GEOMETRY_REGISTRY
            .iter()
            .any(|pair| pair.status == GeneratedPairStatus::Rejected));
        assert_eq!(
            ICON_GEOMETRY_REGISTRY
                .iter()
                .filter(|pair| pair.status == GeneratedPairStatus::Accepted)
                .count(),
            0
        );
        assert_eq!(
            ICON_GEOMETRY_REGISTRY
                .iter()
                .filter(|pair| pair.status == GeneratedPairStatus::Candidate)
                .count(),
            6
        );
        assert_eq!(
            ICON_GEOMETRY_REGISTRY
                .iter()
                .filter(|pair| pair.status == GeneratedPairStatus::Rejected)
                .count(),
            6
        );
        let candidate = ICON_GEOMETRY_REGISTRY
            .iter()
            .find(|pair| pair.id == "circle-to-dot")
            .expect("candidate pair");
        assert_eq!(candidate.id, "circle-to-dot");
        assert!(candidate.geometry_left.is_some());
        assert!(candidate.geometry_right.is_some());
        assert!(candidate.plan.is_some());
        for pair in ICON_GEOMETRY_REGISTRY {
            assert!(!pair.id.is_empty());
            assert!(!pair.source_digest_left.is_empty());
            assert!(!pair.source_digest_right.is_empty());
            assert!(!pair.asset_digest_left.is_empty());
            assert!(!pair.asset_digest_right.is_empty());
            assert!(!pair.quality_notes.is_empty());
            if pair.status != GeneratedPairStatus::Rejected {
                assert!(pair.geometry_left.is_some());
                assert!(pair.geometry_right.is_some());
                assert!(pair.plan.is_some());
            }
            assert!(pair.payload_bytes <= 16 * 1024);
        }
    }

    #[test]
    fn runtime_paints_exact_endpoints_and_rebases_reversals() {
        use poodle_headless::motion_policy::MotionPolicy;

        let mut runtime = create_icon_geometry_runtime(MotionPolicy::Full);
        let intent = GeometryRuntimeIntent {
            owner: String::from("fixture-owner"),
            pair_id: String::from("chevron-left-to-chevron-right"),
            target: GeometryEndpoint::To,
            initial: true,
        };
        let initial = activate_icon_geometry(&mut runtime, intent.clone());
        assert!(!initial.schedule);
        assert!(initial.paint_endpoint);
        assert_eq!(live_geometry_clock_count(&runtime), 0);

        let mut live = intent;
        live.initial = false;
        let forward = activate_icon_geometry(&mut runtime, live.clone());
        assert!(forward.schedule);
        sample_icon_geometry(&mut runtime, &forward.key, 0.4);
        live.target = GeometryEndpoint::From;
        let reverse = activate_icon_geometry(&mut runtime, live);
        assert_eq!(reverse.interruption, poodle_headless::motion_policy::MotionInterruption::Reverse);
        assert!(reverse.schedule);
        assert_eq!(live_geometry_clock_count(&runtime), 1);

        let mut other = GeometryRuntimeIntent {
            owner: String::from("fixture-owner"),
            pair_id: String::from("circle-to-dot"),
            target: GeometryEndpoint::To,
            initial: false,
        };
        let replaced = activate_icon_geometry(&mut runtime, other.clone());
        assert_eq!(
            replaced.interruption,
            poodle_headless::motion_policy::MotionInterruption::Retarget
        );
        assert_eq!(replaced.pair_id, Some("circle-to-dot"));

        other.pair_id = String::from("menu-to-ellipsis");
        let rejected = activate_icon_geometry(&mut runtime, other);
        assert!(!rejected.accepted);
        assert_eq!(live_geometry_clock_count(&runtime), 0);
        assert!(current_icon_geometry_frame(&runtime).is_none());

        let mut frozen = create_icon_geometry_runtime(MotionPolicy::Full);
        let start = activate_icon_geometry(
            &mut frozen,
            GeometryRuntimeIntent {
                owner: String::from("fixture-owner"),
                pair_id: String::from("chevron-left-to-chevron-right"),
                target: GeometryEndpoint::To,
                initial: false,
            },
        );
        sample_icon_geometry(&mut frozen, &start.key, 0.5);
        set_icon_geometry_policy(&mut frozen, MotionPolicy::Frozen);
        assert_eq!(live_geometry_clock_count(&frozen), 0);
        teardown_icon_geometry(&mut frozen, None);
        assert!(current_icon_geometry_frame(&frozen).is_none());
    }

    #[test]
    fn hot_path_samples_reuse_buffer_capacity() {
        use poodle_headless::motion_policy::MotionPolicy;

        let mut runtime = create_icon_geometry_runtime(MotionPolicy::Full);
        let start = activate_icon_geometry(
            &mut runtime,
            GeometryRuntimeIntent {
                owner: String::from("fixture-owner"),
                pair_id: String::from("chevron-left-to-chevron-right"),
                target: GeometryEndpoint::To,
                initial: false,
            },
        );
        sample_icon_geometry(&mut runtime, &start.key, 0.2);
        let first_capacity = runtime.frame.contours[0].points.capacity();
        let first_ptr = runtime.frame.contours[0].points.as_ptr();
        sample_icon_geometry(&mut runtime, &start.key, 0.8);
        assert_eq!(runtime.frame.contours[0].points.capacity(), first_capacity);
        assert_eq!(runtime.frame.contours[0].points.as_ptr(), first_ptr);
    }
}
