# SMasH
## **S**hared **M**emory (**a**daption for **s**peedy) **H**ermit(s)
`SMasH`provides an **easy to use** interface for **sharing memory** among multiple [Hermit](https://github.com/hermit-os/hermit-rs) unikernels.

Although `SMAsH` is primarily developed for the `Hermit` unikernel it can also be used in other [Rust](https://www.rust-lang.org/) environments to share memory amogn processes.

## Prerequisites
To run `Hermit` unikernels two additional files are needed.
- [hermit loader](https://github.com/hermit-os/loader) to run the unikernel with `QEMU` (`./get_loader.sh` provides an easy way to get the latest version)
- [virtiofsd](https://gitlab.com/virtio-fs/virtiofsd) to bridge the unikernel's filesystem with the host ([prebuild binaries](https://gitlab.com/virtio-fs/virtiofsd/-/releases) available for x86_64)

## Objectives
- handling of shared memory like any other **smartpointer**
- no `unsafe` code for the user
- fully distributed memory, no single points of failure
- usage of idiomantic **Rust**

## Building
To use `Hermit` the [Rust toolchain](https://www.rust-lang.org/) and [rust-std-hermit](https://github.com/hermit-os/rust-std-hermit) are required. The [Protocol Buffers](https://protobuf.dev/) compiler `protoc` is needed for the shared memory objects.

The complete development environment is provided by as [Docker image](ghcr.io/felix-scheib/felix-scheib/smash/buildenv).

The following command will build the current directory in **development** configuration. To use any other `cargo` command, simply override the entrypoint of the container.
```shell
docker run -v $(pwd):/app:Z ghcr.io/felix-scheib/felix-scheib/smash/buildenv:1.0
```
