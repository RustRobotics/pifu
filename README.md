
# About
Pifu(aka 蚍蜉), is a cross platform package builder.

## Install
```bash
cargo install pifu
```

## Build dependencies
- exe: [nsis](https://nsis.sourceforge.io/)
- AppImage: [appimagetool](https://github.com/AppImage/AppImageKit/releases), libc-bin (for `ldd`)
- rpm: rpm (for `rpmbuild` command)
- dmg: genisoimage (to generate dmg file), dmg2img (to test dmg file)

## Run dependencies
- AppImage: glusterfs-client

## Related projects
- https://nsis.sourceforge.io
- https://github.com/burtonageo/cargo-bundle
- https://github.com/mmstick/cargo-deb
- https://github.com/electron-userland/electron-builder
- https://github.com/jordansissel/fpm
- https://github.com/AppImage/appimagekit
- https://github.com/linuxdeploy/linuxdeploy
- https://github.com/probonopd/linuxdeployqt
- https://github.com/AppImageCrafters/appimage-builder
- https://github.com/create-dmg/create-dmg
- https://github.com/LinusU/node-appdmg
- https://github.com/al45tair/dmgbuild

## Misc
Linux desktop file can be validated by `desktop-file-validate` command,
which is included in `desktop-file-utils` package.
