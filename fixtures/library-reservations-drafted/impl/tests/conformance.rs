use std::{cell::RefCell, collections::BTreeMap};

use ess_conformance::{
    runner::Runner,
    scenario::{ConformanceSuite, OutcomeRef},
    target::*,
    AdmittedSuite,
};
use ess_primitives::{consistency::ConsistencyToken, node::Node};
use library_tutorial::{Library, Refusal};

#[derive(Default)]
struct Target(RefCell<Library>);

fn text(value: impl Into<String>) -> Node {
    Node::Text(value.into())
}
fn row(values: impl IntoIterator<Item = (&'static str, Node)>) -> ViewRow {
    values
        .into_iter()
        .map(|(key, value)| (key.to_owned(), value))
        .collect()
}
fn qualified(name: &str) -> String {
    format!("library.lending.{name}")
}

impl ConformanceTarget for Target {
    fn identity(&self) -> Result<ImplementationIdentity, TargetError> {
        Ok(ImplementationIdentity::new("library-tutorial", "0.1.0"))
    }
    fn begin_scenario(&self, _: &ScenarioContext) -> Result<(), TargetError> {
        *self.0.borrow_mut() = Library::default();
        Ok(())
    }
    fn end_scenario(&self, _: &ScenarioContext) -> Result<(), TargetError> {
        Ok(())
    }
    fn execute_command(
        &self,
        req: SemanticCommandRequest,
    ) -> Result<SemanticCommandResult, TargetError> {
        let input = |key: &str| match req.input.get(key) {
            Some(Node::Text(value)) => Ok(value.clone()),
            _ => Err(TargetError::unavailable(
                "input",
                format!("{key} must be text"),
            )),
        };
        let mut lib = self.0.borrow_mut();
        let mut payload = BTreeMap::new();
        let (outcome, event, result) = match req.command.to_string().as_str() {
            "library.lending.AddBook" => {
                let title = input("title")?;
                let author = input("author")?;
                let id = lib.add_book(title.clone(), author.clone());
                payload = row([
                    ("book_id", text(id)),
                    ("title", text(title)),
                    ("author", text(author)),
                ]);
                ("added", "BookAdded", Ok(()))
            }
            "library.lending.RegisterMember" => {
                let name = input("name")?;
                let id = lib.register_member(name.clone());
                payload = row([("member_id", text(id)), ("name", text(name))]);
                ("registered", "MemberRegistered", Ok(()))
            }
            "library.lending.BorrowBook" => {
                let book = input("book_id")?;
                let member = input("member_id")?;
                let result = lib.borrow(&book, &member);
                payload = row([("book_id", text(book)), ("member_id", text(member))]);
                ("borrowed", "BookBorrowed", result)
            }
            "library.lending.ReturnBook" => {
                let id = input("book_id")?;
                let result = lib.return_book(&id);
                payload.insert("book_id".into(), text(id));
                ("returned", "BookReturned", result)
            }
            "library.lending.WithdrawBook" => {
                let id = input("book_id")?;
                let result = lib.withdraw(&id);
                payload.insert("book_id".into(), text(id));
                ("withdrawn", "BookWithdrawn", result)
            }
            _ => return Err(TargetError::unsupported("command", req.command.to_string())),
        };
        let result = match result {
            Ok(()) => SemanticCommandResult::took(OutcomeRef::new(
                req.command.clone(),
                outcome.parse().unwrap(),
            ))
            .emitting(ObservedEvent {
                event: qualified(event).parse().unwrap(),
                payload,
                correlation: Some(req.correlation),
                sequence: Some(lib.revision()),
            }),
            Err(error) => {
                let (outcome, name, fields) = match error {
                    Refusal::UnknownBook => (
                        "no-such-book",
                        "BookNotFound",
                        row([("book_id", text(input("book_id")?))]),
                    ),
                    Refusal::WrongState(state) => (
                        "wrong-state",
                        "BookStateConflict",
                        row([("state", text(state))]),
                    ),
                };
                SemanticCommandResult::took(OutcomeRef::new(
                    req.command.clone(),
                    outcome.parse().unwrap(),
                ))
                .with_error(DeclaredErrorValue {
                    error: qualified(name).parse().unwrap(),
                    fields,
                })
            }
        };
        Ok(result.with_consistency(ConsistencyToken::new(lib.revision().to_string()).unwrap()))
    }
    fn query_view(&self, req: SemanticViewRequest) -> Result<SemanticViewResult, TargetError> {
        let lib = self.0.borrow();
        if let Some(token) = req.consistency.token() {
            let revision = token
                .as_str()
                .parse::<u64>()
                .map_err(|_| TargetError::unavailable("consistency", "invalid library token"))?;
            if revision > lib.revision() {
                return Err(TargetError::unavailable(
                    "consistency",
                    "requested revision is not committed",
                ));
            }
        }
        let rows: Vec<ViewRow> = match req.view.to_string().as_str() {
            "library.lending.Members" => lib
                .members
                .iter()
                .map(|(id, name)| row([("member_id", text(id)), ("name", text(name))]))
                .collect(),
            "library.lending.Catalogue" | "library.lending.BooksOnLoan" => {
                let loans = req.view.to_string() == "library.lending.BooksOnLoan";
                lib.books
                    .values()
                    .filter(|book| !loans || book.state == "OnLoan")
                    .map(|book| {
                        let mut value = row([
                            ("book_id", text(&book.id)),
                            ("title", text(&book.title)),
                            (
                                "borrower_id",
                                book.borrower.as_ref().map(text).unwrap_or(Node::Null),
                            ),
                        ]);
                        if !loans {
                            value.insert("author".into(), text(&book.author));
                            value.insert("state".into(), text(book.state));
                        }
                        value
                    })
                    .collect()
            }
            _ => return Err(TargetError::unsupported("view", req.view.to_string())),
        };
        Ok(SemanticViewResult::of(rows))
    }
    fn observe_events(
        &self,
        _: EventObservationRequest,
    ) -> Result<Vec<ObservedEvent>, TargetError> {
        Err(TargetError::unsupported(
            "events",
            "all publications are direct",
        ))
    }
    fn configure_external_outcome(&self, _: ExternalOutcomeControl) -> Result<(), TargetError> {
        Err(TargetError::unsupported(
            "external outcome",
            "no external outcomes",
        ))
    }
    fn redeliver_event(&self, _: RedeliveryRequest) -> Result<(), TargetError> {
        Err(TargetError::unsupported("redelivery", "no bindings"))
    }
    fn observe_invocations(
        &self,
        _: InvocationObservationRequest,
    ) -> Result<Vec<ObservedInvocation>, TargetError> {
        Err(TargetError::unsupported("invocations", "no bindings"))
    }
}

#[test]
fn conforms_to_generated_suite() {
    let suite: ConformanceSuite = serde_json::from_str(include_str!("../suite.json")).unwrap();
    assert!(
        !suite.scenarios.is_empty(),
        "an empty suite is not evidence"
    );
    let admitted = AdmittedSuite::from_suite(&suite).unwrap();
    let report = Runner::for_suite(admitted.suite())
        .run_admitted(&admitted, &Target::default())
        .into_report();
    println!("conformance scenarios: {:?}", report.counts());
    for failure in report.failures() {
        eprintln!("{failure:#?}");
    }
    assert!(
        report.is_conformant(),
        "every scenario must pass, with no skips"
    );
}

#[test]
fn read_refuses_invalid_or_future_consistency_tokens() {
    use ess_primitives::{consistency::QueryConsistency, ids::CorrelationId, time::Timestamp};
    let target = Target::default();
    for token in ["not-a-revision", "1"] {
        let request = SemanticViewRequest {
            view: qualified("Catalogue").parse().unwrap(),
            params: BTreeMap::new(),
            consistency: QueryConsistency::at_least(ConsistencyToken::new(token).unwrap()),
            correlation: CorrelationId::new("freshness-check").unwrap(),
            deadline: Deadline::at(Timestamp::EPOCH),
        };
        assert!(
            target.query_view(request).is_err(),
            "{token} must not become a weaker read"
        );
    }
}
