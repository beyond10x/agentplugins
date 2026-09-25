package main

import (
	"encoding/json"
	"errors"
	"log"
	"net/http"
	"strings"
	"time"
)

func main() {
	shed := NewShed(time.Now)
	log.Fatal(http.ListenAndServe(":8080", routes(shed)))
}

func routes(shed *Shed) http.Handler {
	mux := http.NewServeMux()
	mux.HandleFunc("POST /tools", func(w http.ResponseWriter, r *http.Request) {
		var body struct {
			Name     string `json:"name"`
			Category string `json:"category"`
		}
		if err := json.NewDecoder(r.Body).Decode(&body); err != nil {
			http.Error(w, err.Error(), http.StatusBadRequest)
			return
		}
		tool, err := shed.Add(body.Name, body.Category)
		reply(w, tool, err)
	})
	mux.HandleFunc("POST /tools/{id}/lend", func(w http.ResponseWriter, r *http.Request) {
		var body struct {
			Member string `json:"member"`
			Days   int    `json:"days"`
		}
		if err := json.NewDecoder(r.Body).Decode(&body); err != nil {
			http.Error(w, err.Error(), http.StatusBadRequest)
			return
		}
		tool, err := shed.Lend(r.PathValue("id"), body.Member, body.Days)
		reply(w, tool, err)
	})
	mux.HandleFunc("POST /tools/{id}/return", func(w http.ResponseWriter, r *http.Request) {
		tool, err := shed.Return(r.PathValue("id"))
		reply(w, tool, err)
	})
	mux.HandleFunc("POST /tools/{id}/retire", func(w http.ResponseWriter, r *http.Request) {
		tool, err := shed.Retire(r.PathValue("id"))
		reply(w, tool, err)
	})
	mux.HandleFunc("GET /tools/{id}", func(w http.ResponseWriter, r *http.Request) {
		tool, err := shed.Get(r.PathValue("id"))
		reply(w, tool, err)
	})
	mux.HandleFunc("GET /loans/overdue", func(w http.ResponseWriter, r *http.Request) {
		reply(w, shed.Overdue(), nil)
	})
	return mux
}

func reply(w http.ResponseWriter, value any, err error) {
	switch {
	case errors.Is(err, ErrNotFound):
		http.Error(w, err.Error(), http.StatusNotFound)
	case errors.Is(err, ErrOnLoan), errors.Is(err, ErrNotOnLoan), errors.Is(err, ErrRetired):
		http.Error(w, err.Error(), http.StatusConflict)
	case err != nil:
		http.Error(w, err.Error(), http.StatusUnprocessableEntity)
	default:
		w.Header().Set("Content-Type", "application/json")
		_ = json.NewEncoder(w).Encode(value)
	}
}

// Validation errors carry the offending value in their message only.
func invalid(message string) error { return errors.New(strings.TrimSpace(message)) }
