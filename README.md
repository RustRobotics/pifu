
# About
![Build status](https://github.com/RustRobotics/pifu/actions/workflows/rust.yml/badge.svg)
[![Latest version](https://img.shields.io/crates/v/pifu.svg)](https://crates.io/crates/pifu)
[![Documentation](https://docs.rs/pifu/badge.svg)](https://docs.rs/pifu)
![Minimum rustc version](https://img.shields.io/badge/rustc-1.62+-yellow.svg)
![License](https://img.shields.io/crates/l/pifu.svg)

Pifu(aka 蚍蜉), is a cross platform package builder.

## Install
```bash
cargo install pifu
```

## Build dependencies
- exe: [nsis](https://nsis.sourceforge.io/)
- appimage: [appimagetool](https://github.com/AppImage/AppImageKit/releases), libc-bin (for `ldd`)
- rpm: rpm (for `rpmbuild` command)
- dmg: genisoimage (to generate dmg file), dmg2img (to test dmg file)

## Run dependencies
- appimage: glusterfs-client

## Related projects
- [nsis](https://nsis.sourceforge.io)
- [cargo bundle](https://github.com/burtonageo/cargo-bundle)
- [cargo packager](https://github.com/crabnebula-dev/cargo-packager)
- [cargo deb](https://github.com/mmstick/cargo-deb)
- [electron builder](https://github.com/electron-userland/electron-builder)
- [fpm](https://github.com/jordansissel/fpm)
- [appimagekit](https://github.com/AppImage/appimagekit)
- [linuxdeploy](https://github.com/linuxdeploy/linuxdeploy)
- [linuxdeployqt](https://github.com/probonopd/linuxdeployqt)
- [appimage builder](https://github.com/AppImageCrafters/appimage-builder)
- [create dmg](https://github.com/create-dmg/create-dmg)
- [node appdmg](https://github.com/LinusU/node-appdmg)
- [dmgbuild](https://github.com/al45tair/dmgbuild)

## Misc
Linux desktop file can be validated by `desktop-file-validate` command,
which is included in `desktop-file-utils` package.
