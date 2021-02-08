# sd-id128

[![buy me a coffee](https://img.shields.io/badge/buy%20me%20a%20coffee-or%20I%20sing-53a0d0?style=flat&logo=Buy-Me-A-Coffee)](https://www.buymeacoffee.com/ente)  [![donate@paypal](https://img.shields.io/badge/paypal-donation-53a0d0?style=flat&logo=paypal)](https://www.paypal.com/donate?hosted_button_id=CRGNTJBS4AD4G)

sd-id128 is a rust wrapper for sd-id128 in the systemd API of [libsystemd](https://www.freedesktop.org/software/systemd/man/sd-id128.html). sd-id128 is part of the [systemd.rs](https://github.com/ente76/systemd.rs) project.

github.com | crates.io | docs.rs
-----------|-----------|--------
[![GitHub release (latest by date)](https://img.shields.io/github/v/release/ente76/sd-sys)](https://github.com/ente76/sd-sys) | [![Crates.io](https://img.shields.io/crates/v/sd-sys)](https://crates.io/crates/sd-sys) | [![docs.rs](https://docs.rs/sd-sys/badge.svg)](https://docs.rs/sd-sys/)
[![GitHub release (latest by date)](https://img.shields.io/github/v/release/ente76/sd-id128)](https://github.com/ente76/sd-id128) | [![Crates.io](https://img.shields.io/crates/v/sd-id128)](https://crates.io/crates/sd-id128) | [![docs.rs](https://docs.rs/sd-id128/badge.svg)](https://docs.rs/sd-id128/)
[![GitHub release (latest by date)](https://img.shields.io/github/v/release/ente76/sd-journal)](https://github.com/ente76/sd-journal) | [![Crates.io](https://img.shields.io/crates/v/sd-journal)](https://crates.io/crates/sd-journal) | [![docs.rs](https://docs.rs/sd-journal/badge.svg)](https://docs.rs/sd-journal)

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

## License

sd-id128 is published under the AGPL-3.0, individual licenses may be granted upon request.

```license
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
```
