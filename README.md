# sd-id128

[![GitHub release (latest by date)](https://img.shields.io/github/v/release/ente76/sd-id128?label=github&logo=github)](https://github.com/ente76/sd-id128)  [![Crates.io](https://img.shields.io/crates/v/sd-id128)](https://crates.io/crates/sd-id128)  [![docs.rs](https://docs.rs/sd-id128/badge.svg)](https://docs.rs/sd-id128/)  ![GitHub Workflow Status](https://img.shields.io/github/workflow/status/ente76/sd-id128/test) [![buy me a coffee](https://img.shields.io/badge/buy%20me%20a%20coffee-or%20I%20sing-53a0d0?style=flat&logo=Buy-Me-A-Coffee)](https://www.buymeacoffee.com/ente)  [![donate@paypal](https://img.shields.io/badge/paypal-donation-53a0d0?style=flat&logo=paypal)](https://www.paypal.com/donate?hosted_button_id=CRGNTJBS4AD4G)  


 [sd-id128](https://github.com/ente76/sd-id128) is a rust wrapper for sd-id128 in the systemd API of [libsystemd](https://www.freedesktop.org/software/systemd/man/sd-id128.html). sd-id128 is part of the [systemd.rs](https://github.com/ente76/systemd.rs) project.


## Introduction

### Features

This library is developed against the latest version of systemd. Unfortunately not all systems are up to date in that regard. Compatibility can be mastered using features. Each feature is named after the corresponding systemd version. The following features exist currently:

- 247

Feature may be added to default features at a certain moment.

### cargo.toml

without features:

```toml
[dependencies]
sd-id128 = "0.1"
```

with support for latest version:

```toml
[dependencies]
sd-id128 = { version="0.1", features=["247"]}
```

### Examples

```rust
use sd_id128::{Case, Format, ID128};
let id128 = ID128::boot_id().unwrap();
println!("The boot id in RFC format is: {}", id128);
println!("The boot id in upper case libsystemd format is: {}",
         id128.to_string_formatted(Format::LibSystemD, Case::Upper));
```

## Version History

- 01.01.2021 v0.1.0
  - initial version
- 01.01.2021 v0.1.1
  - documentation errors fixed
- 08.02.2021 v0.1.2
  - project repository moved gitlab --> github
  - minor documentation improvements
- (planned) v0.1.3
  - introduce feature 247

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
