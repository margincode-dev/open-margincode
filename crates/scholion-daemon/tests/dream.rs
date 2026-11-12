//! Plan 16 Phase 2: dreams and verdicts, with hand-written verdicts and no model. The daemon
//! chooses what a run may judge, checks every quote word for word, and gives a verdict no
//! effect until a second look agrees.

use crate::common::*;
use scholion_core::{IntentStore, RecordId};

/// A line of `main`'s `app/greeter.py`, long enough to be a quote.
const GREETER_LINE: &str = "return format_name(name)";

async fn post_json(e: &Env, path: &str, body: Value) -> (StatusCode, Value) {
    call(&e.app, "POST", path, Some(body)).await
}

/// A record of task `t` anchored on `path`.
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

/// An earlier task with records on the greeter, and a later task that edits the greeter, so
/// its packs show them.
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

/// Posts one verdict to a review run and answers its result entry.
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

/// A person agrees with a proposed verdict.
async fn accept(e: &Env, result: &Value) -> Value {
    let vd = result["verdict"]["handle"]
        .as_str()
        .unwrap_or_else(|| panic!("{result}"));
    let (status, body) = post_json(
        e,
        &format!("/v1/verdicts/{vd}/review"),
        json!({"reviewer": "vinh", "agree": true}),
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
    // nothing to judge: no run is made
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
    // a finished run takes no more verdicts
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
    // the record itself is untouched, and its routes say what was found
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
    // a verdict is reviewed once
    let (status, body) = post_json(
        &e,
        &format!("/v1/verdicts/{vd}/review"),
        json!({"reviewer": "vinh", "agree": false}),
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT, "{body}");

    let (status, undone) = post_json(
        &e,
        &format!("/v1/dreams/{}/undo", dream_id(&d)),
        json!({"by": "vinh"}),
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
        json!({"reviewer": "vinh", "agree": false}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["verdict"]["state"], "rejected", "{body}");
    assert_eq!(pack(&e, &reader, "p1").await, before);
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
    // a hidden record is not handed to the next run, and cannot be a canonical record
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
        json!({"by": "vinh"}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["verdict"]["state"], "retracted", "{body}");
    assert_eq!(pack(&e, &reader, "p2").await, before);
    let (status, body) = post_json(
        &e,
        &format!("/v1/verdicts/{vd}/retract"),
        json!({"by": "vinh"}),
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
    // nothing proposed: nothing to verify
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
