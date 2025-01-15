(1)
podman pull  docker.io/rust:latest
(2)
podman run -it --name rust-dev-env -v $(pwd):/usr/src/playground-rust -w /usr/src/playground-rust rust:latest bash
(3)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
(4)
   cargo new util_ssh_client
   cd util_ssh_client
   cargo build
   cargo run
(5) restart container
podman start -ai rust-dev-env
