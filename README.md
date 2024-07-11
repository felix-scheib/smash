# SMasH
## **S**hared **M**emory (**a**daption for **s**peedy) **H**ermit(s)
`SMasH`provides an **easy to use** interface for **sharing memory** among multiple [Hermit](https://github.com/hermit-os/hermit-rs) unikernels.

Although `SMAsH` is primarily developed for the `Hermit` unikernel it can also be used in other [Rust](https://www.rust-lang.org/) environments to share memory amogn processes.

## Objectives
- handling of shared memory like any other **smartpointer**
- no `unsafe` code for the user
- fully distributed memory, no single points of failure
- usage of idiomantic **Rust**
