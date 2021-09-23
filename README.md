# Install_A_Bot

Install_A_Bot is a program that will install and/or set up programs and files on your machine.

## Just why?

We all know the situation:
You have just gotten a new machine.
You synched your dotfiles and maybe set up a few very basic programs.
But as time goes by your machine consists of so much more:
Plain files you have lying around and use as input for other programs (e.g. wordlists if you are into pentesting), software you "git pull'ed" and maybe even applied a patch or two before you use it,
other tools and frameworks you set up via a script call (like [GEF](https://github.com/hugsy/gef), [Oh My Zsh](https://ohmyz.sh) or [Hombrew](https://brew.sh)).

Or in short: Setting up a computer until it's truly yours can take an awful lot of time. Install_A_Bot to the rescue!

## How to use

The basic idea is: you fill out a config file (currently called _packages.toml_) and Install_A_Bot automates all the rest.
It will install regular programs via the provided package manager, "git pull" and "make install" other repositories and "curl" down your files and whatever else you tell it to do.
It even supports different so called targets, so you can e.g. have a the same config file for your Macbook and your Ubuntu desktop.

And the best part:
Since the configuration is just a text file you can easily share it and store it wherever you want (e.g. Dropbox, a separate git repo, ...).

## Current state

Currently Install_A_Bot is in early development and not ready for use.
The README will be updated as soon as it's usable.

## Building on Linux

Install\_A\_Bot uses libcurl bindings to facilitate downloads from the web.
Since however a user is also supposed to be able to just download a release binary be good to go,
we statically link libcurl and openssl (required by libcurl on Linux).

Building openssl requires a short perl setup outlined below:

```bash
# A helper for CPAN downloads, makes our lives easier down the road
cpan App::cpanminus
cpanm FindBin::Real
```