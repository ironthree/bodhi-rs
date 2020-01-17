use crate::data::*;
use std::convert::TryFrom;

#[test]
fn release_try_from_ok() {
    assert_eq!(FedoraRelease::try_from("F31").unwrap(), FedoraRelease::F31);
}

#[test]
fn release_format() {
    assert_eq!(FedoraRelease::F31.to_string().as_str(), "F31");
}

#[test]
#[should_panic]
fn release_try_from_err() {
    FedoraRelease::try_from("X12").unwrap();
}

#[test]
fn idem_compose_request() {
    let strings = vec!["stable", "testing"];

    for string in strings {
        assert_eq!(string.parse::<ComposeRequest>().unwrap().to_string(), string);
    }
}

#[test]
fn idem_compose_status() {
    let strings = vec![
        "cleaning",
        "failed",
        "initializing",
        "notifying",
        "pending",
        "punging",
        "requested",
        "signing_repo",
        "success",
        "syncing_repo",
        "updateinfo",
    ];

    for string in strings {
        assert_eq!(string.parse::<ComposeStatus>().unwrap().to_string(), string);
    }
}

#[test]
fn idem_content_type() {
    let strings = vec!["container", "flatpak", "module", "rpm"];

    for string in strings {
        assert_eq!(string.parse::<ContentType>().unwrap().to_string(), string);
    }
}

#[test]
fn idem_fedora_release() {
    let strings = vec![
        "F32", "F32C", "F31", "F31C", "F31F", "F31M", "F30", "F30C", "F30F", "F30M", "F29", "F29C", "F29F", "F29M",
        "F28", "F28C", "F28M", "F27", "F27M", "F26", "F25", "F24", "F23", "F22", "F21", "EPEL-8", "EPEL-8M", "EPEL-7",
        "EL-6", "EL-5",
    ];

    for string in strings {
        assert_eq!(string.parse::<FedoraRelease>().unwrap().to_string(), string);
    }
}

#[test]
fn idem_karma() {
    let strings = vec!["+1", "±0", "-1"];

    for string in strings {
        assert_eq!(string.parse::<Karma>().unwrap().to_string(), string);
    }
}

#[test]
fn idem_package_manager() {
    let strings = vec!["dnf", "yum"];

    for string in strings {
        assert_eq!(string.parse::<PackageManager>().unwrap().to_string(), string);
    }
}

#[test]
fn idem_release_state() {
    let strings = vec!["archived", "current", "disabled", "frozen", "pending"];

    for string in strings {
        assert_eq!(string.parse::<ReleaseState>().unwrap().to_string(), string);
    }
}

#[test]
fn idem_test_gating_status() {
    let strings = vec![
        "failed",
        "greenwave_failed",
        "ignored",
        "passed",
        "queued",
        "running",
        "waiting",
    ];

    for string in strings {
        assert_eq!(string.parse::<TestGatingStatus>().unwrap().to_string(), string);
    }
}

#[test]
fn idem_update_request() {
    let strings = vec!["obsolete", "revoke", "stable", "testing", "unpush"];

    for string in strings {
        assert_eq!(string.parse::<UpdateRequest>().unwrap().to_string(), string);
    }
}

#[test]
fn idem_update_severity() {
    let strings = vec!["high", "low", "medium", "unspecified", "urgent"];

    for string in strings {
        assert_eq!(string.parse::<UpdateSeverity>().unwrap().to_string(), string);
    }
}

#[test]
fn idem_update_status() {
    let strings = vec![
        "obsolete",
        "pending",
        "side_tag_active",
        "side_tag_expired",
        "stable",
        "testing",
        "unpushed",
    ];

    for string in strings {
        assert_eq!(string.parse::<UpdateStatus>().unwrap().to_string(), string);
    }
}

#[test]
fn idem_update_suggestion() {
    let strings = vec!["logout", "reboot", "unspecified"];

    for string in strings {
        assert_eq!(string.parse::<UpdateSuggestion>().unwrap().to_string(), string);
    }
}

#[test]
fn idem_update_type() {
    let strings = vec!["bugfix", "enhancement", "newpackage", "security", "unspecified"];

    for string in strings {
        assert_eq!(string.parse::<UpdateType>().unwrap().to_string(), string);
    }
}
