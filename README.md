# yew_icons

This crate makes it easy to use [SVG icons](https://finnbear.github.io/yew_icons/) from the following collections (more may be added in the future):

- [Bootstrap](https://icons.getbootstrap.com/) - MIT License
- [Feather](https://feathericons.com/) - MIT License
  - Deprectated (use Lucide)
- [Font Awesome](https://fontawesome.com/icons) - CC BY 4.0 License
  - Regular
  - Solid
- [Lipis Flag Icons](https://github.com/lipis/flag-icons)* - MIT License
- [Lucide](https://github.com/lucide-icons/lucide) - ISC License
  - This is a maintained fork of Feather
- [Octicons](https://primer.style/octicons/) - MIT License

*not all icons included in [gallery](https://finnbear.github.io/yew_icons/).

## Usage

Use the [gallery](https://finnbear.github.io/yew_icons/) to find icons you like, and add them as feature flags.

```toml
[dependencies]
yew_icons = {version = "0.5", features = ["LucideArrowLeftCircle", "LucideArrowRightCircle", "LucideArrowUpCircle"]}
```

Then, add an `<Icon>` component with the corresponding icon id, optionally specifying a `width`, `height`, and/or `onclick` callback.

```rust
use yew::prelude::*;
use yew_icons::{Icon, IconId};

html!{
    <>
        <Icon icon_id={IconId::LucideArrowLeftCircle}/>
        <Icon icon_id={IconId::LucideArrowUpCircle} width={"2em".to_owned()} height={"2em".to_owned()}/>
        <Icon icon_id={IconId::LucideArrowRightCircle} onclick={Callback::from(|_: MouseEvent| {})}/>
    </>
}
```

![example image](example.png)

Note: The icons will inherit their parent's CSS `color`.

## Feature Flags

Each icon collection must be included with the corresponding feature flag, such as `lucide` or `font_awesome_solid`.

To save binary size, individual icons can also be included by feature flag, such as `LucideZoomIn` or `FontAwesomeSolidAtom`.

By default, no collections or icons are included. Be warned that including too many icons may result in a `.wasm` binary
that some WebAssembly engines refuse to load, especially in debug mode (see https://github.com/rustwasm/wasm-pack/issues/981).

## License

Code is licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

Icons are licensed by their respective creators (see above). An license summary is emitted to the DOM for each icon:
```html
<svg data-license="...original license...">
```

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.