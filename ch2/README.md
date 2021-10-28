# Chapter 2

A server implementation in Go and client in Rust.

Run the server:

```shell
make server
```

We can try the `grpc_cli` to query the endpoints (using `grpc/reflection`):

```shell
grpc_cli ls localhost:50051 -l
```
