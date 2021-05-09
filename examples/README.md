
# How to cross compile
[cross](https://github.com/rust-embedded/cross) is a great tool to do this job.

On Debian based system, follow these steps:
1. Install docker and podman: `sudo apt install docker podman`
2. Add current user to docker group: `sudo addgroup ${USER} docker; newgrp docker`
3. Install cross: `cargo install cross`

Now, compile examples with cross:
```bash
$ cross build --example hello --target x86_64-pc-windows-gnu
```
