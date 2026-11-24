//! Dreams and verdicts with hand-written verdicts and no model.

use crate::common::*;
use margincode_core::{IntentStore, RecordId};

/// A line of `main`'s `app/greeter.py`, long enough to be a quote.
const GREETER_LINE: &str = "return format_name(name)";

async fn post_json(e: &Env, path: &str, body: Value) -> (StatusCode, Value) {
    call(&e.app, "POST", path, Some(body)).await
}

async fn record_on(e: &Env, t: &Value, kind: &str, summary: &str, path: &str) -> Value {
    let (status, body) = post_json(
        e,
        &format!("/v1/tasks/{}/records", id(t)),
        json!({"kind": kind, "summary": summary, "agent": "claude",
               "body": format!("Checked by hand: {summary}"),
               "anchors": [{"path": path}]}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    body
}

fn handle(rec: &Value) -> String {
    format!("rec:{}", &rec["id"].as_str().unwrap().replace('-', "")[..8])
}

/// The later task edits the greeter, so its packs show the earlier task's records.
async fn two_tasks(e: &Env) -> (Value, Value) {
    let earlier = start_task(e, "Earlier work").await;
    let reader = start_task(e, "Louder hello").await;
    edit_greeter(e, &reader);
    let (status, cp) = post_json(
        e,
        &format!("/v1/tasks/{}/checkpoint", id(&reader)),
        json!({}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{cp}");
    (earlier, reader)
}

async fn pack(e: &Env, t: &Value, session: &str) -> String {
    let (status, body) = call(
        &e.app,
        "GET",
        &format!("/v1/tasks/{}/context?budget=3000&session={session}", id(t)),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    body["markdown"].as_str().unwrap().to_owned()
}

async fn start_dream(e: &Env, mode: &str, area: &str) -> Value {
    let (status, body) = post_json(
        e,
        "/v1/repos/demo/dreams",
        json!({"agent": "dreamer", "mode": mode, "area": area}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    body
}

fn dream_id(d: &Value) -> String {
    d["dream"]["id"]
        .as_str()
        .unwrap_or_else(|| panic!("no run: {d}"))
        .to_owned()
}

async fn propose(e: &Env, dream: &Value, verdict: Value) -> Value {
    let (status, body) = post_json(
        e,
        &format!("/v1/dreams/{}/verdicts", dream_id(dream)),
        json!({"verdicts": [verdict]}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    body["results"][0].clone()
}

fn code_quote() -> Value {
    json!({"code": "app/greeter.py", "quote": GREETER_LINE})
}

async fn accept(e: &Env, result: &Value) -> Value {
    let vd = result["verdict"]["handle"]
        .as_str()
        .unwrap_or_else(|| panic!("{result}"));
    let (status, body) = post_json(
        e,
        &format!("/v1/verdicts/{vd}/review"),
        json!({"reviewer": "owner", "agree": true}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["verdict"]["state"], "effective", "{body}");
    body
}

fn line_of<'a>(md: &'a str, needle: &str) -> &'a str {
    md.lines()
        .find(|l| l.contains(needle))
        .unwrap_or_else(|| panic!("no line with {needle:?}:\n{md}"))
}

#[tokio::test(flavor = "multi_thread")]
async fn a_review_run_is_handed_records_with_their_task_and_the_code_on_main() {
    let e = env().await;
    let none = start_dream(&e, "review", "").await;
    assert!(
        none["dream"].is_null() && none["items"] == json!([]),
        "{none}"
    );

    let (earlier, _) = two_tasks(&e).await;
    let rec = record_on(
        &e,
        &earlier,
        "decision",
        "hello never shouts",
        "app/greeter.py",
    )
    .await;
    record_on(
        &e,
        &earlier,
        "intent",
        "not a claim to judge",
        "app/greeter.py",
    )
    .await;
    let d = start_dream(&e, "review", "").await;
    assert_eq!(d["dream"]["status"], "running", "{d}");
    assert_eq!(d["dream"]["worklist"]["records"], json!([rec["id"]]), "{d}");
    let item = &d["items"][0];
    assert_eq!(item["record"]["id"], handle(&rec), "{item}");
    assert_eq!(item["record"]["standing"]["origin"], "open", "{item}");
    assert_eq!(item["task"]["slug"], earlier["task"]["slug"], "{item}");
    assert!(
        item["code"][0]["text"]
            .as_str()
            .unwrap()
            .contains(GREETER_LINE),
        "the anchored code as main has it: {item}"
    );
    // an area with none of the records has no run either
    let (status, body) = post_json(
        &e,
        "/v1/repos/demo/dreams",
        json!({"agent": "dreamer", "area": "docs"}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert!(body["dream"].is_null(), "{body}");
    let (status, _) = post_json(&e, "/v1/repos/nope/dreams", json!({"agent": "dreamer"})).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test(flavor = "multi_thread")]
async fn one_run_of_a_mode_per_area_at_a_time() {
    let e = env().await;
    let (earlier, _) = two_tasks(&e).await;
    record_on(
        &e,
        &earlier,
        "decision",
        "hello never shouts",
        "app/greeter.py",
    )
    .await;
    record_on(&e, &earlier, "note", "names are title case", "app/util.py").await;
    let whole = start_dream(&e, "review", "").await;
    let (status, body) = post_json(
        &e,
        "/v1/repos/demo/dreams",
        json!({"agent": "other", "area": "app"}),
    )
    .await;
    assert_eq!(
        status,
        StatusCode::CONFLICT,
        "an area inside a running one: {body}"
    );
    assert!(
        body["error"].as_str().unwrap().contains(&dream_id(&whole)),
        "{body}"
    );
    let (status, body) = post_json(
        &e,
        &format!("/v1/dreams/{}/finish", dream_id(&whole)),
        json!({}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["dream"]["status"], "done", "{body}");
    let d = start_dream(&e, "review", "app").await;
    assert_eq!(d["dream"]["area"], "app", "{d}");
    let (status, body) = post_json(
        &e,
        &format!("/v1/dreams/{}/verdicts", dream_id(&whole)),
        json!({"verdicts": []}),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
}

#[tokio::test(flavor = "multi_thread")]
async fn a_verdict_is_refused_unless_its_evidence_is_found_word_for_word() {
    let e = env().await;
    let (earlier, _) = two_tasks(&e).await;
    let rec = record_on(
        &e,
        &earlier,
        "decision",
        "hello never shouts",
        "app/greeter.py",
    )
    .await;
    let d = start_dream(&e, "review", "").await;
    let late = record_on(
        &e,
        &earlier,
        "note",
        "written after the run began",
        "app/util.py",
    )
    .await;
    let disputed = |evidence: Value| {
        json!({"record": handle(&rec), "verdict": "disputed",
               "reason": "the greeter adds no mark at all", "evidence": evidence})
    };
    for (verdict, why) in [
        (
            disputed(json!([{"code": "app/greeter.py", "quote": "return shout(name) + '!!!'"}])),
            "word for word",
        ),
        (
            disputed(json!([{"code": "app/missing.py", "quote": GREETER_LINE}])),
            "not a file",
        ),
        (
            disputed(json!([{"code": "app/greeter.py", "quote": "return"}])),
            "at least",
        ),
        (
            disputed(json!([{"record": handle(&rec), "quote": "hello never shouts"}])),
            "quote of code or an event",
        ),
        (disputed(json!([])), "quote of code or an event"),
        (disputed(json!([{"event": 999_999}])), "not an event"),
        (
            json!({"record": handle(&rec), "verdict": "disputed", "evidence": [code_quote()]}),
            "needs a reason",
        ),
        (
            json!({"record": handle(&rec), "verdict": "disputed", "reason": "x".repeat(201),
                   "evidence": [code_quote()]}),
            "at most 200",
        ),
        (
            json!({"record": handle(&rec), "verdict": "wrong", "evidence": [code_quote()]}),
            "must be confirmed",
        ),
        (
            json!({"record": handle(&rec), "verdict": "confirmed",
                   "evidence": [{"record": handle(&rec), "quote": "hello never shouts"}]}),
            "quote of the code",
        ),
        (
            json!({"record": handle(&late), "verdict": "confirmed", "evidence": [code_quote()]}),
            "not on this run's worklist",
        ),
    ] {
        let r = propose(&e, &d, verdict.clone()).await;
        assert_eq!(r["accepted"], false, "{verdict}: {r}");
        assert!(
            r["why"].as_str().unwrap().contains(why),
            "wanted {why:?}: {r}"
        );
    }
    let (_, listed) = call(&e.app, "GET", "/v1/repos/demo/verdicts", None).await;
    assert_eq!(
        listed["verdicts"],
        json!([]),
        "a refused verdict writes nothing"
    );

    let ok = propose(&e, &d, disputed(json!([code_quote()]))).await;
    assert_eq!(ok["accepted"], true, "{ok}");
    assert_eq!(ok["verdict"]["state"], "proposed", "{ok}");
    assert_eq!(ok["verdict"]["author_agent"], "dreamer", "{ok}");
    let again = propose(&e, &d, disputed(json!([code_quote()]))).await;
    assert!(
        again["why"].as_str().unwrap().contains("already judged"),
        "{again}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn a_verdict_changes_no_pack_until_a_second_look_agrees_and_undo_takes_it_back() {
    let e = env().await;
    let (earlier, reader) = two_tasks(&e).await;
    let rec = record_on(
        &e,
        &earlier,
        "decision",
        "hello never shouts",
        "app/greeter.py",
    )
    .await;
    let before = pack(&e, &reader, "p0").await;
    assert!(!before.contains("disputed"), "{before}");

    let d = start_dream(&e, "review", "").await;
    let proposed = propose(
        &e,
        &d,
        json!({"record": handle(&rec), "verdict": "disputed",
               "reason": "the greeter adds no mark at all", "evidence": [code_quote()]}),
    )
    .await;
    assert_eq!(proposed["accepted"], true, "{proposed}");
    assert_eq!(pack(&e, &reader, "p1").await, before, "proposed: no effect");

    let vd = accept(&e, &proposed).await["verdict"]["handle"]
        .as_str()
        .unwrap()
        .to_owned();
    let after = pack(&e, &reader, "p2").await;
    let line = line_of(&after, "hello never shouts");
    assert!(
        line.contains(&format!(
            "(disputed: the greeter adds no mark at all, {vd})"
        )),
        "{line}"
    );
    let (_, recs) = call(
        &e.app,
        "GET",
        &format!("/v1/tasks/{}/records", id(&earlier)),
        None,
    )
    .await;
    let stored = recs["records"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["id"] == rec["id"])
        .unwrap();
    assert_eq!(stored["summary"], "hello never shouts");
    assert_eq!(stored["verdict"]["kind"], "disputed", "{stored}");
    assert_eq!(stored["verdict"]["handle"], vd, "{stored}");
    assert!(
        stored["standing"]["factor"].as_f64().unwrap() < 0.5,
        "{stored}"
    );
    let (status, body) = post_json(
        &e,
        &format!("/v1/verdicts/{vd}/review"),
        json!({"reviewer": "owner", "agree": false}),
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT, "{body}");

    let (status, undone) = post_json(
        &e,
        &format!("/v1/dreams/{}/undo", dream_id(&d)),
        json!({"by": "owner"}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{undone}");
    assert_eq!(
        (&undone["dream"]["status"], &undone["retracted"]),
        (&json!("undone"), &json!(1))
    );
    assert_eq!(
        pack(&e, &reader, "p3").await,
        before,
        "undone: the earlier pack, byte for byte"
    );
    let (_, listed) = call(
        &e.app,
        "GET",
        "/v1/repos/demo/verdicts?state=retracted",
        None,
    )
    .await;
    assert_eq!(listed["verdicts"].as_array().unwrap().len(), 1, "{listed}");
}

/// A pack short of room shows a disputed record as its handle, `(disputed)` alone; expanding the
/// record says why and names the verdict.
#[tokio::test(flavor = "multi_thread")]
async fn expanding_a_disputed_record_gives_the_reason_and_the_verdict() {
    let e = env().await;
    let (earlier, reader) = two_tasks(&e).await;
    let rec = record_on(
        &e,
        &earlier,
        "decision",
        "hello never shouts",
        "app/greeter.py",
    )
    .await;
    let d = start_dream(&e, "review", "").await;
    let proposed = propose(
        &e,
        &d,
        json!({"record": handle(&rec), "verdict": "disputed",
               "reason": "the greeter adds no mark at all", "evidence": [code_quote()]}),
    )
    .await;
    let vd = accept(&e, &proposed).await["verdict"]["handle"]
        .as_str()
        .unwrap()
        .to_owned();
    let (status, body) = call(
        &e.app,
        "GET",
        &format!("/v1/tasks/{}/items/{}", id(&reader), handle(&rec)),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert!(
        body["full"].as_str().unwrap().contains(&format!(
            "(disputed: the greeter adds no mark at all, {vd})"
        )),
        "{body}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn a_verdict_a_person_rejects_has_no_effect() {
    let e = env().await;
    let (earlier, reader) = two_tasks(&e).await;
    let rec = record_on(
        &e,
        &earlier,
        "decision",
        "hello never shouts",
        "app/greeter.py",
    )
    .await;
    let before = pack(&e, &reader, "p0").await;
    let d = start_dream(&e, "review", "").await;
    let proposed = propose(
        &e,
        &d,
        json!({"record": handle(&rec), "verdict": "outdated", "evidence": [code_quote()]}),
    )
    .await;
    let vd = proposed["verdict"]["handle"].as_str().unwrap();
    let (status, body) = post_json(
        &e,
        &format!("/v1/verdicts/{vd}/review"),
        json!({"reviewer": "owner", "agree": false}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["verdict"]["state"], "rejected", "{body}");
    assert_eq!(pack(&e, &reader, "p1").await, before);
    let (status, one) = call(&e.app, "GET", &format!("/v1/verdicts/{vd}"), None).await;
    assert_eq!(status, StatusCode::OK, "{one}");
    assert_eq!(one["verdict"]["state"], "rejected", "{one}");
    assert_eq!(one["record"]["summary"], "hello never shouts", "{one}");
    assert_eq!(one["repo"], "demo", "{one}");
    let reviews = one["reviews"].as_array().expect("reviews");
    assert_eq!(reviews.len(), 1, "{one}");
    assert_eq!(reviews[0]["reviewer"], "owner", "{one}");
    assert_eq!(reviews[0]["agree"], false, "{one}");
    assert!(reviews[0]["dream"].is_null(), "{one}");
    assert_eq!(reviews[0]["evidence"], json!([]), "{one}");
}

#[tokio::test(flavor = "multi_thread")]
async fn a_duplicate_is_hidden_and_the_canonical_record_is_shown() {
    let e = env().await;
    let (earlier, reader) = two_tasks(&e).await;
    let kept = record_on(
        &e,
        &earlier,
        "decision",
        "hello never shouts",
        "app/greeter.py",
    )
    .await;
    let dup = record_on(
        &e,
        &earlier,
        "decision",
        "do not make hello shout",
        "app/greeter.py",
    )
    .await;
    let note = record_on(
        &e,
        &earlier,
        "note",
        "a note on the same file",
        "app/greeter.py",
    )
    .await;
    let d = start_dream(&e, "review", "").await;
    let both = json!([{"record": handle(&dup), "quote": "do not make hello shout"},
                      {"record": handle(&kept), "quote": "hello never shouts"}]);
    for (verdict, why) in [
        (
            json!({"record": handle(&dup), "verdict": "duplicate", "evidence": both}),
            "names the canonical",
        ),
        (
            json!({"record": handle(&dup), "verdict": "duplicate", "canonical": handle(&dup),
                   "evidence": both}),
            "of itself",
        ),
        (
            json!({"record": handle(&dup), "verdict": "duplicate", "canonical": handle(&note),
                   "evidence": both}),
            "same kind and anchor",
        ),
        (
            json!({"record": handle(&dup), "verdict": "duplicate", "canonical": handle(&kept),
                   "evidence": [{"record": handle(&dup), "quote": "do not make hello shout"}]}),
            "from each of the two",
        ),
    ] {
        let r = propose(&e, &d, verdict.clone()).await;
        assert!(
            r["why"].as_str().unwrap_or("").contains(why),
            "wanted {why:?}: {r}"
        );
    }
    let ok = propose(
        &e,
        &d,
        json!({"record": handle(&dup), "verdict": "duplicate", "canonical": handle(&kept),
               "evidence": both}),
    )
    .await;
    assert_eq!(ok["verdict"]["canonical"], kept["id"], "{ok}");
    accept(&e, &ok).await;

    let md = pack(&e, &reader, "p1").await;
    assert!(
        md.contains("hello never shouts") && !md.contains("do not make hello shout"),
        "{md}"
    );
    let (_, at) = call(
        &e.app,
        "GET",
        &format!("/v1/tasks/{}/records/at?path=app/greeter.py", id(&reader)),
        None,
    )
    .await;
    let at = at["markdown"].as_str().unwrap();
    assert!(
        at.contains("hello never shouts") && !at.contains("do not make hello shout"),
        "{at}"
    );
    // search still finds it, and says why it is not shown
    let (_, hits) = call(&e.app, "GET", "/v1/repos/demo/search?q=shout", None).await;
    let hit = hits["records"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["id"] == dup["id"])
        .unwrap_or_else(|| panic!("{hits}"));
    assert_eq!(hit["verdict"]["kind"], "duplicate", "{hit}");
    let (_, done) = post_json(
        &e,
        &format!("/v1/dreams/{}/finish", dream_id(&d)),
        json!({}),
    )
    .await;
    assert_eq!(done["dream"]["counts"]["duplicate"], 1, "{done}");
    let next = start_dream(&e, "review", "").await;
    let listed = next["dream"]["worklist"]["records"].as_array().unwrap();
    assert!(
        !listed.contains(&dup["id"]) && listed.contains(&kept["id"]),
        "{next}"
    );
    let r = propose(
        &e,
        &next,
        json!({"record": handle(&kept), "verdict": "duplicate", "canonical": handle(&dup),
               "evidence": both}),
    )
    .await;
    assert!(r["why"].as_str().unwrap().contains("itself hidden"), "{r}");
}

#[tokio::test(flavor = "multi_thread")]
async fn an_outdated_record_leaves_packs_and_a_retraction_brings_it_back() {
    let e = env().await;
    let (earlier, reader) = two_tasks(&e).await;
    let rec = record_on(
        &e,
        &earlier,
        "note",
        "hello used to cache names",
        "app/greeter.py",
    )
    .await;
    let before = pack(&e, &reader, "p0").await;
    let d = start_dream(&e, "review", "").await;
    let ok = propose(
        &e,
        &d,
        json!({"record": handle(&rec), "verdict": "outdated", "reason": "no cache in the greeter",
               "evidence": [code_quote()]}),
    )
    .await;
    let vd = accept(&e, &ok).await["verdict"]["handle"]
        .as_str()
        .unwrap()
        .to_owned();
    assert!(!pack(&e, &reader, "p1").await.contains("used to cache"));

    let (status, body) = post_json(
        &e,
        &format!("/v1/verdicts/{vd}/retract"),
        json!({"by": "owner"}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["verdict"]["state"], "retracted", "{body}");
    assert_eq!(pack(&e, &reader, "p2").await, before);
    let (status, body) = post_json(
        &e,
        &format!("/v1/verdicts/{vd}/retract"),
        json!({"by": "owner"}),
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT, "{body}");
}

#[tokio::test(flavor = "multi_thread")]
async fn a_confirmed_verdict_clears_the_changed_mark_and_such_records_are_looked_at_first() {
    let e = env().await;
    // the reader changes util.py and leaves the greeter as main has it: what a pack calls
    // changed is reckoned at the reader's own head
    let earlier = start_task(&e, "Earlier work").await;
    let reader = start_task(&e, "Upper names").await;
    std::fs::write(
        ws_dir(&e, &reader).join("app/util.py"),
        "def format_name(n):\n    return n.upper()\n",
    )
    .unwrap();
    let (status, cp) = post_json(
        &e,
        &format!("/v1/tasks/{}/checkpoint", id(&reader)),
        json!({}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{cp}");
    let plain = record_on(
        &e,
        &earlier,
        "decision",
        "names are title case",
        "app/util.py",
    )
    .await;
    let (status, moved) = post_json(
        &e,
        &format!("/v1/tasks/{}/records", id(&earlier)),
        json!({"kind": "decision", "summary": "hello never shouts", "agent": "claude",
               "anchors": [{"path": "app/greeter.py", "symbol": "Greeter.hello"}]}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{moved}");
    // written when the greeter looked different
    let moved_id = RecordId(moved["id"].as_str().unwrap().parse().unwrap());
    e.store
        .set_anchor_hashes(moved_id, &[Some("0".repeat(64))])
        .await
        .unwrap();
    assert!(
        line_of(&pack(&e, &reader, "p0").await, "hello never shouts")
            .contains("(changed since recorded)")
    );
    let d = start_dream(&e, "review", "").await;
    assert_eq!(
        d["dream"]["worklist"]["records"],
        json!([moved["id"], plain["id"]]),
        "the record whose code changed comes first: {d}"
    );
    let ok = propose(
        &e,
        &d,
        json!({"record": handle(&moved), "verdict": "confirmed", "evidence": [code_quote()]}),
    )
    .await;
    accept(&e, &ok).await;
    let md = pack(&e, &reader, "p1").await;
    assert!(
        !line_of(&md, "hello never shouts").contains("changed since recorded"),
        "{md}"
    );
}

/// A review run of `limit` records that finishes without a verdict: the run as started, and
async fn declined_run(e: &Env, limit: usize) -> (Value, Vec<Value>) {
    let (status, d) = post_json(
        e,
        "/v1/repos/demo/dreams",
        json!({"agent": "dreamer", "mode": "review", "limit": limit}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{d}");
    let handed = d["dream"]["worklist"]["records"]
        .as_array()
        .unwrap()
        .clone();
    let (status, body) =
        post_json(e, &format!("/v1/dreams/{}/finish", dream_id(&d)), json!({})).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    (d, handed)
}

#[tokio::test(flavor = "multi_thread")]
async fn a_record_a_finished_run_left_undecided_waits_behind_records_no_run_declined() {
    let e = env().await;
    let t = start_task(&e, "Earlier work").await;
    // two records that rank the same: one kind each, no shared anchor, no verdict
    let a = record_on(&e, &t, "decision", "greetings stay short", "app/greeter.py").await;
    let b = record_on(&e, &t, "note", "names keep their case", "app/util.py").await;
    let (d, handed) = declined_run(&e, 1).await;
    let first = handed[0].clone();
    assert!(first == a["id"] || first == b["id"], "{first}");
    let key = first.as_str().unwrap();
    assert_eq!(d["dream"]["worklist"]["scores"][key], json!(1.0), "{d}");
    let second = declined_run(&e, 1).await.1[0].clone();
    assert!(second == a["id"] || second == b["id"], "{second}");
    assert_ne!(first, second, "a declined record is not handed again first");
    assert_eq!(declined_run(&e, 1).await.1[0], first);
}

#[tokio::test(flavor = "multi_thread")]
async fn a_declined_record_leads_again_once_it_scores_higher_than_when_it_was_declined() {
    let e = env().await;
    let t = start_task(&e, "Earlier work").await;
    let a = record_on(&e, &t, "decision", "greetings stay short", "app/greeter.py").await;
    let b = record_on(&e, &t, "note", "names keep their case", "app/util.py").await;
    // one run is handed both and judges neither: both declined at 1.0
    let (_, handed) = declined_run(&e, 2).await;
    assert_eq!(handed.len(), 2, "{handed:?}");
    // a second decision on a's anchor: a and a2 now share it (3.0 each); d is a record no run
    // was handed, of another kind than b (1.0)
    let a2 = record_on(
        &e,
        &t,
        "decision",
        "greetings end with a stop",
        "app/greeter.py",
    )
    .await;
    let d = record_on(
        &e,
        &t,
        "attempt",
        "caching the names was slower",
        "app/util.py",
    )
    .await;
    let (_, handed) = declined_run(&e, 3).await;
    assert_eq!(handed.len(), 3, "{handed:?}");
    let mut front: Vec<&str> = handed[..2].iter().map(|v| v.as_str().unwrap()).collect();
    front.sort_unstable();
    let mut want = vec![a["id"].as_str().unwrap(), a2["id"].as_str().unwrap()];
    want.sort_unstable();
    assert_eq!(front, want, "{handed:?}");
    // b scores what it was declined at, so it waits behind d
    assert_eq!(handed[2], d["id"], "{handed:?}; b = {}", b["id"]);
}

/// The record a review run of limit 1 is handed after a person's review of `verdict` on an
/// abandoned task's decision, with a never-judged note written after that run.
async fn handed_after_a_review_of(verdict: Value, agree: bool) -> (Value, Value, Value) {
    let e = env().await;
    let earlier = start_task(&e, "Earlier work").await;
    let decision = record_on(
        &e,
        &earlier,
        "decision",
        "hello never shouts",
        "app/greeter.py",
    )
    .await;
    let (status, body) = post_json(
        &e,
        &format!("/v1/tasks/{}/abandon", id(&earlier)),
        json!({"agent": "claude"}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let d = start_dream(&e, "review", "").await;
    let mut verdict = verdict;
    verdict["record"] = json!(handle(&decision));
    let proposed = propose(&e, &d, verdict).await;
    let vd = proposed["verdict"]["handle"]
        .as_str()
        .unwrap_or_else(|| panic!("{proposed}"));
    let (status, body) = post_json(
        &e,
        &format!("/v1/verdicts/{vd}/review"),
        json!({"reviewer": "owner", "agree": agree}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let (status, body) = post_json(
        &e,
        &format!("/v1/dreams/{}/finish", dream_id(&d)),
        json!({}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");

    let later = start_task(&e, "Later work").await;
    let note = record_on(&e, &later, "note", "names keep their case", "app/util.py").await;
    let (_, handed) = declined_run(&e, 1).await;
    (decision, note, handed[0].clone())
}

#[tokio::test(flavor = "multi_thread")]
async fn a_judged_record_no_longer_leads_the_worklist_for_its_abandoned_task() {
    for verdict in [
        json!({"verdict": "disputed", "reason": "the greeter adds no mark at all",
               "evidence": [code_quote()]}),
        json!({"verdict": "confirmed", "evidence": [code_quote()]}),
    ] {
        let (_, note, handed) = handed_after_a_review_of(verdict.clone(), true).await;
        assert_eq!(handed, note["id"], "after an agreed {verdict}");
    }
    let (decision, _, handed) = handed_after_a_review_of(
        json!({"verdict": "disputed", "reason": "the greeter adds no mark at all",
               "evidence": [code_quote()]}),
        false,
    )
    .await;
    assert_eq!(
        handed, decision["id"],
        "a rejected verdict has not judged the record"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn a_verify_run_reviews_what_another_run_proposed_with_a_quote_of_its_own() {
    let e = env().await;
    let (earlier, reader) = two_tasks(&e).await;
    let rec = record_on(
        &e,
        &earlier,
        "decision",
        "hello never shouts",
        "app/greeter.py",
    )
    .await;
    assert!(start_dream(&e, "verify", "").await["dream"].is_null());
    let d = start_dream(&e, "review", "").await;
    let proposed = propose(
        &e,
        &d,
        json!({"record": handle(&rec), "verdict": "disputed",
               "reason": "the greeter adds no mark at all", "evidence": [code_quote()]}),
    )
    .await;
    let vd = proposed["verdict"]["handle"].as_str().unwrap();
    let path = format!("/v1/verdicts/{vd}/review");
    let review = |body: Value| post_json(&e, &path, body);

    // the run that proposed it is not a second look
    let (status, body) = review(json!({"reviewer": "dreamer", "agree": true,
                                       "dream": dream_id(&d), "evidence": [code_quote()]}))
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");

    let v = start_dream(&e, "verify", "").await;
    assert_eq!(
        v["dream"]["worklist"]["verdicts"],
        json!([proposed["verdict"]["id"]]),
        "{v}"
    );
    let item = &v["items"][0];
    assert_eq!(item["verdict"]["handle"], vd, "{item}");
    assert_eq!(
        item["verdict"]["evidence"][0]["quote"], GREETER_LINE,
        "{item}"
    );
    assert_eq!(item["record"]["summary"], "hello never shouts", "{item}");

    let (status, body) = review(json!({"reviewer": "checker", "agree": true,
                                       "dream": dream_id(&v)}))
    .await;
    assert_eq!(
        status,
        StatusCode::BAD_REQUEST,
        "no quote of its own: {body}"
    );
    let (status, body) = review(
        json!({"reviewer": "checker", "agree": true, "dream": dream_id(&v),
        "evidence": [{"code": "app/greeter.py", "quote": "nothing like this is in the file"}]}),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "{body}");
    assert!(!pack(&e, &reader, "p0").await.contains("disputed"));

    let (status, body) = review(
        json!({"reviewer": "checker", "agree": true, "dream": dream_id(&v),
        "evidence": [{"code": "app/greeter.py", "quote": "def hello(self, name):"}]}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["verdict"]["state"], "effective", "{body}");
    assert!(
        pack(&e, &reader, "p1")
            .await
            .contains("(disputed: the greeter adds no mark")
    );
    let (status, one) = call(&e.app, "GET", &format!("/v1/verdicts/{vd}"), None).await;
    assert_eq!(status, StatusCode::OK, "{one}");
    assert_eq!(
        one["reviews"],
        json!([{"reviewer": "checker", "agree": true, "dream": dream_id(&v),
                "evidence": [{"code": "app/greeter.py", "quote": "def hello(self, name):"}],
                "created_at": one["reviews"][0]["created_at"]}]),
        "{one}"
    );

    let (_, feed) = call(
        &e.app,
        "GET",
        "/v1/events?repo=demo&since=0&limit=200",
        None,
    )
    .await;
    let kinds: Vec<&str> = feed["events"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|ev| ev["kind"].as_str())
        .filter(|k| k.starts_with("dream") || *k == "verdict")
        .collect();
    assert_eq!(
        kinds,
        ["dream:started", "verdict", "dream:started", "verdict"],
        "{feed}"
    );
}

/// A guard with a policy for shared memory: only `owner` reviews verdicts, `grok` supersedes nothing.
#[derive(Default)]
struct MemoryPolicy(std::sync::Mutex<Vec<String>>);

#[async_trait::async_trait]
impl margincode_daemon::MergeGuard for MemoryPolicy {
    async fn check(&self, action: margincode_daemon::Guarded<'_>) -> Result<(), String> {
        use margincode_daemon::MemoryAction;
        let margincode_daemon::Guarded::MemoryWrite {
            repo,
            action,
            by,
            dream,
        } = action
        else {
            return Ok(());
        };
        let (what, refused) = match action {
            MemoryAction::Supersede { record } => {
                (format!("supersede {:?}", record.summary), by == "grok")
            }
            MemoryAction::VerdictReview {
                kind,
                record,
                agree,
                ..
            } => (
                format!("review {kind} of {:?} agree={agree}", record.summary),
                by != "owner",
            ),
            MemoryAction::Rule { .. } => return Ok(()),
        };
        let from = if dream.is_some() { " from a run" } else { "" };
        self.0
            .lock()
            .unwrap()
            .push(format!("{repo}: {what} by {by}{from}"));
        if refused {
            Err(format!("{by} may not change shared memory"))
        } else {
            Ok(())
        }
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn a_guard_can_refuse_a_memory_write_and_nothing_is_written() {
    let guard = std::sync::Arc::new(MemoryPolicy::default());
    let g = guard.clone();
    let e = env_with(move |s| s.with_merge_guard(g)).await;
    let (earlier, reader) = two_tasks(&e).await;
    let rec = record_on(
        &e,
        &earlier,
        "decision",
        "hello never shouts",
        "app/greeter.py",
    )
    .await;

    // a verdict's review: refused, the verdict stays proposed and may still be reviewed
    let d = start_dream(&e, "review", "").await;
    let proposed = propose(
        &e,
        &d,
        json!({"record": handle(&rec), "verdict": "outdated", "evidence": [code_quote()]}),
    )
    .await;
    let vd = proposed["verdict"]["handle"].as_str().unwrap();
    let v = start_dream(&e, "verify", "").await;
    let (status, body) = post_json(
        &e,
        &format!("/v1/verdicts/{vd}/review"),
        json!({"reviewer": "checker", "agree": true, "dream": dream_id(&v),
               "evidence": [{"code": "app/greeter.py", "quote": "def hello(self, name):"}]}),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
    assert!(
        body["error"]
            .as_str()
            .unwrap()
            .contains("checker may not change shared memory"),
        "{body}"
    );
    let (_, one) = call(&e.app, "GET", &format!("/v1/verdicts/{vd}"), None).await;
    assert_eq!(one["verdict"]["state"], "proposed", "{one}");
    // a review the daemon's own rules refuse never reaches the guard
    let (status, _) = post_json(
        &e,
        &format!("/v1/verdicts/{vd}/review"),
        json!({"reviewer": "checker", "agree": true, "dream": dream_id(&v)}),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    let (status, body) = post_json(
        &e,
        &format!("/v1/verdicts/{vd}/review"),
        json!({"reviewer": "owner", "agree": true}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["verdict"]["state"], "effective", "{body}");
    // reviewed once: the guard is not asked again
    let (status, _) = post_json(
        &e,
        &format!("/v1/verdicts/{vd}/review"),
        json!({"reviewer": "owner", "agree": false}),
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT);

    // a supersede: refused, no record is written and the old one stays live
    let other = record_on(
        &e,
        &earlier,
        "note",
        "the greeter is sync",
        "app/greeter.py",
    )
    .await;
    let path = format!("/v1/tasks/{}/records", id(&reader));
    let correction = |agent: &str| {
        json!({"kind": "note", "summary": "the greeter is async now", "agent": agent,
               "supersedes": handle(&other)})
    };
    let (status, body) = post_json(&e, &path, correction("grok")).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
    assert!(
        body["error"]
            .as_str()
            .unwrap()
            .contains("cannot supersede rec:"),
        "{body}"
    );
    let live = e
        .store
        .get_record(RecordId(other["id"].as_str().unwrap().parse().unwrap()))
        .await
        .unwrap();
    assert!(live.superseded_by.is_none(), "{live:?}");
    let (status, body) = post_json(&e, &path, correction("claude")).await;
    assert_eq!(status, StatusCode::OK, "{body}");

    assert_eq!(
        *guard.0.lock().unwrap(),
        [
            "demo: review outdated of \"hello never shouts\" agree=true by checker from a run",
            "demo: review outdated of \"hello never shouts\" agree=true by owner",
            "demo: supersede \"the greeter is sync\" by grok",
            "demo: supersede \"the greeter is sync\" by claude",
        ]
    );
}

/// What an agent on a later task sees of an abandoned task's records, one disputed and one
/// outdated by accepted verdicts.
struct Seen {
    plain_flag: Value,
    first: String,
    after: String,
    stored: Value,
    at: String,
    foreign_supersede: StatusCode,
    refusal: String,
    own_supersede: StatusCode,
}

async fn seen_by_a_later_task(plain: bool) -> Seen {
    let e = env_with(|s| s.with_plain_records(plain)).await;
    let (_, version) = call(&e.app, "GET", "/v1/version", None).await;
    let (earlier, reader) = two_tasks(&e).await;
    let disputed = record_on(
        &e,
        &earlier,
        "decision",
        "hello never shouts",
        "app/greeter.py",
    )
    .await;
    let outdated = record_on(
        &e,
        &earlier,
        "note",
        "hello used to cache names",
        "app/greeter.py",
    )
    .await;
    let (status, body) = post_json(
        &e,
        &format!("/v1/tasks/{}/abandon", id(&earlier)),
        json!({"agent": "claude"}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let first = pack(&e, &reader, "p0").await;

    let d = start_dream(&e, "review", "").await;
    for (rec, verdict, reason) in [
        (&disputed, "disputed", "the greeter adds no mark at all"),
        (&outdated, "outdated", "no cache in the greeter"),
    ] {
        let proposed = propose(
            &e,
            &d,
            json!({"record": handle(rec), "verdict": verdict, "reason": reason,
                   "evidence": [code_quote()]}),
        )
        .await;
        accept(&e, &proposed).await;
    }
    let after = pack(&e, &reader, "p1").await;

    let (_, recs) = call(
        &e.app,
        "GET",
        &format!("/v1/tasks/{}/records", id(&earlier)),
        None,
    )
    .await;
    let stored = recs["records"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["id"] == disputed["id"])
        .unwrap()
        .clone();
    let (status, at) = call(
        &e.app,
        "GET",
        &format!("/v1/tasks/{}/records/at?path=app/greeter.py", id(&reader)),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{at}");

    let (foreign_supersede, refusal) = post_json(
        &e,
        &format!("/v1/tasks/{}/records", id(&reader)),
        json!({"kind": "decision", "summary": "hello shouts after all", "agent": "claude",
               "supersedes": handle(&disputed)}),
    )
    .await;
    let own = record_on(&e, &reader, "note", "hello is loud", "app/greeter.py").await;
    let (own_supersede, body) = post_json(
        &e,
        &format!("/v1/tasks/{}/records", id(&reader)),
        json!({"kind": "note", "summary": "hello is very loud", "agent": "claude",
               "supersedes": handle(&own)}),
    )
    .await;
    assert_eq!(own_supersede, StatusCode::OK, "{body}");
    Seen {
        plain_flag: version["plain_records"].clone(),
        first,
        after,
        stored,
        at: at["markdown"].as_str().unwrap().to_owned(),
        foreign_supersede,
        refusal: refusal.to_string(),
        own_supersede,
    }
}

/// `AV_PLAIN_RECORDS`: packs show records as they were before an agent could correct another
/// task's records and before a record carried its task's outcome; verdicts still act.
#[tokio::test(flavor = "multi_thread")]
async fn plain_records_keep_verdicts_but_drop_origins_and_corrections() {
    let full = seen_by_a_later_task(false).await;
    let plain = seen_by_a_later_task(true).await;
    assert_eq!(
        (&full.plain_flag, &plain.plain_flag),
        (&json!(false), &json!(true))
    );

    let label = |md: &str| line_of(md, "hello never shouts").contains(" decision, abandoned: ");
    assert!(label(&full.first) && label(&full.after), "{}", full.after);
    assert!(
        !label(&plain.first) && !label(&plain.after),
        "{}",
        plain.after
    );
    assert!(full.first.contains("--supersedes"), "{}", full.first);
    assert!(!plain.first.contains("--supersedes"), "{}", plain.first);
    assert!(full.at.contains("--supersedes"), "{}", full.at);
    assert!(
        plain.at.contains("hello never shouts") && !plain.at.contains("--supersedes"),
        "{}",
        plain.at
    );

    for seen in [&full, &plain] {
        let line = line_of(&seen.after, "hello never shouts");
        assert!(
            line.contains("(disputed: the greeter adds no mark"),
            "{line}"
        );
        assert!(!seen.after.contains("used to cache"), "{}", seen.after);
        assert_eq!(
            seen.stored["verdict"]["kind"], "disputed",
            "{}",
            seen.stored
        );
        assert_eq!(seen.own_supersede, StatusCode::OK);
    }
    assert_eq!(
        full.stored["standing"]["origin"], "abandoned",
        "{}",
        full.stored
    );
    assert!(plain.stored.get("standing").is_none(), "{}", plain.stored);

    assert_eq!(full.foreign_supersede, StatusCode::OK, "{}", full.refusal);
    assert_eq!(
        plain.foreign_supersede,
        StatusCode::UNPROCESSABLE_ENTITY,
        "{}",
        plain.refusal
    );
    assert!(
        plain.refusal.contains("AV_PLAIN_RECORDS"),
        "{}",
        plain.refusal
    );
}

const RULE: &str = "greetings in app/ never shout";

fn promote_of(rec: &Value, text: &str, glob: &str) -> Value {
    json!({"record": handle(rec), "verdict": "promote", "text": text, "scope_glob": glob,
           "evidence": [{"record": handle(rec), "quote": "hello never shouts"}]})
}

async fn conventions(e: &Env) -> Vec<Value> {
    let (status, body) = call(&e.app, "GET", "/v1/repos/demo/conventions", None).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    body["conventions"].as_array().unwrap().clone()
}

async fn person_review(e: &Env, vd: &str, agree: bool) -> (StatusCode, Value) {
    post_json(
        e,
        &format!("/v1/verdicts/{vd}/review"),
        json!({"reviewer": "owner", "agree": agree}),
    )
    .await
}

#[tokio::test(flavor = "multi_thread")]
async fn a_promote_is_refused_off_a_decision_without_text_or_with_a_bad_glob() {
    let e = env().await;
    let (earlier, _) = two_tasks(&e).await;
    let rec = record_on(
        &e,
        &earlier,
        "decision",
        "hello never shouts",
        "app/greeter.py",
    )
    .await;
    let note = record_on(&e, &earlier, "note", "names are title case", "app/util.py").await;
    let d = start_dream(&e, "review", "").await;
    let on_note = json!({"record": handle(&note), "verdict": "promote", "text": RULE,
                         "scope_glob": "app/**",
                         "evidence": [{"record": handle(&note), "quote": "names are title case"}]});
    let mut no_quote = promote_of(&rec, RULE, "app/**");
    no_quote["evidence"] = json!([code_quote()]);
    let mut no_glob = promote_of(&rec, RULE, "");
    no_glob.as_object_mut().unwrap().remove("scope_glob");
    for (verdict, why) in [
        (on_note, "decision"),
        (promote_of(&rec, "  ", "app/**"), "text"),
        (no_glob, "scope_glob"),
        (promote_of(&rec, RULE, "["), "bad glob"),
        (no_quote, "quote of the decision"),
        (
            json!({"record": handle(&rec), "verdict": "wrong", "evidence": [code_quote()]}),
            "promote",
        ),
    ] {
        let r = propose(&e, &d, verdict.clone()).await;
        assert_eq!(r["accepted"], false, "{verdict}: {r}");
        assert!(
            r["why"].as_str().unwrap().contains(why),
            "wanted {why:?}: {r}"
        );
    }
    let (_, listed) = call(&e.app, "GET", "/v1/repos/demo/verdicts", None).await;
    assert_eq!(listed["verdicts"], json!([]), "{listed}");
}

#[tokio::test(flavor = "multi_thread")]
async fn a_promote_takes_effect_only_when_a_review_without_a_run_accepts_it() {
    let e = env().await;
    let (earlier, reader) = two_tasks(&e).await;
    let rec = record_on(
        &e,
        &earlier,
        "decision",
        "hello never shouts",
        "app/greeter.py",
    )
    .await;
    let other = record_on(
        &e,
        &earlier,
        "note",
        "names are title case",
        "app/greeter.py",
    )
    .await;
    let before = pack(&e, &reader, "p0").await;
    assert!(!before.contains(RULE), "{before}");

    let d = start_dream(&e, "review", "").await;
    let promoted = propose(&e, &d, promote_of(&rec, RULE, "app/**")).await;
    assert_eq!(promoted["accepted"], true, "{promoted}");
    assert_eq!(promoted["verdict"]["verdict"], "promote", "{promoted}");
    assert_eq!(
        promoted["verdict"]["convention"]["text"], RULE,
        "{promoted}"
    );
    assert_eq!(
        promoted["verdict"]["convention"]["scope_glob"], "app/**",
        "{promoted}"
    );
    let disputed = propose(
        &e,
        &d,
        json!({"record": handle(&other), "verdict": "disputed",
               "reason": "names are kept as given", "evidence": [code_quote()]}),
    )
    .await;
    assert_eq!(disputed["accepted"], true, "{disputed}");
    assert_eq!(pack(&e, &reader, "p1").await, before, "proposed: no effect");

    // a verify run is not handed the promote, and its agreement is refused
    let v = start_dream(&e, "verify", "").await;
    assert_eq!(
        v["dream"]["worklist"]["verdicts"],
        json!([disputed["verdict"]["id"]]),
        "{v}"
    );
    let vd = promoted["verdict"]["handle"].as_str().unwrap();
    let (status, body) = post_json(
        &e,
        &format!("/v1/verdicts/{vd}/review"),
        json!({"reviewer": "checker", "agree": true, "dream": dream_id(&v),
               "evidence": [code_quote()]}),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
    assert!(
        body["error"].as_str().unwrap().contains("without `dream`"),
        "{body}"
    );
    let (_, one) = call(&e.app, "GET", &format!("/v1/verdicts/{vd}"), None).await;
    assert_eq!(one["verdict"]["state"], "proposed", "{one}");
    assert_eq!(one["reviews"], json!([]), "{one}");
    assert!(conventions(&e).await.is_empty());
    let p2 = pack(&e, &reader, "p2").await;
    assert!(!p2.contains(RULE), "{p2}");

    let (status, body) = person_review(&e, vd, true).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["verdict"]["state"], "effective", "{body}");
    let conv = conventions(&e).await;
    assert_eq!(conv.len(), 1, "{conv:?}");
    assert_eq!(conv[0]["text"], RULE);
    assert_eq!(conv[0]["scope_glob"], "app/**");
    let full = promoted["verdict"]["id"].as_str().unwrap().replace('-', "");
    assert_eq!(conv[0]["source"], format!("vd:{full}"));
    let after = pack(&e, &reader, "p3").await;
    assert!(after.contains(RULE), "the next pack carries it:\n{after}");
    // the decision itself is shown as before: a promote judges no record
    assert_eq!(
        line_of(&after, "hello never shouts"),
        line_of(&before, "hello never shouts")
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn a_promote_a_person_rejects_writes_no_convention() {
    let e = env().await;
    let (earlier, reader) = two_tasks(&e).await;
    let rec = record_on(
        &e,
        &earlier,
        "decision",
        "hello never shouts",
        "app/greeter.py",
    )
    .await;
    let d = start_dream(&e, "review", "").await;
    let promoted = propose(&e, &d, promote_of(&rec, RULE, "app/**")).await;
    let vd = promoted["verdict"]["handle"].as_str().unwrap();
    let (status, body) = person_review(&e, vd, false).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["verdict"]["state"], "rejected", "{body}");
    assert!(conventions(&e).await.is_empty());
    assert!(!pack(&e, &reader, "p0").await.contains(RULE));
}

#[tokio::test(flavor = "multi_thread")]
async fn retract_and_undo_take_a_promoted_convention_out_and_leave_a_hand_written_one() {
    let e = env().await;
    let (earlier, reader) = two_tasks(&e).await;
    let rec = record_on(
        &e,
        &earlier,
        "decision",
        "hello never shouts",
        "app/greeter.py",
    )
    .await;
    let (status, hand) = post_json(
        &e,
        "/v1/repos/demo/conventions",
        json!({"scope_glob": "app/**", "text": "a hand-written rule stays"}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{hand}");
    let only_hand = |conv: &[Value]| {
        assert_eq!(conv.len(), 1, "{conv:?}");
        assert_eq!(conv[0]["text"], "a hand-written rule stays");
        assert_eq!(conv[0]["source"], "");
    };

    // retract
    let d = start_dream(&e, "review", "").await;
    let promoted = propose(&e, &d, promote_of(&rec, RULE, "app/**")).await;
    let vd = promoted["verdict"]["handle"].as_str().unwrap().to_owned();
    person_review(&e, &vd, true).await;
    post_json(
        &e,
        &format!("/v1/dreams/{}/finish", dream_id(&d)),
        json!({}),
    )
    .await;
    assert_eq!(conventions(&e).await.len(), 2);
    assert!(pack(&e, &reader, "p0").await.contains(RULE));
    let (status, body) = post_json(
        &e,
        &format!("/v1/verdicts/{vd}/retract"),
        json!({"by": "owner"}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    only_hand(&conventions(&e).await);
    let p1 = pack(&e, &reader, "p1").await;
    assert!(
        !p1.contains(RULE) && p1.contains("a hand-written rule stays"),
        "{p1}"
    );

    // dream undo
    let d = start_dream(&e, "review", "").await;
    let promoted = propose(&e, &d, promote_of(&rec, RULE, "app/**")).await;
    assert_eq!(promoted["accepted"], true, "{promoted}");
    person_review(&e, promoted["verdict"]["handle"].as_str().unwrap(), true).await;
    post_json(
        &e,
        &format!("/v1/dreams/{}/finish", dream_id(&d)),
        json!({}),
    )
    .await;
    assert_eq!(conventions(&e).await.len(), 2);
    let (status, body) = post_json(
        &e,
        &format!("/v1/dreams/{}/undo", dream_id(&d)),
        json!({"by": "owner"}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    only_hand(&conventions(&e).await);
    let p2 = pack(&e, &reader, "p2").await;
    assert!(
        !p2.contains(RULE) && p2.contains("a hand-written rule stays"),
        "{p2}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn a_promotes_acceptance_is_the_guards_memory_write_question() {
    let guard = std::sync::Arc::new(MemoryPolicy::default());
    let g = guard.clone();
    let e = env_with(move |s| s.with_merge_guard(g)).await;
    let (earlier, _) = two_tasks(&e).await;
    let rec = record_on(
        &e,
        &earlier,
        "decision",
        "hello never shouts",
        "app/greeter.py",
    )
    .await;
    let d = start_dream(&e, "review", "").await;
    let promoted = propose(&e, &d, promote_of(&rec, RULE, "app/**")).await;
    let vd = promoted["verdict"]["handle"].as_str().unwrap();
    let (status, body) = post_json(
        &e,
        &format!("/v1/verdicts/{vd}/review"),
        json!({"reviewer": "someone", "agree": true}),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
    assert!(
        conventions(&e).await.is_empty(),
        "a refused review writes nothing"
    );
    let (status, body) = person_review(&e, vd, true).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(conventions(&e).await.len(), 1);
    assert_eq!(
        *guard.0.lock().unwrap(),
        [
            "demo: review promote of \"hello never shouts\" agree=true by someone",
            "demo: review promote of \"hello never shouts\" agree=true by owner",
        ]
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn a_promote_is_accepted_under_a_registered_agents_name() {
    let e = env().await;
    let (earlier, _) = two_tasks(&e).await;
    let rec = record_on(
        &e,
        &earlier,
        "decision",
        "hello never shouts",
        "app/greeter.py",
    )
    .await;
    let (status, body) = call(
        &e.app,
        "PUT",
        "/v1/agents/grok",
        Some(json!({"best": ["image"], "can": [], "cost_rank": 2, "aliases": ["grok-cli"]})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let d = start_dream(&e, "review", "").await;
    let promoted = propose(&e, &d, promote_of(&rec, RULE, "app/**")).await;
    let vd = promoted["verdict"]["handle"].as_str().unwrap();
    let (status, body) = post_json(
        &e,
        &format!("/v1/verdicts/{vd}/review"),
        json!({"reviewer": "GROK-cli", "agree": true}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["verdict"]["state"], "effective", "{body}");
    assert_eq!(conventions(&e).await.len(), 1);
}

#[tokio::test(flavor = "multi_thread")]
async fn the_agent_that_proposed_a_promote_never_reviews_it() {
    let e = env().await;
    let (earlier, _) = two_tasks(&e).await;
    let rec = record_on(
        &e,
        &earlier,
        "decision",
        "hello never shouts",
        "app/greeter.py",
    )
    .await;
    let d = start_dream(&e, "review", "").await;
    let promoted = propose(&e, &d, promote_of(&rec, RULE, "app/**")).await;
    assert_eq!(promoted["verdict"]["author_agent"], "dreamer", "{promoted}");
    let vd = promoted["verdict"]["handle"].as_str().unwrap();
    // the verdict's author, which is also the proposing run's agent, in any case
    for agent in ["dreamer", "Dreamer", "DREAMER"] {
        for agree in [true, false] {
            let (status, body) = post_json(
                &e,
                &format!("/v1/verdicts/{vd}/review"),
                json!({"reviewer": agent, "agree": agree}),
            )
            .await;
            assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{agent}: {body}");
            let why = body["error"].as_str().unwrap();
            assert!(
                why.contains("proposed it") && why.contains("--agent") && why.contains("AV_AGENT"),
                "{agent}: {body}"
            );
        }
    }
    let (_, one) = call(&e.app, "GET", &format!("/v1/verdicts/{vd}"), None).await;
    assert_eq!(one["verdict"]["state"], "proposed", "{one}");
    assert_eq!(one["reviews"], json!([]), "{one}");
    assert!(conventions(&e).await.is_empty());
    let (status, body) = post_json(
        &e,
        &format!("/v1/verdicts/{vd}/review"),
        json!({"reviewer": "checker", "agree": false}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["verdict"]["state"], "rejected", "{body}");
    assert!(conventions(&e).await.is_empty());
}

#[tokio::test(flavor = "multi_thread")]
async fn waiting_promotes_do_not_crowd_a_verify_run_out() {
    let e = env().await;
    let (earlier, _) = two_tasks(&e).await;
    let rec = record_on(
        &e,
        &earlier,
        "decision",
        "hello never shouts",
        "app/greeter.py",
    )
    .await;
    let d = start_dream(&e, "review", "").await;
    let disputed = propose(
        &e,
        &d,
        json!({"record": handle(&rec), "verdict": "disputed",
               "reason": "the greeter adds no mark at all", "evidence": [code_quote()]}),
    )
    .await;
    assert_eq!(disputed["accepted"], true, "{disputed}");
    // more waiting promotes, all newer than it, than a verify run's window holds
    sqlx::query(
        "INSERT INTO record_verdicts (id, record_id, verdict, author_agent, convention_text,
                                      scope_glob, created_at)
         SELECT gen_random_uuid(), $1, 'promote', 'dreamer', 'rule ' || n, 'app/**',
                now() + make_interval(secs => n)
           FROM generate_series(1, 1200) n",
    )
    .bind(rec["id"].as_str().unwrap().parse::<uuid::Uuid>().unwrap())
    .execute(e.store.pool())
    .await
    .unwrap();
    let v = start_dream(&e, "verify", "").await;
    assert_eq!(
        v["dream"]["worklist"]["verdicts"],
        json!([disputed["verdict"]["id"]]),
        "{v}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn a_promotes_text_is_at_most_400_characters() {
    let e = env().await;
    let (earlier, _) = two_tasks(&e).await;
    let rec = record_on(
        &e,
        &earlier,
        "decision",
        "hello never shouts",
        "app/greeter.py",
    )
    .await;
    let d = start_dream(&e, "review", "").await;
    let r = propose(&e, &d, promote_of(&rec, &"x".repeat(401), "app/**")).await;
    assert_eq!(r["accepted"], false, "{r}");
    assert!(r["why"].as_str().unwrap().contains("at most 400"), "{r}");
    let r = propose(&e, &d, promote_of(&rec, &"x".repeat(400), "app/**")).await;
    assert_eq!(r["accepted"], true, "{r}");
    // the store holds to it too, and to which rows carry a convention at all
    let rid: uuid::Uuid = rec["id"].as_str().unwrap().parse().unwrap();
    for (verdict, text, glob) in [
        ("promote", Some("x".repeat(401)), Some("app/**")),
        ("confirmed", Some("a rule".to_owned()), None),
        ("confirmed", None, Some("app/**")),
        ("confirmed", Some("a rule".to_owned()), Some("app/**")),
    ] {
        let wrote = sqlx::query(
            "INSERT INTO record_verdicts (id, record_id, verdict, author_agent, convention_text,
                                          scope_glob)
             VALUES (gen_random_uuid(), $1, $2, 'dreamer', $3, $4)",
        )
        .bind(rid)
        .bind(verdict)
        .bind(&text)
        .bind(glob)
        .execute(e.store.pool())
        .await;
        assert!(wrote.is_err(), "{verdict} {text:?} {glob:?} was stored");
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn a_decision_has_one_promote_at_a_time_and_a_superseded_one_is_not_accepted() {
    let e = env().await;
    let (earlier, reader) = two_tasks(&e).await;
    let rec = record_on(
        &e,
        &earlier,
        "decision",
        "hello never shouts",
        "app/greeter.py",
    )
    .await;
    let d = start_dream(&e, "review", "").await;
    let first = propose(&e, &d, promote_of(&rec, RULE, "app/**")).await;
    let vd = first["verdict"]["handle"].as_str().unwrap().to_owned();
    post_json(
        &e,
        &format!("/v1/dreams/{}/finish", dream_id(&d)),
        json!({}),
    )
    .await;
    // a run is not handed a record whose verdict waits; once a person accepted it, a later
    // run is handed the record again, and a second promote of it is refused
    let (status, body) = person_review(&e, &vd, true).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let d2 = start_dream(&e, "review", "").await;
    let r = propose(&e, &d2, promote_of(&rec, "another rule", "app/**")).await;
    assert_eq!(r["accepted"], false, "{r}");
    assert!(r["why"].as_str().unwrap().contains("already"), "{r}");
    post_json(
        &e,
        &format!("/v1/dreams/{}/finish", dream_id(&d2)),
        json!({}),
    )
    .await;
    // once the first is retracted, the decision may be promoted again
    post_json(
        &e,
        &format!("/v1/verdicts/{vd}/retract"),
        json!({"by": "owner"}),
    )
    .await;
    let d3 = start_dream(&e, "review", "").await;
    let r = propose(&e, &d3, promote_of(&rec, "another rule", "app/**")).await;
    assert_eq!(r["accepted"], true, "{r}");
    let vd3 = r["verdict"]["handle"].as_str().unwrap().to_owned();
    // the decision is superseded before a person looks: the promote is not accepted
    let (status, body) = post_json(
        &e,
        &format!("/v1/tasks/{}/records", id(&reader)),
        json!({"kind": "decision", "summary": "hello may shout now", "agent": "claude",
               "supersedes": handle(&rec), "anchors": [{"path": "app/greeter.py"}]}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let (status, body) = person_review(&e, &vd3, true).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
    assert!(
        body["error"].as_str().unwrap().contains("superseded"),
        "{body}"
    );
    assert!(conventions(&e).await.is_empty());
}
