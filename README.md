# speckle

Connects to a redis database to check if the `Bearer` token in the `Authorization` header exists.

If the token does not exist an exception is logged.

## Getting Started

1. Start redis:

```bash
$ docker-compose up redis-db
```

2. Run the application:

```bash
$ cargo run
```
