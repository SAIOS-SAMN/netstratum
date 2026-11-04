//! Mycelium's client conformance read, in process (`check_codecs`), for the
//! KERNEL profile at the pinned tag: every act this uplink speaks
//! ([`ns_hypha::SPOKEN`]) through both ends of its codec, and every MUST act
//! of the profile spoken.
use ns_hypha::mycelium_sdk::conformance::{check_codecs, Profile};

#[test]
fn the_uplink_conforms_to_the_kernel_profile_in_process() {
    let report = check_codecs(&[Profile::Kernel], |a| ns_hypha::SPOKEN.contains(&a.name));
    let failures: Vec<String> = report
        .failures()
        .map(|(act, why)| format!("{act}: {why}"))
        .collect();
    assert!(report.conforms(), "failing rows:\n{}", failures.join("\n"));
}

#[test]
fn every_act_spoken_is_named_by_the_statement() {
    let named: Vec<&str> = ns_hypha::mycelium_sdk::conformance::STATEMENT
        .union(&[Profile::Kernel])
        .iter()
        .map(|a| a.name)
        .collect();
    for act in ns_hypha::SPOKEN {
        assert!(named.contains(act), "{act} is no act of the kernel profile");
    }
}
