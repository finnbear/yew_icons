## 0.9.0

Update to `yew@0.22`.

Replace `enum IconId` (having a feature-flagged variant per icon) with `struct IconData` (having an associated `const` per icon). Now there is only one feature flag per icon collection.

Before:
```rust
<Icon icon_id={IconId::LucideArrowLeftCircle}/>
```

After:
```rust
<Icon data={IconData::LUCIDE_ARROW_LEFT_CIRCLE}/>
```

This was motivated by [this crates.io issue](https://blog.rust-lang.org/2023/10/26/broken-badges-and-23k-keywords/) but is also associated with a significant binary size reduction.