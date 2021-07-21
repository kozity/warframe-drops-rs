# DROP
This is a rickety command line script for [Warframe][1] item drop information. I mean seriously, a lot of things are hardcoded (although it should work on most UNIX-like systems), and certain search keys ("cetus bounty", even with "-d") will return nothing useful. Nevertheless, for quick and simple searches, it's great.

## Installation
Pretty much your only option is to download the source and compile it yourself. If you have a working `cargo` installation, you can `cargo install` it (from source; this is not in [crates.io][2]) just fine. See `Cargo.toml` for dependencies if you're into that sort of thing. The crates therein are the only things handling network connections, for the security-skeptical. In the same vein, you should never need to run this with root permissions.

## Usage
`drop [ --update | --dedicated <search-key> | <search-key> ]`

### Flags
This program will accept no more than one flag at a time.

Short | Long | Effect
------|------|-------
`-u` | `--update` | Updates the drop table by pulling a new copy from the hardcoded URL and overwriting the hardcoded filepath. This flag takes no arguments.
`-d` | `--dedicated` | Changes the search logic to look for a table entirely dedicated to the search term. Takes one argument: the search keyphrase. See note below.

*Note:* It takes some experimentation to know which terms to use `-d` for. I frequently use `-d arbitrations` for the full Arbitration reward tables (the mission ones, not the Arbitration Honors store). I use no flag for things like `epitaph` or `'corrupted holokeys'` (note that spaces must be quoted or otherwise escaped).

[1]: https://www.warframe.com
[2]: https://crates.io
