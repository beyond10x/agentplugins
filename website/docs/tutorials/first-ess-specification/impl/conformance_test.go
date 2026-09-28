package library_test

import (
	"errors"
	"fmt"
	"os"
	"testing"

	"example.com/library"
	"example.com/library/essconform"
)

func TestConformance(t *testing.T) {
	// The suite runs only once a report format is chosen; choose it here so plain `go test ./...`
	// runs it. ESS_REPORT_OUT, when set, still decides where the report goes.
	if os.Getenv("ESS_REPORT_FORMAT") == "" {
		t.Setenv("ESS_REPORT_FORMAT", "2")
	}
	essconform.Run(t, func() essconform.Target { return newTarget() })
}

// target drives one in-memory Library through the suite's Target interface.
type target struct {
	lib *library.Library
}

func newTarget() *target { return &target{lib: library.New()} }

func (t *target) Identity() (essconform.Identity, error) {
	return essconform.Identity{Name: "example.com/library", Version: "v1"}, nil
}

func (t *target) BeginScenario(essconform.ScenarioContext) error { return nil }
func (t *target) EndScenario(essconform.ScenarioContext) error   { return nil }

func (t *target) ExecuteCommand(req essconform.CommandRequest) (essconform.CommandResult, error) {
	in := func(name string) (string, error) {
		v, ok := req.Input[name].(string)
		if !ok {
			return "", fmt.Errorf("%s: input %q is not text", req.Command, name)
		}
		return v, nil
	}
	event := func(name string, payload map[string]essconform.Node) []essconform.ObservedEvent {
		return []essconform.ObservedEvent{{Event: name, Payload: payload}}
	}

	switch req.Command {
	case "library.lending.AddBook":
		title, err := in("title")
		if err != nil {
			return essconform.CommandResult{}, err
		}
		author, err := in("author")
		if err != nil {
			return essconform.CommandResult{}, err
		}
		b := t.lib.AddBook(title, author)
		return essconform.CommandResult{
			Outcome: "added",
			DirectEvents: event("library.lending.BookAdded", map[string]essconform.Node{
				"book_id": b.ID, "title": b.Title, "author": b.Author,
			}),
		}, nil

	case "library.lending.RegisterMember":
		name, err := in("name")
		if err != nil {
			return essconform.CommandResult{}, err
		}
		m := t.lib.RegisterMember(name)
		return essconform.CommandResult{
			Outcome: "registered",
			DirectEvents: event("library.lending.MemberRegistered", map[string]essconform.Node{
				"member_id": m.ID, "name": m.Name,
			}),
		}, nil

	case "library.lending.BorrowBook":
		bookID, err := in("book_id")
		if err != nil {
			return essconform.CommandResult{}, err
		}
		memberID, err := in("member_id")
		if err != nil {
			return essconform.CommandResult{}, err
		}
		// The member check is the implementation's own rule, outside the specification; the
		// suite borrows for members it never registered, so it is off here.
		if err := t.lib.Borrow(bookID, memberID, false); err != nil {
			return refusal(err)
		}
		return essconform.CommandResult{
			Outcome: "borrowed",
			DirectEvents: event("library.lending.BookBorrowed", map[string]essconform.Node{
				"book_id": bookID, "member_id": memberID,
			}),
		}, nil

	case "library.lending.ReturnBook":
		bookID, err := in("book_id")
		if err != nil {
			return essconform.CommandResult{}, err
		}
		if err := t.lib.Return(bookID); err != nil {
			return refusal(err)
		}
		return essconform.CommandResult{
			Outcome:      "returned",
			DirectEvents: event("library.lending.BookReturned", map[string]essconform.Node{"book_id": bookID}),
		}, nil

	case "library.lending.WithdrawBook":
		bookID, err := in("book_id")
		if err != nil {
			return essconform.CommandResult{}, err
		}
		if err := t.lib.Withdraw(bookID); err != nil {
			return refusal(err)
		}
		return essconform.CommandResult{
			Outcome:      "withdrawn",
			DirectEvents: event("library.lending.BookWithdrawn", map[string]essconform.Node{"book_id": bookID}),
		}, nil
	}
	return essconform.CommandResult{}, fmt.Errorf("unknown command %q: %w", req.Command, essconform.ErrUnsupported)
}

// refusal maps a library error onto the declared outcome and error.
func refusal(err error) (essconform.CommandResult, error) {
	var conflict *library.StateConflictError
	switch {
	case errors.As(err, &conflict):
		return essconform.CommandResult{Outcome: "wrong-state", Error: "library.lending.BookStateConflict"}, nil
	case errors.Is(err, library.ErrBookNotFound):
		return essconform.CommandResult{Outcome: "no-such-book", Error: "library.lending.BookNotFound"}, nil
	}
	return essconform.CommandResult{}, err
}

func (t *target) QueryView(req essconform.ViewRequest) (essconform.ViewResult, error) {
	var rows []essconform.Row
	switch req.View {
	case "library.lending.Catalogue":
		for _, b := range t.lib.Books() {
			rows = append(rows, essconform.Row{
				"book_id": b.ID, "title": b.Title, "author": b.Author,
				"state": string(b.State), "borrower_id": borrower(b),
			})
		}
	case "library.lending.BooksOnLoan":
		for _, b := range t.lib.Books() {
			if b.State == library.OnLoan {
				rows = append(rows, essconform.Row{
					"book_id": b.ID, "title": b.Title, "borrower_id": borrower(b),
				})
			}
		}
	case "library.lending.Members":
		for _, m := range t.lib.Members() {
			rows = append(rows, essconform.Row{"member_id": m.ID, "name": m.Name})
		}
	default:
		return essconform.ViewResult{}, fmt.Errorf("unknown view %q: %w", req.View, essconform.ErrUnsupported)
	}
	return essconform.ViewResult{Rows: rows}, nil
}

func borrower(b library.Book) essconform.Node {
	if b.Borrower == nil {
		return nil
	}
	return *b.Borrower
}

// Every event is returned directly by the command that emits it; there is nothing to observe apart.
func (t *target) ObserveEvents(essconform.EventObservationRequest) ([]essconform.ObservedEvent, error) {
	return nil, essconform.ErrUnsupported
}

func (t *target) ConfigureExternalOutcome(essconform.ExternalOutcomeControl) error {
	return essconform.ErrUnsupported
}

func (t *target) RedeliverEvent(essconform.RedeliveryRequest) error {
	return essconform.ErrUnsupported
}

func (t *target) ObserveInvocations(essconform.InvocationObservationRequest) ([]essconform.Invocation, error) {
	return nil, essconform.ErrUnsupported
}
