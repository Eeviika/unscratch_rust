# UnScratch

[![language](https://img.shields.io/badge/language-Rust-orange)](https://rust-lang.org/)

A program / CLI tool to pack and unpack Scratch `.sb3` files on your filesystem.

It can be used to unpack and "export" sprites, assets, etc. from `.sb3` files, and convert them into readable `.json`, `.png` / `.svg`, or `.mp3` files.
Or do the same in reverse.

**It is recommended to use this version over [the NodeJS version of Unscratch](https://github.com/Eeviika/UnScratch).**

## Description

UnScratch is a tool that can "unpack" `.sb3` files into a project directory containing all the sprite
files, code, assets, and initial variables / lists; making it easier for other projects to read, but also
making it git-friendly as well.

UnScratch should (mostly) work on any TurboWarp and Scratch project, though support for other platforms like PenguinMod is uncertain.

This is a new project (as well as my first open-source project), so there's not a lot to be said as of right now.
Soon, I hope that UnScratch will have more features like assigning user-friendly IDs instead of the
randomized (and confusing) IDs that Scratch uses, as well as making certain files (like code files)
easier to digest and read.

## Support

If you encounter any issues with UnScratch (which honestly, you probably will), please explain
the issue in full. Specifically, mention:

* The command you ran
* UnScratch's output (in verbose mode)
* Any errors that UnScratch threw
* What the expected result was

## Contributing

This project is open to contributions and actively encourages them as I am a solo dev with not a lot
of time on my hands. If you'd like to contribute, please feel free to do so. When contributing, please do
the following:

* Try to branch off of `dev` for most things.
* Name your branch something clear, like `feature/scratch_assets_rename`. If it's a bugfix, `fix/weird_bug`.
