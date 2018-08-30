[![Tests](https://img.shields.io/travis/cdown/mack/master.svg)](https://travis-ci.org/cdown/mack)

mack is to music files as [black][black] is to code formatting. It enforces
standards around both consistency of the metadata (eg. ID3 version) and the
metadata itself (eg. "feat" tagging).

# Examples of fixes

- Moving featured artists from the artist tag to the title
- Enforcing a consistent "feat" format in title tags
- Whitespace normalisation

# Usage

    mack [DIR]

You can also see what would be changed first using `--dry-run`.

# Building

You need [TagLib](http://taglib.org/) installed on your system to build. This
can be found in the following packages:

- Arch Linux: [taglib](https://www.archlinux.org/packages/extra/x86_64/taglib/)
- CentOS/Fedora: taglib
- Gentoo: [media-libs/taglib](https://packages.gentoo.org/packages/media-libs/taglib)
- Ubuntu/Debian: [libtagc0-dev](https://packages.debian.org/search?searchon=names&keywords=libtagc0-dev)


After that, `cargo build` as normal.

# Performance

mack has a strong focus on performance. Files which were not updated since the
last mack run will not be examined at all. On a sample modern laptop with a
mid-spec SSD, this means that we only take 0.02 seconds to run over 5000 files
under most circumstances (0.2 seconds on the very first run).

# Configuration

In a similar philosophy to [black][black], most things cannot be configured --
you either use mack or you don't. There is one thing you can control though: if
you don't want a particular file to be touched by mack, add `_NO_MACK` as a
substring anywhere in the comment tag.

[black]: https://github.com/ambv/black
