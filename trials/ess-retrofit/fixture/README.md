# toolshed

The neighbourhood tool library: members borrow drills, ladders and saws, and bring them back.

```console
go run .            # listens on :8080
```

| method | path | does |
|---|---|---|
| `POST` | `/tools` | add a tool (`{"name": "…", "category": "…"}`) |
| `POST` | `/tools/{id}/lend` | lend it to a member (`{"member": "…", "days": 7}`) |
| `POST` | `/tools/{id}/return` | take it back |
| `POST` | `/tools/{id}/retire` | take it out of circulation |
| `GET` | `/tools/{id}` | one tool |
| `GET` | `/loans/overdue` | tools out longer than agreed |
