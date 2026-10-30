use scholion_core::{AgentFamily, Tier};
use scholion_pack::{Allocation, Item, Section, allocate, allocate_with_shares, estimate_tokens};

const FAM: AgentFamily = AgentFamily::Other;

fn text(words: usize) -> String {
    "alpha ".repeat(words)
}

fn item(id: &str, section: Section, score: f32, h: usize, s: usize, f: usize) -> Item {
    Item {
        id: id.into(),
        section,
        score,
        tiers: [text(h), text(s), text(f)],
        reason: "test",
        source: None,
        required: false,
        group: None,
        order: None,
    }
}

fn tok(t: &str) -> u32 {
    estimate_tokens(t, FAM)
}

fn used_by(a: &Allocation, section: Section) -> u32 {
    a.chosen
        .iter()
        .filter(|(i, _)| i.section == section)
        .map(|(i, t)| tok(i.text(*t)))
        .sum()
}

/// What `used` must equal: chosen tier texts plus the listed omitted lines.
fn recomputed_used(a: &Allocation) -> u32 {
    a.chosen.iter().map(|(i, t)| tok(i.text(*t))).sum::<u32>()
        + a.omitted.iter().map(|l| tok(l)).sum::<u32>()
}

#[test]
fn estimate_tokens_counts_and_grows_with_text() {
    assert_eq!(estimate_tokens("", FAM), 0);
    assert_eq!(estimate_tokens("hello world", FAM), 2);
    assert!(tok(&text(50)) > tok(&text(5)));
}

#[test]
fn shares_split_ten_five_ten_twenty_fortyfive_ten() {
    let mut items = vec![];
    for (section, name) in [
        (Section::Task, "t"),
        (Section::Conflicts, "x"),
        (Section::Map, "m"),
        (Section::History, "h"),
        (Section::Code, "c"),
        (Section::ConventionsLive, "v"),
    ] {
        // the conflicts share (5%) holds the handles of fewer items
        let n = if section == Section::Conflicts {
            60
        } else {
            150
        };
        for i in 0..n {
            items.push(item(
                &format!("{name}-{i}"),
                section,
                1.0 - i as f32 / 1000.0,
                3,
                12,
                40,
            ));
        }
    }
    let a = allocate(items, 10_000, FAM); // effective budget 9_000
    assert!(a.omitted.is_empty());
    for (section, share) in [
        (Section::Task, 900),
        (Section::Conflicts, 450),
        (Section::Map, 900),
        (Section::History, 1800),
        (Section::Code, 4050),
        (Section::ConventionsLive, 900),
    ] {
        let used = used_by(&a, section) as i64;
        assert!(
            (used - share).abs() <= 45,
            "{section:?}: used {used}, share {share}"
        );
    }
    assert!(a.used <= 9_000);
}

#[test]
fn unused_share_spills_to_next_section() {
    let mut items = vec![item("task-only", Section::Task, 1.0, 3, 12, 40)];
    for i in 0..150 {
        items.push(item(
            &format!("m-{i}"),
            Section::Map,
            1.0 - i as f32 / 1000.0,
            3,
            12,
            40,
        ));
    }
    // only the shares up to the map, so the last-pass refill (tested below) adds nothing
    let shares = [("task", 10), ("conflicts", 5), ("map", 10)].map(|(k, p)| (k.to_owned(), p));
    let a = allocate_with_shares(items, 10_000, FAM, &shares);
    let map_used = used_by(&a, Section::Map);
    assert!(
        map_used > 1950 && map_used <= 2250,
        "map used {map_used}: own 900 + spill from the task and (empty) conflicts sections"
    );
}

#[test]
fn higher_score_gets_richer_tier_first() {
    // task section budget = 10% of effective(500) = 45; three handles cost 6, so 39 remain to upgrade with
    let items = vec![
        item("low", Section::Task, 0.1, 2, 10, 30),
        item("high", Section::Task, 0.9, 2, 10, 30),
        item("mid", Section::Task, 0.5, 2, 10, 30),
    ];
    // the task share alone: the other sections' budget would reach these items in the last pass
    let a = allocate_with_shares(items, 500, FAM, &[("task".to_owned(), 10)]);
    let tier_of = |id: &str| a.chosen.iter().find(|(i, _)| i.id == id).unwrap().1;
    assert_eq!(tier_of("high"), Tier::Full);
    assert_eq!(tier_of("mid"), Tier::Summary);
    assert_eq!(tier_of("low"), Tier::Handle);
    assert!(a.omitted.is_empty());
}

#[test]
fn tiny_budget_returns_handles_only_and_lists_omitted() {
    let mut items = vec![];
    for (section, name) in [
        (Section::Task, "t"),
        (Section::Map, "m"),
        (Section::History, "h"),
        (Section::Code, "c"),
        (Section::ConventionsLive, "v"),
    ] {
        for i in 0..8 {
            items.push(item(
                &format!("{name}-{i}"),
                section,
                1.0 - i as f32 / 100.0,
                6,
                20,
                60,
            ));
        }
    }
    let a = allocate(items, 150, FAM);
    assert!(a.chosen.iter().all(|(_, t)| *t == Tier::Handle));
    assert!(!a.omitted.is_empty());
    assert_eq!(a.chosen.len() + a.omitted_total, 40);
    assert!(a.used <= 150, "used {}", a.used);
    assert_eq!(a.used, recomputed_used(&a));
}

#[test]
fn omitted_list_is_capped_at_twenty_lines() {
    let items: Vec<Item> = (0..100)
        .map(|i| {
            item(
                &format!("c-{i}"),
                Section::Code,
                1.0 - i as f32 / 1000.0,
                6,
                20,
                60,
            )
        })
        .collect();
    let a = allocate(items, 300, FAM);
    assert!(a.omitted.len() <= 20);
    assert!(a.omitted_total > a.omitted.len());
    assert!(a.used <= 300);
}

#[test]
fn empty_items_ok() {
    let a = allocate(vec![], 1000, FAM);
    assert!(a.chosen.is_empty() && a.omitted.is_empty());
    assert_eq!((a.used, a.omitted_total), (0, 0));
    assert_eq!(
        allocate(vec![item("x", Section::Task, 1.0, 1, 2, 3)], 0, FAM).used,
        0
    );
}

#[test]
fn result_does_not_depend_on_input_order() {
    let make = || -> Vec<Item> {
        (0..30)
            .map(|i| {
                item(
                    &format!("c-{i}"),
                    Section::Code,
                    (i % 5) as f32 / 10.0,
                    4,
                    16,
                    50,
                )
            })
            .collect()
    };
    let mut reversed = make();
    reversed.reverse();
    let summary = |a: Allocation| {
        a.chosen
            .into_iter()
            .map(|(i, t)| (i.id, t))
            .collect::<Vec<_>>()
    };
    assert_eq!(
        summary(allocate(make(), 600, FAM)),
        summary(allocate(reversed, 600, FAM))
    );
}

#[test]
fn never_exceeds_budget_for_random_inputs() {
    let mut seed: u64 = 0x9E3779B97F4A7C15;
    let mut next = move |m: u64| {
        seed = seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (seed >> 33) % m
    };
    let sections = [
        Section::Task,
        Section::Map,
        Section::History,
        Section::Code,
        Section::ConventionsLive,
    ];
    for round in 0..300 {
        let n = next(60) as usize;
        let items: Vec<Item> = (0..n)
            .map(|i| {
                let h = 1 + next(8) as usize;
                item(
                    &format!("i{round}-{i}"),
                    sections[next(5) as usize],
                    next(1000) as f32 / 1000.0,
                    h,
                    h + next(30) as usize,
                    h + next(200) as usize,
                )
            })
            .collect();
        let budget = (next(8000) + (round % 3) as u64) as u32;
        let a = allocate(items, budget, FAM);
        assert!(
            a.used <= budget,
            "round {round}: used {} > budget {budget}",
            a.used
        );
        assert_eq!(a.used, recomputed_used(&a), "round {round}");
    }
}

fn required(mut i: Item) -> Item {
    i.required = true;
    i
}

#[test]
fn required_item_survives_a_tiny_budget() {
    let mut items: Vec<Item> = (0..30)
        .map(|n| item(&format!("sym:{n}"), Section::Code, 5.0, 20, 80, 300))
        .collect();
    items.push(required(item(
        "conv:1",
        Section::ConventionsLive,
        0.0,
        15,
        40,
        60,
    )));
    let a = allocate(items, 150, FAM);
    assert!(a.used <= 150, "{}", a.used);
    assert!(
        a.chosen.iter().any(|(i, _)| i.id == "conv:1"),
        "required item dropped: {:?}",
        a.chosen.iter().map(|(i, _)| &i.id).collect::<Vec<_>>()
    );
}

#[test]
fn required_items_never_exceed_budget() {
    let items: Vec<Item> = (0..40)
        .map(|n| {
            required(item(
                &format!("conv:{n}"),
                Section::ConventionsLive,
                n as f32,
                30,
                60,
                90,
            ))
        })
        .collect();
    let a = allocate(items, 200, FAM);
    assert!(a.used <= 200, "{}", a.used);
    assert!(a.omitted_total > 0, "not all 40 handles fit in 200 tokens");
    // the best-scored required items are the ones kept
    assert!(a.chosen.iter().any(|(i, _)| i.id == "conv:39"));
}

#[test]
fn required_item_can_still_upgrade_to_full_when_room() {
    let a = allocate(
        vec![required(item(
            "conv:1",
            Section::ConventionsLive,
            0.0,
            10,
            20,
            30,
        ))],
        4000,
        FAM,
    );
    assert_eq!(a.chosen[0].1, scholion_core::Tier::Full);
}

fn grouped(id: &str, group: &str, score: f32, words: usize) -> Item {
    let mut i = item(id, Section::Code, score, 3, words, words * 2);
    i.group = Some(group.to_owned());
    i
}

#[test]
fn custom_shares_respect_budget_and_spill_over() {
    let shares = [("empty".to_owned(), 50u8), ("busy".to_owned(), 50u8)];
    let mut items: Vec<Item> = (0..6)
        .map(|n| grouped(&format!("b:{n}"), "busy", 6.0 - n as f32, 80))
        .collect();
    items.push(grouped("x:1", "unlisted", 9.0, 5));
    let budget = 1000;
    let a = allocate_with_shares(items, budget, FAM, &shares);
    assert!(a.used <= budget, "used {} > {budget}", a.used);
    assert_eq!(a.used, recomputed_used(&a));
    let busy: u32 = a
        .chosen
        .iter()
        .filter(|(i, _)| i.group.as_deref() == Some("busy"))
        .map(|(i, t)| tok(i.text(*t)))
        .sum();
    // "empty" used nothing, so its half flowed on to "busy"
    assert!(busy > budget / 2, "busy got only {busy} of {budget}");
    assert!(
        a.chosen.iter().all(|(i, _)| i.id != "x:1"),
        "an item of a group without a share is never chosen"
    );
    assert_eq!(a.omitted_total, 1, "{:?}", a.omitted);
    // chosen items come grouped in share order, best score first
    let ids: Vec<&str> = a.chosen.iter().map(|(i, _)| i.id.as_str()).collect();
    assert_eq!(ids, ["b:0", "b:1", "b:2", "b:3", "b:4", "b:5"]);
}

#[test]
fn budget_left_after_the_last_section_upgrades_earlier_handles() {
    // Code holds every item; the conventions/live share after it is empty. What it leaves must
    // not be wasted while code items are still handles.
    let items: Vec<Item> = (0..40)
        .map(|i| {
            item(
                &format!("c-{i:02}"),
                Section::Code,
                1.0 - i as f32 / 100.0,
                2,
                6,
                60,
            )
        })
        .collect();
    let a = allocate(items, 1000, FAM);
    let left = 900 - a.used;
    for (i, t) in &a.chosen {
        if *t == Tier::Handle {
            let up = tok(i.text(Tier::Summary)) - tok(i.text(Tier::Handle));
            assert!(left < up, "{} is a handle with {left} tokens unused", i.id);
        }
    }
    assert_eq!(a.used, recomputed_used(&a));
}
