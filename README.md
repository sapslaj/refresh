# refresh

`watch` but (poorly) written in Rust

This is mainly a project for me to learn Rust and not meant to be used
seriously.

## y tho

### Goals

- to be a mostly drop-in replacement for procps-ng's `watch`.
- pipe stdout and stderr through, don't try to be "smart" about escape codes.
- `humantime` intervals (i have Go brainrot and `5s` makes more sense to me
  than just `5` ok i'm sorry)

### Non-goals

- to be a _full_ drop-in replacement for procps-ng's `watch`.
- `sh -c` support.
- diff support
- publishing to crates.io
