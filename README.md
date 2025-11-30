# vslam-rs
Hobby project for VSLAM built with Rust

![Rust Build Status](https://github.com/sachinkum0009/vslam-rs/actions/workflows/rust-build.yml/badge.svg)
![Pre-commit Build Status](https://github.com/sachinkum0009/vslam-rs/actions/workflows/pre-commit.yml/badge.svg)


## Architecture

```mermaid
---
title: VSLAM Architecture
---

classDiagram
    class VSLAM {
        +new()
        +process_frame(frame: Frame) -> Pose
        +get_map() -> Map
    }

    class Frame {
        +id: u32
        +image_data: Image
        +timestamp: u64
    }

    class Pose {
        +position: Vector3
        +orientation: Quaternion
    }

    class Map {
        +keyframes: Vec<Frame>
        +landmarks: Vec<Landmark>
    }

    class Landmark {
        +id: u32
        +position: Vector3
    }
```
