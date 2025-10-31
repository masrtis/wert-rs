# Ray Tracing In One Weekend

A ray tracer inspired by [Ray Tracing In One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html), written in Rust. This repository contains a ray tracer capable of generating the final image from the book using Rayon to parallelize the color computation by pixel and by sample. The final image without the parallel algorithm took roughly 7000 seconds to render, while parallelization generates an image in about 270 seconds.

## TODOs

- Continue on with the rest of the books in the series
- CI pipeline using Github Actions
- Move calculations to the GPU
