# sd-id128

sd-id128 is a rust wrapper for sd-id128 in the systemd API of the native library `libsystemd`. See <https://www.freedesktop.org/software/systemd/man/sd-id128.html> for details.

sd-id128 is part of the [systemd.rs](https://gitlab.com/systemd.rs) project

- [sd-id128](https://gitlab.com/systemd.rs/sd-id128) [![Crates.io](https://img.shields.io/crates/v/sd-id128)](https://crates.io/crates/sd-id128) [docs.rs](https://docs.rs/sd-id128/0.1.1/sd_id128/)
- [sd-sys](https://gitlab.com/systemd.rs/sd-sys) [![Crates.io](https://img.shields.io/crates/v/sd-sys)](https://crates.io/crates/sd-sys) [docs.rs](https://docs.rs/sd-sys/0.1.0/sd_sys/)
- [sd-journal](https://gitlab.com/systemd.rs/sd-journal)

systemd.rs is an alternative to the [systemd-rust](https://github.com/jmesmon/rust-systemd) project.

- systemd.rs is published under the AGPL-3.0 license. Individual/commercial licenses are available upon request.
- focused coverage of sd-id128 & sd-journal only (currently there are no plans to extend this coverage)
- good documentation with links to the libsystemd documentation
- 100% coverage of libsystemd within the area of focus
- good test coverage

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

Individual licenses may be granted upon request.

## ToDo

- [ ] Implement Serde as feature
- [ ] Check Trait completenes
  - [ ] Ord
  - [ ] PartialOrd
  - [ ] Hash
- [ ] check conversion completenes
  - [ ] AsRef
  - [ ] AsMut
- [ ] improve documentation with examples
- [ ] extend tests
- [ ] setup bench tests
