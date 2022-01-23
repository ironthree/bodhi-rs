use crate::data::*;

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
    use ComposeState::*;

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
        assert_eq!(string.parse::<ComposeState>().unwrap().to_string(), string);
    }

    for value in values {
        assert_eq!(value.to_string().parse::<ComposeState>().unwrap(), value);
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
