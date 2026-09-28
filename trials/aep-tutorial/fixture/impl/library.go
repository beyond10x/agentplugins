// Package library is a minimal in-memory lending library: books are added and withdrawn, members
// are registered, and a member borrows a book and returns it.
package library

import (
	"crypto/rand"
	"errors"
	"fmt"
)

// BookState is where a book is in its lifecycle.
type BookState string

const (
	OnShelf   BookState = "OnShelf"
	OnLoan    BookState = "OnLoan"
	Withdrawn BookState = "Withdrawn"
)

// Book is one physical book. Borrower is set only while the book is OnLoan.
type Book struct {
	ID       string
	Title    string
	Author   string
	State    BookState
	Borrower *string
}

// Member is a registered member.
type Member struct {
	ID   string
	Name string
}

// ErrBookNotFound is returned when no book has the given identity.
var ErrBookNotFound = errors.New("book not found")

// ErrUnknownMember is returned when a borrow names a member nobody registered.
var ErrUnknownMember = errors.New("member not registered")

// StateConflictError is returned when a book is not in a state the operation acts from.
type StateConflictError struct {
	State BookState
}

func (e *StateConflictError) Error() string {
	return fmt.Sprintf("book is %s", e.State)
}

// Library holds the collection and the members in memory. It is not safe for concurrent use.
type Library struct {
	books   map[string]*Book
	members map[string]*Member
	// order keeps listings stable in insertion order.
	bookOrder   []string
	memberOrder []string
}

// New returns an empty library.
func New() *Library {
	return &Library{books: map[string]*Book{}, members: map[string]*Member{}}
}

// AddBook puts a new book on the shelf.
func (l *Library) AddBook(title, author string) Book {
	b := &Book{ID: newID(), Title: title, Author: author, State: OnShelf}
	l.books[b.ID] = b
	l.bookOrder = append(l.bookOrder, b.ID)
	return *b
}

// RegisterMember registers a new member.
func (l *Library) RegisterMember(name string) Member {
	m := &Member{ID: newID(), Name: name}
	l.members[m.ID] = m
	l.memberOrder = append(l.memberOrder, m.ID)
	return *m
}

// Borrow lends a book on the shelf to a member.
//
// requireMember makes an unregistered member a refusal. The specification leaves that check to the
// implementation and its conformance suite borrows for members it never registered, so the
// conformance target turns it off.
func (l *Library) Borrow(bookID, memberID string, requireMember bool) error {
	b, ok := l.books[bookID]
	if !ok {
		return ErrBookNotFound
	}
	if b.State != OnShelf {
		return &StateConflictError{State: b.State}
	}
	if requireMember {
		if _, ok := l.members[memberID]; !ok {
			return ErrUnknownMember
		}
	}
	b.State = OnLoan
	b.Borrower = &memberID
	return nil
}

// Return puts a book on loan back on the shelf.
func (l *Library) Return(bookID string) error {
	b, ok := l.books[bookID]
	if !ok {
		return ErrBookNotFound
	}
	if b.State != OnLoan {
		return &StateConflictError{State: b.State}
	}
	b.State = OnShelf
	b.Borrower = nil
	return nil
}

// Withdraw takes a book on the shelf out of the collection for good.
func (l *Library) Withdraw(bookID string) error {
	b, ok := l.books[bookID]
	if !ok {
		return ErrBookNotFound
	}
	if b.State != OnShelf {
		return &StateConflictError{State: b.State}
	}
	b.State = Withdrawn
	return nil
}

// Books lists every book, in every state.
func (l *Library) Books() []Book {
	out := make([]Book, 0, len(l.bookOrder))
	for _, id := range l.bookOrder {
		out = append(out, *l.books[id])
	}
	return out
}

// Members lists every member.
func (l *Library) Members() []Member {
	out := make([]Member, 0, len(l.memberOrder))
	for _, id := range l.memberOrder {
		out = append(out, *l.members[id])
	}
	return out
}

func newID() string {
	var b [16]byte
	if _, err := rand.Read(b[:]); err != nil {
		panic(err)
	}
	b[6] = b[6]&0x0f | 0x40
	b[8] = b[8]&0x3f | 0x80
	return fmt.Sprintf("%x-%x-%x-%x-%x", b[0:4], b[4:6], b[6:8], b[8:10], b[10:16])
}
