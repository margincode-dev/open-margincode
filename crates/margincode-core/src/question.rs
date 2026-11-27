//! A question an agent asks a person (`ask`), what the daemon tells the agent to do meanwhile, and
//! the decision that answers it.

use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::error::{AvError, Result};
use crate::types::{MAX_SUMMARY_CHARS, str_enum};

str_enum!(AskLevel {
    AskFirst => "ask_first",
    Balanced => "balanced",
    CarryOn => "carry_on",
});

str_enum!(Undo {
    Workspace => "workspace",
    Outside => "outside",
});

str_enum!(Instruction {
    Proceed => "proceed",
    Park => "park",
});

/// More open questions than this on one task and `ask` is refused.
pub const MAX_OPEN_QUESTIONS: usize = 3;
pub const MAX_OPTION_CHARS: usize = 60;
pub const MAX_WHY_CHARS: usize = 200;
pub const MAX_DEPENDS_CHARS: usize = 400;
pub const MAX_STATE_CHARS: usize = 2000;
pub const MAX_COMMENT_CHARS: usize = 2000;

impl AskLevel {
    /// `s` as a level, or a message naming the three.
    pub fn named(s: &str) -> Result<AskLevel> {
        s.trim().parse().map_err(|_| {
            AvError::Invalid(format!(
                "an ask level is ask_first, balanced or carry_on, not {s:?}"
            ))
        })
    }

    /// What an agent at this level does while the question waits, given what a wrong guess
    /// would touch.
    pub fn instruction(self, undo: Undo) -> Instruction {
        match (self, undo) {
            (AskLevel::AskFirst, _) => Instruction::Park,
            (AskLevel::Balanced, Undo::Workspace) => Instruction::Proceed,
            (AskLevel::Balanced, Undo::Outside) => Instruction::Park,
            (AskLevel::CarryOn, _) => Instruction::Proceed,
        }
    }
}

impl Undo {
    pub fn named(s: &str) -> Result<Undo> {
        s.trim().parse().map_err(|_| {
            AvError::Invalid(format!(
                "undo is workspace (a wrong option costs only redoing work in this task) or \
                 outside (it changes something outside the task's workspace), not {s:?}"
            ))
        })
    }
}

/// The one line the asking agent is told.
pub fn told(instruction: Instruction, recommended: &str) -> String {
    match instruction {
        Instruction::Proceed => {
            format!("Carry on with {recommended}. Record what you build on it.")
        }
        Instruction::Park => {
            "Finish what does not depend on this, checkpoint, then stop this task.".to_owned()
        }
    }
}

/// What is done and what remains when the question is asked, as a handoff says it.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuestionState {
    #[serde(default)]
    pub done: String,
    #[serde(default)]
    pub remaining: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Question {
    pub question: String,
    pub options: Vec<String>,
    /// One of `options`, the one the agent would take.
    pub recommended: String,
    pub why: String,
    /// What work rests on the answer.
    pub depends: String,
    pub undo: Undo,
    #[serde(default)]
    pub state: QuestionState,
}

impl Question {
    /// Trimmed and checked; `recommended` takes its option's spelling.
    pub fn checked(self) -> Result<Question> {
        let question = text("the question", &self.question, MAX_SUMMARY_CHARS)?;
        let options: Vec<String> = self
            .options
            .iter()
            .map(|o| text("an option", o, MAX_OPTION_CHARS))
            .collect::<Result<_>>()?;
        // an answer's body starts `Chosen: <option>` and `chosen` reads that first line back
        if let Some(o) = options.iter().find(|o| o.contains(['\n', '\r'])) {
            return Err(AvError::Invalid(format!("an option is one line: {o:?}")));
        }
        if !(2..=4).contains(&options.len()) {
            return Err(AvError::Invalid(format!(
                "{} options given; give two to four",
                options.len()
            )));
        }
        for (i, o) in options.iter().enumerate() {
            if options[..i].iter().any(|p| p.eq_ignore_ascii_case(o)) {
                return Err(AvError::Invalid(format!("option {o:?} is given twice")));
            }
        }
        let checked = Question {
            question,
            options,
            recommended: String::new(),
            why: text("why", &self.why, MAX_WHY_CHARS)?,
            depends: text("depends", &self.depends, MAX_DEPENDS_CHARS)?,
            undo: self.undo,
            state: QuestionState {
                done: limited("state.done", &self.state.done, MAX_STATE_CHARS)?,
                remaining: limited("state.remaining", &self.state.remaining, MAX_STATE_CHARS)?,
            },
        };
        if checked.why.contains('\n') {
            return Err(AvError::Invalid("why is one line".into()));
        }
        let recommended = checked
            .option(&self.recommended)
            .ok_or_else(|| {
                AvError::Invalid(format!(
                    "recommended must be one of the options ({}) or its number, not {:?}",
                    checked.options.join(", "),
                    self.recommended.trim()
                ))
            })?
            .to_owned();
        Ok(Question {
            recommended,
            ..checked
        })
    }

    /// The option `given` names: its text, in any case, else its number counted from 1 (so of
    /// the options `2` and `5`, `2` is the first).
    pub fn option(&self, given: &str) -> Option<&str> {
        let given = given.trim();
        let by_number = || {
            given
                .parse::<usize>()
                .ok()
                .and_then(|n| n.checked_sub(1))
                .and_then(|i| self.options.get(i))
        };
        self.options
            .iter()
            .find(|o| o.eq_ignore_ascii_case(given))
            .or_else(by_number)
            .map(String::as_str)
    }
}

/// Whether `a` and `b` name one agent: one name in any case, or names (any case) of one profile,
/// its name or an alias.
pub fn same_agent(profiles: &[crate::routing::Profile], a: &str, b: &str) -> bool {
    let canonical = |n: &str| {
        let n = n.trim();
        profiles
            .iter()
            .find(|p| {
                p.name.eq_ignore_ascii_case(n)
                    || p.aliases.iter().any(|x| x.eq_ignore_ascii_case(n))
            })
            .map_or(n, |p| p.name.as_str())
            .to_lowercase()
    };
    canonical(a) == canonical(b)
}

/// The body of a `question` record: what was asked, and what the daemon told the agent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Asked {
    #[serde(flatten)]
    pub question: Question,
    pub level: AskLevel,
    pub instruction: Instruction,
}

impl Asked {
    pub fn told(&self) -> String {
        told(self.instruction, &self.question.recommended)
    }
}

/// Summary and body of the decision that answers `q` with `option` (one of its options).
pub fn answer_record(q: &Question, option: &str, comment: &str) -> (String, String) {
    let arrow = format!(" → {option}");
    let room = MAX_SUMMARY_CHARS - arrow.chars().count();
    let asked = if q.question.chars().count() <= room {
        q.question.clone()
    } else {
        let cut: String = q.question.chars().take(room - 1).collect();
        format!("{}…", cut.trim_end())
    };
    let summary = format!("{asked}{arrow}");
    let matched = if option == q.recommended {
        "It is the recommended option.".to_owned()
    } else {
        format!("It is not the recommended option ({}).", q.recommended)
    };
    let mut body = format!("Chosen: {option}\n{matched}\nAsked: {}", q.question);
    if !comment.trim().is_empty() {
        body.push_str(&format!("\nComment: {}", comment.trim()));
    }
    (summary, body)
}

/// The option an answer's body names (`answer_record`'s first line).
pub fn chosen(answer_body: &str) -> Option<&str> {
    answer_body.lines().next()?.strip_prefix("Chosen: ")
}

fn text(what: &str, v: &str, max: usize) -> Result<String> {
    let v = v.trim();
    if v.is_empty() {
        return Err(AvError::Invalid(format!("{what} must not be empty")));
    }
    limited(what, v, max)
}

fn limited(what: &str, v: &str, max: usize) -> Result<String> {
    let n = v.trim().chars().count();
    if n > max {
        return Err(AvError::Invalid(format!(
            "{what} is {n} characters; the limit is {max}"
        )));
    }
    Ok(v.trim().to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn q() -> Question {
        Question {
            question: " Which database? ".into(),
            options: vec!["Postgres".into(), " SQLite ".into()],
            recommended: "postgres".into(),
            why: "one already runs".into(),
            depends: "the schema".into(),
            undo: Undo::Workspace,
            state: QuestionState::default(),
        }
    }

    #[test]
    fn the_level_and_undo_give_the_instruction() {
        use AskLevel::*;
        use Instruction::*;
        let table = [
            (AskFirst, Undo::Workspace, Park),
            (AskFirst, Undo::Outside, Park),
            (Balanced, Undo::Workspace, Proceed),
            (Balanced, Undo::Outside, Park),
            (CarryOn, Undo::Workspace, Proceed),
            (CarryOn, Undo::Outside, Proceed),
        ];
        for (level, undo, want) in table {
            assert_eq!(level.instruction(undo), want, "{level:?} {undo:?}");
        }
    }

    #[test]
    fn a_checked_question_is_trimmed_and_recommends_an_options_spelling() {
        let c = q().checked().unwrap();
        assert_eq!(c.question, "Which database?");
        assert_eq!(c.options, ["Postgres", "SQLite"]);
        assert_eq!(c.recommended, "Postgres");
        assert_eq!(c.option("2"), Some("SQLite"));
        assert_eq!(c.option("sqlite"), Some("SQLite"));
        assert_eq!(c.option("0"), None);
        assert_eq!(c.option("3"), None);
    }

    #[test]
    fn an_options_text_wins_over_its_number() {
        let mut retries = q();
        retries.options = vec!["2".into(), "5".into()];
        retries.recommended = "2".into();
        assert_eq!(retries.clone().checked().unwrap().recommended, "2");
        retries.recommended = "1".into();
        let c = retries.clone().checked().unwrap();
        assert_eq!(c.recommended, "2");
        assert_eq!(c.option("2"), Some("2"));
        assert_eq!(c.option("5"), Some("5"));
        assert_eq!(c.option("1"), Some("2"));
        assert_eq!(c.option("3"), None);
        retries.recommended = "3".into();
        assert!(retries.checked().is_err());
    }

    #[test]
    fn an_option_is_one_line() {
        for broken in ["SQLite\nwith WAL", "SQLite\rin memory"] {
            let mut two = q();
            two.options = vec!["Postgres".into(), broken.into()];
            let e = two.checked().unwrap_err();
            assert!(matches!(e, AvError::Invalid(_)), "{e:?}");
            assert!(e.to_string().contains("one line"), "{e}");
        }
    }

    #[test]
    fn one_agent_is_one_agent_under_any_case_or_alias() {
        let careful = crate::routing::Profile {
            name: "careful".into(),
            best: vec![],
            can: vec![],
            cost_rank: 1,
            aliases: vec!["Claude".into()],
            max_open: None,
            managed_by: None,
        };
        let profiles = [careful];
        assert!(same_agent(&profiles, "claude", "Claude"));
        assert!(same_agent(&profiles, "claude", "CAREFUL"));
        assert!(same_agent(&profiles, "grok", " Grok "));
        assert!(!same_agent(&profiles, "claude", "alice"));
        assert!(!same_agent(&[], "claude", "careful"));
    }

    #[test]
    fn levels_and_undo_are_named_with_their_choices() {
        assert_eq!(AskLevel::named(" carry_on").unwrap(), AskLevel::CarryOn);
        let e = AskLevel::named("often").unwrap_err().to_string();
        assert!(e.contains("ask_first, balanced or carry_on"), "{e}");
        let e = Undo::named("disk").unwrap_err().to_string();
        assert!(e.contains("workspace") && e.contains("outside"), "{e}");
    }

    #[test]
    fn an_answer_keeps_the_option_in_its_summary_however_long_the_question() {
        let mut long = q().checked().unwrap();
        long.question = "x".repeat(MAX_SUMMARY_CHARS);
        let (summary, body) = answer_record(&long, "SQLite", " on one disk ");
        assert_eq!(summary.chars().count(), MAX_SUMMARY_CHARS);
        assert!(summary.ends_with("… → SQLite"), "{summary}");
        assert_eq!(chosen(&body), Some("SQLite"));
        assert!(body.contains("not the recommended option (Postgres)"));
        assert!(body.ends_with("Comment: on one disk"), "{body}");
        let (_, body) = answer_record(&long, "Postgres", "");
        assert!(body.contains("It is the recommended option."), "{body}");
        assert!(!body.contains("Comment"), "{body}");
    }
}
