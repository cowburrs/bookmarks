# Quick Start

## Getting started in bash

Add the following to your bashrc.

```bash
g() {
	eval "$(bm go "$@")"
}
```

Then you can use it like so:

```
┌──[~]
╰─$ g yazi
┌──[~/.config/yazi]
╰─$
```

## Adding this plugin to yazi

### Nix

You can use the plugin declaratively without home manager, as defined in
[configuration](configuration.md#yazi)

### Other Distributions

Or alternatively use the native yazi package manager to add bookmarks

`-$ ya pkg add cowburrs/bookmarks:bookmarks`

### Keybinding

Then add the keybind to your `keymap.toml` (The following example uses the TOML table
json-like syntax)

```toml
# { on = "<C-z>", run = "suspend", desc = "Suspend the process" },
{ on = "<C-z>", run = "plugin bookmarks", desc = "bashmarks jump" },
```
