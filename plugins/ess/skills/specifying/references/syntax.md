# ESS syntax by example

A small todo service in three files, with every section a specification usually needs. It validates
as written (`ess specify validate --path <directory>` → `todo v1 — 3 file(s), valid`). Copy the shape,
not the domain.

## `system.yaml`

```yaml
format: ess/1
system: todo
version: v1

domains:
  - todo.list
```

## `components.yaml`

Who runs the domain, which commands it accepts, which events it publishes, how it is reached.

```yaml
components:
  - component: todo-service
    summary: Holds every todo list and its tasks.
    owns:
      domains:
        - todo.list
    accepts:
      commands:
        - todo.list.CreateList
        - todo.list.AddTask
        - todo.list.CompleteTask
    publishes:
      events:
        - todo.list.ListCreated
        - todo.list.TaskAdded
        - todo.list.TaskCompleted
    reached_by: network
```

## `domains/list.yaml`

```yaml
domain: todo.list

summary: Lists of tasks that a person adds and completes.

naming:
  wire: lists
  display: Todo lists

# Types: `newtype` over a primitive, `enum`, `struct` (with `fields`, optional `invariants`),
# `union` (tagged: `tag:` plus `variants:` name → type). Primitives include Uuid, String, Integer,
# Decimal, Boolean, Bytes, Timestamp, Duration; wrappers Optional<T>, List<T>, Map<K, V>.
types:
  - name: todo.list.ListId
    kind: newtype
    of: Uuid

  - name: todo.list.TaskId
    kind: newtype
    of: Uuid

  - name: todo.list.Title
    kind: newtype
    of: String

  - name: todo.list.Priority
    kind: enum
    variants: [Low, Normal, High]

# Entities: an identity, fields, relations to other entities, and a lifecycle whose transitions
# commands move. `owns` means the target cannot outlive this entity; `references` means it can.
entities:
  - name: todo.list.TodoList
    identity:
      name: list_id
      type: todo.list.ListId
    fields:
      - name: title
        type: todo.list.Title
    relations:
      - name: tasks
        kind: owns
        target: todo.list.Task
        cardinality: many
        via: list_id
    lifecycle:
      initial: Active
      states: [Active]
      terminal: [Active]

  - name: todo.list.Task
    identity:
      name: task_id
      type: todo.list.TaskId
    fields:
      - name: list_id
        type: todo.list.ListId
      - name: title
        type: todo.list.Title
      - name: priority
        type: todo.list.Priority
      - name: estimate_minutes
        type: Integer
    invariants:
      - estimate_minutes > 0
    lifecycle:
      initial: Open
      states: [Open, Done]
      terminal: [Done]
      transitions:
        - name: complete
          from: [Open]
          to: Done

# Actors: who may invoke which commands.
actors:
  - name: todo.list.Owner
    may:
      - todo.list.CreateList
      - todo.list.AddTask
      - todo.list.CompleteTask
    naming:
      display: List owner

# Errors a command outcome can return.
errors:
  - name: todo.list.InvalidEstimate
    summary: The estimate is not a positive number of minutes.
    fields:
      - name: submitted
        type: Integer

  - name: todo.list.TaskStateConflict
    summary: The task is not in a state this command acts from, so nothing moved.
    fields:
      - name: state
        type: todo.list.Task.State

# Commands: input, then outcomes. An outcome `creates` an entity or `moves` one through a
# transition, `emits` events with a `payload` built from `input.<field>`, or returns an `error`.
commands:
  - name: todo.list.CreateList
    naming:
      wire: create-list
      display: Create a list
    input:
      - name: title
        type: todo.list.Title
    outcomes:
      - name: created
        creates: todo.list.TodoList
        instance: list_id
        emits:
          - todo.list.ListCreated
        payload:
          todo.list.ListCreated:
            title: input.title
        summary: The list exists and is empty.

  # Two outcomes of one command: all but one need a `when` over the command's input, or the result
  # is not determined by the input and `validate` refuses it as `conflicting_declaration`.
  - name: todo.list.AddTask
    naming:
      wire: add-task
      display: Add a task
    input:
      - name: list_id
        type: todo.list.ListId
      - name: title
        type: todo.list.Title
      - name: priority
        type: todo.list.Priority
      - name: estimate_minutes
        type: Integer
    outcomes:
      - name: added
        when: estimate_minutes > 0
        creates: todo.list.Task
        instance: task_id
        emits:
          - todo.list.TaskAdded
        payload:
          todo.list.TaskAdded:
            list_id: input.list_id
            title: input.title
        summary: The task is on the list and Open.

      - name: refused
        error: todo.list.InvalidEstimate
        summary: The estimate was not positive, and nothing was added.

  # A command that moves an entity: `wrong_state: true` answers from every state the transition does
  # not start from, so the `from:` list lives in one place.
  - name: todo.list.CompleteTask
    naming:
      wire: complete-task
      display: Complete a task
    input:
      - name: task_id
        type: todo.list.TaskId
    outcomes:
      - name: completed
        moves: todo.list.Task.complete
        instance: task_id
        emits:
          - todo.list.TaskCompleted
        payload:
          todo.list.TaskCompleted:
            task_id: input.task_id
        summary: The task is Done.

      - name: wrong-state
        wrong_state: true
        error: todo.list.TaskStateConflict
        summary: The task is not Open, so nothing was completed.

events:
  - name: todo.list.ListCreated
    fields:
      - name: list_id
        type: todo.list.ListId
      - name: title
        type: todo.list.Title

  - name: todo.list.TaskAdded
    fields:
      - name: task_id
        type: todo.list.TaskId
      - name: list_id
        type: todo.list.ListId
      - name: title
        type: todo.list.Title

  - name: todo.list.TaskCompleted
    fields:
      - name: task_id
        type: todo.list.TaskId

# Views: read models over an entity. `read_your_writes` or `eventual`; an optional `filter`.
views:
  - name: todo.list.OpenTasks
    source: todo.list.Task
    consistency: read_your_writes
    filter: state == Open
    fields:
      - name: task_id
        type: todo.list.TaskId
      - name: title
        type: todo.list.Title
    naming:
      wire: open-tasks
      display: Open tasks
```

## What `when` can and cannot say

A `when` reads only the command's own input. A condition on another entity — "the list must exist",
"the customer is active" — is not an input guard. Express it as a transition of that entity (a command
that `moves` it, answered by `wrong_state` from states it does not start from), or leave an
`UNMAPPED:` marker naming the rule and report it; never invent an outcome the compiler cannot decide.
