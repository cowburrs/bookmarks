# Commands

There are only 4 basic commands

## Go

This function returns a command, which assuming you have the gnu coreutil of cd
you can run/eval

```
┌──[~]
╰─$ bm go yazi
cd /home/burrs/.config/yazi
┌──[~]
╰─$ bm go yaz
bookmarks: no match found, using zoxide instead
cd /home/burrs/.config/yazi
```

This is what the g function does, it just evaluates the output (Also
the bookmarks: text and zoxide: text is redirected to stderr so they arent
evaluated by the function. This may clash if you intend to mix stderr with
stdout)

```
┌──[~]
╰─$ g yazi
┌──[~/.config/yazi]
╰─$ cd ~
┌──[~]
╰─$ g yaz
bookmarks: no match found, using zoxide instead
┌──[~/.config/yazi]
╰─$
```

`~`, `.`, `..` and other sensible defaults
will not be passed to zoxide.

This function will check to see if a directory is available to cd into, and will cd into that directory
before trying zoxide. It will not pass into zoxide if the text contains . or / at any point.

## Delete

You can delete bookmarks using this syntax, if you misstype, this command will
helpfully show you similar commands.

```
┌──[~/.config/yazi]
╰─$ bm delete yaz
bookmarks: "yaz" not found, did you mean: yazi?
┌──[~/.config/yazi]
╰─$ bm delete yazi
```

Alternatively you can delete the entry in `.config/bookmarks/dirs`, this position
will vary depending on your os of choice.

## Save

You can save bookmarks using the cli

```
┌──[~/.config/yazi]
╰─$ bm save yazi
bookmarks: Saved yazi as /home/burrs/nixos/resources/.config/yazi
┌──[~/.config/yazi]
╰─$ bm save yazi
bookmarks: key exists, first delete bookmark
```

Also alternatively you can add entries manually. This is not recommended as two entries
with the same name may cause undefined behaviour.

## List

You can search the lists. Lists are filtered out based on string similarity,
so you can misstype here.

```
┌──[~/.config/yazi]
╰─$ bm list yazi
yazi       -> $HOME/nixos/resources/.config/yazi
┌──[~/.config/yazi]
╰─$ bm list yaz
yazi       -> $HOME/nixos/resources/.config/yazi
```

This string similarity uses the Jaro-Wrinkler implementation, and filters out
scores of < 0.8

```
┌──[~/.config/yazi]
╰─$ bm list ni
nix        -> $HOME/nixos
nvim       -> $HOME/.config/nvim
nixos      -> $HOME/nixos
```
