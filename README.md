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

# Configuration

In a similar philosophy to [black][black], most things cannot be configured --
you either use mack or you don't. There is one thing you can control though: if
you don't want a particular file to be touched by mack, add `_NO_MACK` as a
substring anywhere in the comment tag.

[black]: https://github.com/ambv/black
