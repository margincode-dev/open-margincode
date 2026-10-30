//! Applied migrations must never change: sqlx stores each one's checksum, and a daemon whose
//! migration file differs from the applied one refuses to start ("migration N was previously
//! applied but has been modified"). This happened once, when a project-wide rename touched a
//! comment in 0002. A new migration gets a new file and a new line here; an old one is never
//! edited, not even a comment.

use sha2::{Digest, Sha256};

const FROZEN: &[(&str, &str)] = &[
    (
        "0001_init.sql",
        "ef796e90014d2bf5e53013b5a9901966b6242e33f2ea8aa1f28c30b9fd4f15a6",
    ),
    (
        "0002_index.sql",
        "92a4d06a91f2f5727124116deb53f0fdb3e1be69a531f86354010651a617a297",
    ),
    (
        "0003_semantic.sql",
        "12861b75a06fa6e773704ab8e5ef80ad67ed59ab3e82105a750e7a2ff0e37d21",
    ),
    (
        "0004_receipts.sql",
        "e51f4b540aa1b953023692a049fda3ae9a764d2b944cb5018f1271cc172061d4",
    ),
    (
        "0005_freshness.sql",
        "1cdc14d6ee358684e9a1bda3b5fc920104af6d23056947c9de641be10362c8cd",
    ),
    (
        "0006_budget.sql",
        "770a0a0a8bc2bd317f738aaf05670b8d3654611f57805f4592e44cc00407b104",
    ),
    (
        "0007_expand.sql",
        "c045b53f829c71f6fe5a323262def293784dbd970dede98c381ad564bed0d2ad",
    ),
    (
        "0008_flow.sql",
        "0a0052066db35d59d98949af111fd713fb7eb8397999144da757d5378dad3d6d",
    ),
    (
        "0009_index_scopes.sql",
        "21e7514176de8773b08b36c752fe7bec0a3e98a0c28e26ca6d0ef0878d15293c",
    ),
    (
        "0010_claims.sql",
        "d7e413712145af625af15a9a70e7fa7962f1d867f1ea6daabdb3dd96e550008c",
    ),
    (
        "0011_backup.sql",
        "79f7ed255ccd74f6eeb50a3b53b7377dc9d1d326f4a4fea37157f6e27d5a28e0",
    ),
    (
        "0012_prune.sql",
        "2aa96e308cb448714689f0150bf9e2f4019d533740a06bec3e70077b8160c67a",
    ),
    (
        "0013_pack_mode.sql",
        "bad9d6dbbd0700cb895c1ecced2b05ec1d63d1846c9eb2759ae1baad4c1f1f75",
    ),
    (
        "0014_events.sql",
        "6025ba9e81cccf2c92df051eacd7e8925f7b21cf159f61e6e211beb1d75ded8e",
    ),
    (
        "0015_carried_approvals.sql",
        "43b110398790701fa4b1bb36ac366dce92347f628b1691d1c547bf4fd7bcd078",
    ),
    (
        "0016_convention_checks.sql",
        "0e620b9d3ec05a672de632d7e14d0e2bdccb6982f3b640db6d68b12e6b562de5",
    ),
    (
        "0017_checks.sql",
        "a35282593f0ca2598fbf2f72a118f8b2e5bfa22d29f1f0c9ad0867f5af0ebf92",
    ),
    (
        "0018_derived.sql",
        "899b2c685df6866c49cecd3a2e34d4ddca8f456610d247e091f918754a5e243b",
    ),
    (
        "0019_code_fts.sql",
        "edaca9127b1da383e607f22ce67b3bb132d3ef999f226e715f99407ee29ce64f",
    ),
    (
        "0020_constants.sql",
        "eb90ab46cf77ea193401f0ee43fe69c1e18ebe08c962f8ea63f59c73c077b80f",
    ),
    (
        "0021_overlay.sql",
        "891098c74d7f8fc6871a1b3346f112353a01ef444f565e15ff53c1b9931271a6",
    ),
];

#[test]
fn applied_migrations_are_byte_for_byte_unchanged() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("migrations");
    let mut files: Vec<String> = std::fs::read_dir(&dir)
        .unwrap()
        .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
        .filter(|n| n.ends_with(".sql"))
        .collect();
    files.sort();
    for (name, want) in FROZEN {
        let bytes = std::fs::read(dir.join(name))
            .unwrap_or_else(|e| panic!("{name}: {e} (a migration was deleted or renamed)"));
        let got = format!("{:x}", Sha256::digest(&bytes));
        assert_eq!(
            &got, want,
            "{name} changed. Applied migrations are immutable: revert it and put the change in a new migration"
        );
    }
    for f in &files {
        assert!(
            FROZEN.iter().any(|(n, _)| n == f),
            "{f} is not in FROZEN: add (\"{f}\", \"<sha256>\") once the migration is final"
        );
    }
}
