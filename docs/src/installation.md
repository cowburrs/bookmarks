# Installation

## Prerequisites

#### \*Nix users can skip this step.

- This is strictly a linux tool, this tool is untested for non linux operating systems.

  - *\*although the implementation of the directory saving is OS agnostic*

- Bookmarks utilizes Zoxide when there is no directory bookmark saved.

- Bash is officially supported with bookmark.

- Manpages is also available, if that is a preferred method of documentation.

- Nix is HIGHLY recommended, these docs show all steps and nixos is the only officially
  supported os, and nix is the only supported package manager.

## Nix

Add the repository to your inputs in flake.nix

```nix
inputs = {
  bookmarks = {
    url = "github:cowburrs/bookmarks";
  };
};
```

If your attribute set input to output is named as inputs like so:

```nix
outputs =
{
  self,
  nixpkgs,
  nixpkgs-unstable,
  ...
}@inputs:
```

You can define the following in your modules (or if you inherit inputs you can
add this to your configuration.nix)

```nix
environment.systemPackages =
  with pkgs;
  [
    inputs.rose-pine-hyprcursor.packages.${pkgs.system}.default
  ];
```

## Other linux distributions

See the release tab in github, all binaries are compiled using github actions.
Meaning they're reasonably safe.

\[TL Note\]: Autocompletions and Man pages and dependencies are not shipped.
The nix download is heavily favoured over this manual approach
