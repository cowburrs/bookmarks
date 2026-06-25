# Configuration

## Nix

Optionally you can add the yazi plugin through nix. Heres my current
configuration as a sample that might help you get started. This is my
current configuration as of jun 25

Assuming you have bookmarks defined in your inputs.

```nix
  programs.yazi = {
    plugins = {
      bookmarks = "${inputs.bookmarks}/bookmarks.yazi";
    };
  };
```

*NOTE: You need the prior nix configuration defined in [installation](installation.md).
Reiterating, you can check my own dotfiles to see a working configuration.*

## Linux

Bookmarks follows the freedesktop.org specification $XDG_CONFIG_HOME. To configure
the bookmarks directory where the bookmarks are stored, optionally pass the
environment variable to the command

```bash
-$ XDG_CONFIG_HOME=/path/to/dir bm save yazi
```

If using this method, you should use either nix's built in pkgs [trivial builders](https://ryantm.github.io/nixpkgs/builders/trivial-builders/),
or create a shell function/alias.

You can also see [Configuration Options](config-options.md)
