# mack | [![Tests](https://img.shields.io/github/actions/workflow/status/cdown/mack/ci.yml?branch=master)](https://github.com/cdown/mack/actions?query=branch%3Amaster)

mack is an opinionated, fast music organiser. It enforces:

- Directory layout
- File name format
- Metadata consistency (e.g., consistent "feat" tagging)
- Format consistency (e.g., ID3 version)
- ...and more!

## Examples of fixes

- Moving featured artists from the artist tag to the title
- Enforcing a consistent "feat" format in title tags
- Whitespace normalisation
- Renaming files to format "{artist}/{album}/{track} {title}", or another
  format specified with `--fmt`

## Usage

See `--help`. An example invocation is:

    % mack --dry-run -o Music .
    01 Pyramid.mp3: renamed to Music/宇宙コンビニ/染まる音を確認したら/01 Pyramid.mp3
    02 8films.mp3: renamed to Music/宇宙コンビニ/染まる音を確認したら/02 8films.mp3
    03 tobira.mp3: renamed to Music/宇宙コンビニ/染まる音を確認したら/03 tobira.mp3
    04 Compass.mp3: renamed to Music/宇宙コンビニ/染まる音を確認したら/04 Compass.mp3
    05 strings.mp3: renamed to Music/宇宙コンビニ/染まる音を確認したら/05 strings.mp3

You can see what would be changed first using `--dry-run`.

## Installation

    cargo install mack

## Performance

mack has a strong focus on performance. Files which were not updated since the
last mack run will not be examined at all. On a sample modern laptop with a
mid-spec SSD, this means that we only take 0.005 seconds to run over ~3500
files under most circumstances (0.015 seconds on the very first run).

## Configuration

If you don't want a particular file to be touched by mack, add `_NO_MACK` as a
substring anywhere in the comment tag.
