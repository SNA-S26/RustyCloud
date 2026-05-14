## RustyCloud

### Usage
- Install [Mongo](https://www.mongodb.com/docs/manual/installation/), [Redis](https://redis.io/docs/latest/operate/oss_and_stack/install/archive/install-redis/), and [Rust](https://doc.rust-lang.org/book/ch01-01-installation.html).
- Run the application with `cargo`:

    ```bash
    MONGODB_URI=<mongo uri> REDIS_URI=<redis uri> NFS_MOUNT_POINT=<dir> cargo run -- <port>
    ```

### Running in Docker
- Install [Mongo](https://www.mongodb.com/docs/manual/installation/) and [Redis](https://redis.io/docs/latest/operate/oss_and_stack/install/archive/install-redis/), and [Docker](https://docs.docker.com/engine/install/).
- Build Docker image:

    ```bash
    docker build --tag rustycloud .
    ```
- Run the container:

    ```bash
    docker run --name rustycloud \
    -e MONGODB_URI=<mongo uri> \
    -e REDIS_URI=<redis uri> \
    -e NFS_MOUNT_POINT=<dir> \
    -p <port>:80 -d rustycloud
    ```