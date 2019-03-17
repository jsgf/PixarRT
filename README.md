# Pixar Raytracer

Original C++ code by Andrew Kensler of Pixar taken from [this blog post](https://fabiensanglard.net/postcard_pathtracer/).

I ported it to Rust and made it parallel with Rayon as a learning exercise
in both ray tracing and using Rayon.

Build with `RUSTFLAGS='-Ctarget-cpu=native ' cargo build --release` to get full optimization for
your current CPU. Use `--features vec-simd` to get explicit AVX code on platforms which support
it - though this is slower on my hardware.