# sd-id128

sd-id128 is a rust wrapper for sd-id128 in the systemd API of the native library `libsystemd`. See <https://www.freedesktop.org/software/systemd/man/sd-id128.html> for details.

sd-id128 is an alternative to systemd from the [systemd-rust](https://github.com/jmesmon/rust-systemd) project. The main differences of sd-id128 to systemd are:

- sd-id128 is published under the AGPL-3.0 license. Individual/commercial licenses are available upon request.
- focused coverage of sd-id128 only ([sd-journal](https://gitlab.com/systemd.rs/sd-journal) is available as well)

## License

sd-id128: a wrapper for sd-id128 of libsystemd

Copyright (C) 2020 Christian Klaue [mail@ck76.de]

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.

Individual licenses are granted upon request.

## ToDo

- Implement Serde as feature
- Implement Logging as feature
- Test send and sync
- Check Trait completenes
  - Ord
  - PartialOrd
  - Hash
- check conversion completenes
  - AsRef
  - AsMut
- improve documentation
- extend tests
- setup bench tests
