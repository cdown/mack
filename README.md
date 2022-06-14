[![Tests](https://img.shields.io/travis/cdown/mack/master.svg)](https://travis-ci.org/cdown/mack)

`mack` is to music files as [black][black] is to code formatting. It enforces
standards around both consistency of the metadata (e.g. ID3 version) and the
metadata itself (e.g. "feat" tagging).

# Examples of fixes

- Moving featured artists from the artist tag to the title
- Enforcing a consistent "feat" format in title tags
- Whitespace normalisation
- Renaming files to format "{artist}/{album}/{track number} {title}"

# Usage

    mack [DIR]

You can also see what would be changed first using `--dry-run`.

# Building

Run `cargo build` as normal.

# Performance

`mack` has a strong focus on performance. Files which were not updated since the
last mack run will not be examined at all. On a sample modern laptop with a
mid-spec SSD, this means that we only take 0.02 seconds to run over 5000 files
under most circumstances (0.2 seconds on the very first run).

# Configuration

In a similar philosophy to [black][black], most things cannot be configured --
you either use mack or you don't. There is one thing you can control though: if
you don't want a particular file to be touched by mack, add `_NO_MACK` as a
substring anywhere in the comment tag.

[black]: https://github.com/ambv/black
