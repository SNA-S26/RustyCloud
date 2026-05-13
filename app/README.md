## RustyCloud

### Usage
- Install [Rust](https://doc.rust-lang.org/book/ch01-01-installation.html).
- Run the application with `cargo`:

    ```bash
    cargo run -- <port>
    ```

### Running in Docker
- Install [Docker](https://docs.docker.com/engine/install/).
- Build Docker image:

    ```bash
    docker build --tag rustycloud .
    ```
- Run the container:

    ```bash
    docker run --name rustycloud -p <port>:80 -d rustycloud
    ```