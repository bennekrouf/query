
# Query gRPC Service

This service provides an API to query log entries stored in Parquet files using a gRPC-based interface. It uses [DataFusion](https://github.com/apache/arrow-datafusion) to execute SQL queries on Parquet files and serves the results over gRPC.

## Features

- Query log entries based on tenant and a date range.
- Store log entries in Parquet format.
- gRPC-based API for remote querying.
- Dynamically infers schema based on the Parquet file contents.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Protobuf compiler (`protoc`)](https://grpc.io/docs/protoc-installation/)

### Project Setup

1. Clone the repository:

    ```bash
    git clone https://github.com/your-repo/query-grpc.git
    cd query-grpc
    ```

2. Install the necessary Rust dependencies:

    ```bash
    cargo build
    ```

3. Ensure that `protoc` is installed and in your `PATH` to generate gRPC code from `.proto` files.

4. Start the gRPC server:

    ```bash
    cargo run
    ```

By default, the gRPC server will listen on `localhost:50051`.

## Usage

### Service Definition

The `Query` gRPC service exposes the `ReadLogs` method, which queries logs based on tenant name and a date range.

See proto-definitions


### Example gRPC Call

You can use a gRPC client like grpcurl, or generate a Rust client using tonic to interact with the service. Below is an example using grpcurl.
1. grpcurl Installation

grpcurl is a command-line tool that lets you interact with gRPC servers. Install it by following the instructions here.
2. Making a gRPC Request

Once the server is running, you can query logs with grpcurl. Here's an example request:

```bash

grpcurl -d '{
  "tenant": "TenantA",
  "date_from": "2024-08-26T00:00:00Z",
  "date_to": "2024-08-27T00:00:00Z"
}' -plaintext localhost:50051 query.Query/ReadLogs
```
```
```
This command queries the logs for TenantA within the specified date range.

3. Response Format

The server will respond with a list of log entries that match the query. Here's an example response:



{
  "logs": [
    {
      "datetime": "2024-08-26T10:15:42Z",
      "tenant_name": "TenantA",
      "item_id": "Item123",
      "status": "SUCCESS",
      "qty": 100.5
    }
  ]
}


## How It Works

    gRPC Server: The server uses tonic to implement the gRPC service. It listens on a specified port (default: 50051) and exposes a ReadLogs method.

    Parquet Storage: Logs are stored in a Parquet file. The schema is dynamically inferred when reading the file, so it can accommodate various log structures.

    DataFusion Integration: The server uses DataFusion to execute SQL queries on the Parquet file. This allows filtering logs by tenant name and date range.

    Arrow & Parquet: The server leverages Apache Arrow and Parquet for efficient in-memory and disk-based storage, providing high-performance analytics.



 ## Conclusion

This gRPC service provides an efficient way to query logs stored in Parquet files. It leverages DataFusion for SQL execution, and Apache Arrow and Parquet for high-performance data processing. You can interact with this service through any gRPC client, providing flexibility for integration in various environments.
