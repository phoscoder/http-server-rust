[![progress-banner](https://backend.codecrafters.io/progress/http-server/e523cf6d-d65e-4012-94d9-1fb436404389)](https://app.codecrafters.io/users/phoscoder?r=2qF)

This project is a Rust implementation of an HTTP/1.1 server, built as part of the ["Build Your Own HTTP server" Challenge](https://app.codecrafters.io/courses/http-server/overview) by CodeCrafters.

[HTTP](https://en.wikipedia.org/wiki/Hypertext_Transfer_Protocol) is the
protocol that powers the web. This server is capable of handling multiple clients concurrently and understanding basic HTTP requests.

**Note**: If you're viewing this repo on GitHub, head over to
[codecrafters.io](https://codecrafters.io) to try the challenge.

## 🚀 Features

This HTTP server supports the following features:
*   **TCP Server**: Establishes a TCP listener to accept client connections.
*   **Concurrent Client Handling** 🧑‍🤝‍🧑: Uses Tokio's asynchronous capabilities to manage multiple client connections simultaneously.
*   **HTTP Request Parsing**: Parses incoming HTTP/1.1 requests, including headers and body.
*   **Route Handling**:
    *   `/`: Responds with a 200 OK.
    *   `/echo/{str}` 🌬️: Responds with the `{str}` part of the path. Supports `gzip` compression if requested by the `Accept-Encoding` header.
    *   `/user-agent` 🕵️: Responds with the value of the `User-Agent` header from the request.
    *   `/files/{filename}`:
        *   `GET` 📄: Serves the content of `{filename}` from a specified directory.
        *   `POST` 💾: Saves the request body to `{filename}` in a specified directory.
*   **Response Generation**: Constructs appropriate HTTP responses, including status codes, headers (like `Content-Type`, `Content-Length`, `Content-Encoding`), and body.
*   **File System Interaction**: Reads files from and writes files to the local file system.
*   **Keep-Alive Connections** 🔄: Supports persistent connections based on the `Connection` header.
*   **Configurable Directory**: Allows specifying the directory to serve files from via a command-line argument.
*   **Timeout Handling** ⏳: Implements a read timeout for client connections to prevent server hangs.

## 🛠️ Usage

1.  Ensure you have `cargo` (Rust's package manager and build tool) installed.
2.  Clone the repository.
3.  Run the server using:
    ```sh
    cargo run [-- --directory /path/to/your/files]
    ```
    Replace `/path/to/your/files` with the actual directory you want to serve files from. If no directory is specified, it defaults to the current directory.
4.  The server will start on `127.0.0.1:4221` by default.