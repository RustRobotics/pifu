
# How to cross compile
[cross](https://github.com/rust-embedded/cross) is a great tool to do this job.

On Debian based system, follow these steps:
1. Install docker: `sudo apt install docker`
2. Install cross: `cargo install cross`
3. Add current user to docker group: `sudo adduser ${USER} docker; newgrp docker`

Now, compile examples with cross:
```bash
$ cross build --example hello --target x86_64-pc-windows-gnu
```
