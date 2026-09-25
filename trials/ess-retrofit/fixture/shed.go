package main

import (
	"crypto/rand"
	"encoding/hex"
	"errors"
	"fmt"
	"sync"
	"time"
)

var (
	ErrNotFound  = errors.New("no such tool")
	ErrOnLoan    = errors.New("tool is on loan")
	ErrNotOnLoan = errors.New("tool is not on loan")
	ErrRetired   = errors.New("tool is retired")
)

// Status is where a tool is: on the shelf, with a member, or gone for good.
type Status string

const (
	Available Status = "available"
	OnLoan    Status = "on_loan"
	Retired   Status = "retired"
)

// Categories the shed shelves tools under; anything else is refused.
var categories = map[string]bool{"power": true, "hand": true, "garden": true, "ladder": true}

type Tool struct {
	ID       string     `json:"id"`
	Name     string     `json:"name"`
	Category string     `json:"category"`
	Status   Status     `json:"status"`
	Member   string     `json:"member,omitempty"`
	DueAt    *time.Time `json:"due_at,omitempty"`
	Loans    int        `json:"loans"`
}

// Event is what the shed announces; the notice board and the reminder mailer read them.
type Event struct {
	Kind   string    `json:"kind"`
	ToolID string    `json:"tool_id"`
	Member string    `json:"member,omitempty"`
	At     time.Time `json:"at"`
}

type Shed struct {
	mu     sync.Mutex
	now    func() time.Time
	tools  map[string]*Tool
	Events []Event
}

func NewShed(now func() time.Time) *Shed {
	return &Shed{now: now, tools: map[string]*Tool{}}
}

func newID() string {
	b := make([]byte, 8)
	_, _ = rand.Read(b)
	return hex.EncodeToString(b)
}

func (s *Shed) emit(kind string, tool *Tool) {
	s.Events = append(s.Events, Event{Kind: kind, ToolID: tool.ID, Member: tool.Member, At: s.now()})
}

func (s *Shed) Add(name, category string) (Tool, error) {
	s.mu.Lock()
	defer s.mu.Unlock()
	if name == "" {
		return Tool{}, invalid("a tool needs a name")
	}
	if !categories[category] {
		return Tool{}, invalid(fmt.Sprintf("unknown category %q", category))
	}
	tool := &Tool{ID: newID(), Name: name, Category: category, Status: Available}
	s.tools[tool.ID] = tool
	s.emit("ToolAdded", tool)
	return *tool, nil
}

func (s *Shed) Lend(id, member string, days int) (Tool, error) {
	s.mu.Lock()
	defer s.mu.Unlock()
	tool, ok := s.tools[id]
	if !ok {
		return Tool{}, ErrNotFound
	}
	if days < 1 || days > 28 {
		return Tool{}, invalid(fmt.Sprintf("a loan is 1 to 28 days, not %d", days))
	}
	switch tool.Status {
	case OnLoan:
		return Tool{}, ErrOnLoan
	case Retired:
		return Tool{}, ErrRetired
	}
	due := s.now().Add(time.Duration(days) * 24 * time.Hour)
	tool.Status, tool.Member, tool.DueAt = OnLoan, member, &due
	tool.Loans++
	s.emit("ToolLent", tool)
	return *tool, nil
}

func (s *Shed) Return(id string) (Tool, error) {
	s.mu.Lock()
	defer s.mu.Unlock()
	tool, ok := s.tools[id]
	if !ok {
		return Tool{}, ErrNotFound
	}
	if tool.Status != OnLoan {
		return Tool{}, ErrNotOnLoan
	}
	s.emit("ToolReturned", tool)
	tool.Status, tool.Member, tool.DueAt = Available, "", nil
	return *tool, nil
}

// Retire takes a tool out of circulation. A tool on loan is retired when it comes back: the call
// succeeds and changes nothing, and the desk is expected to try again later.
func (s *Shed) Retire(id string) (Tool, error) {
	s.mu.Lock()
	defer s.mu.Unlock()
	tool, ok := s.tools[id]
	if !ok {
		return Tool{}, ErrNotFound
	}
	switch tool.Status {
	case Retired:
		return Tool{}, ErrRetired
	case OnLoan:
		return *tool, nil
	}
	tool.Status = Retired
	s.emit("ToolRetired", tool)
	return *tool, nil
}

func (s *Shed) Get(id string) (Tool, error) {
	s.mu.Lock()
	defer s.mu.Unlock()
	tool, ok := s.tools[id]
	if !ok {
		return Tool{}, ErrNotFound
	}
	return *tool, nil
}

// Overdue lists the tools whose loan ran past its due date, oldest first by insertion.
func (s *Shed) Overdue() []Tool {
	s.mu.Lock()
	defer s.mu.Unlock()
	now := s.now()
	var late []Tool
	for _, tool := range s.tools {
		if tool.Status == OnLoan && tool.DueAt != nil && now.After(*tool.DueAt) {
			late = append(late, *tool)
		}
	}
	return late
}
