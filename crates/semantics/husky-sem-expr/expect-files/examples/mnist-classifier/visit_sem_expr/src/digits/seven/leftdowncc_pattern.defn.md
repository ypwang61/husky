```rust
Some(
    [
        "cc",
        "cc.displacement()",
        "let dp = cc.displacement()",
        "dp",
        "dp.y",
        "0.0",
        "dp.y < 0.0",
        "dp.y < 0.0",
        "require dp.y < 0.0",
        "cc",
        "cc.relative_bounding_box",
        "cc.relative_bounding_box.ymin()",
        "0.3",
        "cc.relative_bounding_box.ymin() < 0.3",
        "cc.relative_bounding_box.ymin() < 0.3",
        "require cc.relative_bounding_box.ymin() < 0.3",
        "cc",
        "cc.start_tangent()",
        "true",
        "cc.start_tangent().angle(true)",
        "let ang = cc.start_tangent().angle(true)",
        "ang",
        "30.0",
        "ang < 30.0",
        "ang < 30.0",
        "require ang < 30.0",
        "ang",
        "ang",
        "let dp = cc.displacement()\n    require dp.y < 0.0\n\n    require cc.relative_bounding_box.ymin() < 0.3\n\n    let ang = cc.start_tangent().angle(true)\n    require ang < 30.0\n    ang",
    ],
)
```