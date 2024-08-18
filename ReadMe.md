Certainly! Here's a concise version of the `README.md`:

---

#  Payment Microservice in Rust using gRPC

A Rust project implementing a gRPC server and client using `tonic` and `prost` for processing Bitcoin payments.


1. **Build the Project**

   ```bash
   cargo build
   ```

   This will compile the `.proto` files and the Rust code.

## Running the Server

Start the server with:

```bash
cargo run
```

The server listens on `localhost:50051`.

## Running the Client

If a client binary is defined, run it with:

```bash
cargo run --bin payment-client
```

## Example

- **Protobuf Definition:** `proto/payment.proto`
- **Server Implementation:** `src/main.rs`

## Development

- **Update Protobuf:** Modify `.proto` files and run `cargo build`.
- **Testing:** Add tests in the `tests/` directory and run with `cargo test`.

