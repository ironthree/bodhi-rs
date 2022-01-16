use crate::data::*;
use std::convert::TryFrom;

#[test]
fn idem_compose_request() {
    use ComposeRequest::*;

    let strings = vec!["stable", "testing"];

    let values = vec![Stable, Testing];

    for string in strings {
        assert_eq!(string.parse::<ComposeRequest>().unwrap().to_string(), string);
    }

    for value in values {
        assert_eq!(value.to_string().parse::<ComposeRequest>().unwrap(), value);
    }
}

#[test]
fn idem_compose_status() {
    use ComposeStatus::*;

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

    let values = vec![
        Cleaning,
        Failed,
        Initializing,
        Notifying,
        Pending,
        Punging,
        Requested,
        SigningRepo,
        Success,
        SyncingRepo,
        UpdateInfo,
    ];

    for string in strings {
        assert_eq!(string.parse::<ComposeStatus>().unwrap().to_string(), string);
    }

    for value in values {
        assert_eq!(value.to_string().parse::<ComposeStatus>().unwrap(), value);
    }
}

#[test]
fn idem_content_type() {
    use ContentType::*;

    let strings = vec!["container", "flatpak", "module", "rpm"];

    let values = vec![Container, Flatpak, Module, RPM];

    for string in strings {
        assert_eq!(string.parse::<ContentType>().unwrap().to_string(), string);
    }

    for value in values {
        assert_eq!(value.to_string().parse::<ContentType>().unwrap(), value);
    }
}

/*
#[test]
fn idem_fedora_release() {
    use FedoraRelease::*;

    let strings = vec![
        "F36", "F36C", "F36F", "F36M", "F35", "F35C", "F35F", "F35M", "F34", "F34C", "F34F", "F34M", "F33", "F33C",
        "F33F", "F33M", "F32", "F32C", "F32F", "F32M", "F31", "F31C", "F31F", "F31M", "F30", "F30C", "F30F", "F30M",
        "F29", "F29C", "F29F", "F29M", "F28", "F28C", "F28M", "F27", "F27M", "F26", "F25", "F24", "F23", "F22", "F21",
        "EPEL-9", "EPEL-9N", "EPEL-8", "EPEL-8M", "EPEL-8N", "EPEL-7", "EL-6", "EL-5", "ELN",
    ];

    let values = vec![
        F36, F36C, F36F, F36M, F35, F35C, F35F, F35M, F34, F34C, F34F, F34M, F33, F33C, F33F, F33M, F32, F32C, F32F,
        F32M, F31, F31C, F31F, F31M, F30, F30C, F30F, F30M, F29, F29C, F29F, F29M, F28, F28C, F28M, F27, F27M, F26,
        F25, F24, F23, F22, F21, EPEL9, EPEL9N, EPEL8, EPEL8M, EPEL8N, EPEL7, EL6, EL5, ELN,
    ];

    assert_eq!(strings.len(), values.len());

    for (string, value) in strings.iter().zip(values.iter()) {
        assert_eq!(&string.parse::<FedoraRelease>().unwrap(), value);
        assert_eq!(string, &value.to_string());
    }

    for string in strings {
        assert_eq!(string.parse::<FedoraRelease>().unwrap().to_string(), string);
    }

    for value in values {
        assert_eq!(value.to_string().parse::<FedoraRelease>().unwrap(), value);
    }
}
*/

#[test]
fn idem_karma() {
    use Karma::*;

    let strings = vec!["+1", "±0", "-1"];

    let values = vec![Positive, Neutral, Negative];

    for string in strings {
        assert_eq!(string.parse::<Karma>().unwrap().to_string(), string);
    }

    for value in values {
        assert_eq!(value.to_string().parse::<Karma>().unwrap(), value);
    }
}

#[test]
fn idem_package_manager() {
    use PackageManager::*;

    let strings = vec!["dnf", "yum"];

    let values = vec![DNF, YUM];

    for string in strings {
        assert_eq!(string.parse::<PackageManager>().unwrap().to_string(), string);
    }

    for value in values {
        assert_eq!(value.to_string().parse::<PackageManager>().unwrap(), value);
    }
}

#[test]
fn idem_release_state() {
    use ReleaseState::*;

    let strings = vec!["archived", "current", "disabled", "frozen", "pending"];

    let values = vec![Archived, Current, Disabled, Frozen, Pending];

    for string in strings {
        assert_eq!(string.parse::<ReleaseState>().unwrap().to_string(), string);
    }

    for value in values {
        assert_eq!(value.to_string().parse::<ReleaseState>().unwrap(), value);
    }
}

#[test]
fn idem_test_gating_status() {
    use TestGatingStatus::*;

    let strings = vec![
        "failed",
        "greenwave_failed",
        "ignored",
        "passed",
        "queued",
        "running",
        "waiting",
    ];

    let values = vec![Failed, GreenwaveFailed, Ignored, Passed, Queued, Running, Waiting];

    for string in strings {
        assert_eq!(string.parse::<TestGatingStatus>().unwrap().to_string(), string);
    }

    for value in values {
        assert_eq!(value.to_string().parse::<TestGatingStatus>().unwrap(), value);
    }
}

#[test]
fn idem_update_request() {
    use UpdateRequest::*;

    let strings = vec!["obsolete", "revoke", "stable", "testing", "unpush"];

    let values = vec![Obsolete, Revoke, Stable, Testing, Unpush];

    for string in strings {
        assert_eq!(string.parse::<UpdateRequest>().unwrap().to_string(), string);
    }

    for value in values {
        assert_eq!(value.to_string().parse::<UpdateRequest>().unwrap(), value);
    }
}

#[test]
fn idem_update_severity() {
    use UpdateSeverity::*;

    let strings = vec!["high", "low", "medium", "unspecified", "urgent"];

    let values = vec![High, Low, Medium, Unspecified, Urgent];

    for string in strings {
        assert_eq!(string.parse::<UpdateSeverity>().unwrap().to_string(), string);
    }

    for value in values {
        assert_eq!(value.to_string().parse::<UpdateSeverity>().unwrap(), value);
    }
}

#[test]
fn idem_update_status() {
    use UpdateStatus::*;

    let strings = vec![
        "obsolete",
        "pending",
        "side_tag_active",
        "side_tag_expired",
        "stable",
        "testing",
        "unpushed",
    ];

    let values = vec![
        Obsolete,
        Pending,
        SideTagActive,
        SideTagExpired,
        Stable,
        Testing,
        Unpushed,
    ];

    for string in strings {
        assert_eq!(string.parse::<UpdateStatus>().unwrap().to_string(), string);
    }

    for value in values {
        assert_eq!(value.to_string().parse::<UpdateStatus>().unwrap(), value);
    }
}

#[test]
fn idem_update_suggestion() {
    use UpdateSuggestion::*;

    let strings = vec!["logout", "reboot", "unspecified"];

    let values = vec![Logout, Reboot, Unspecified];

    for string in strings {
        assert_eq!(string.parse::<UpdateSuggestion>().unwrap().to_string(), string);
    }

    for value in values {
        assert_eq!(value.to_string().parse::<UpdateSuggestion>().unwrap(), value);
    }
}

#[test]
fn idem_update_type() {
    use UpdateType::*;

    let strings = vec!["bugfix", "enhancement", "newpackage", "security", "unspecified"];

    let values = vec![BugFix, Enhancement, NewPackage, Security, Unspecified];

    for string in strings {
        assert_eq!(string.parse::<UpdateType>().unwrap().to_string(), string);
    }

    for value in values {
        assert_eq!(value.to_string().parse::<UpdateType>().unwrap(), value);
    }
}
