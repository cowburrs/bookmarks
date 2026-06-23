# Getting started in bash

Add the following to your bashrc.

```
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

# Adding this plugin to yazi

Use the native yazi package manager to add bookmarks

`-$ ya pkg add cowburrs/bookmarks:bookmarks`

Then add the keybind to your `keymap.toml` (The following example uses the TOML table
json-like syntax)

```
# { on = "<C-z>", run = "suspend", desc = "Suspend the process" },
{ on = "<C-z>", run = "plugin bookmarks", desc = "bashmarks jump" },
```
