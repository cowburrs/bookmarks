# Prerequisites

- This is strictly a linux tool, this tool is untested
  - \*although the implemetation of the directory saving is OS agnostic
- A shell is necessary, this is a cli tool

# Nix

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

# Other linux distributions

See the release tab in github, all binaries are compiled using github actions.
Meaning they're reasonably safe. (This binary is unwrapped without dependancies
like zoxide because it bloats the binary around 1.5mb -> 30mb)
