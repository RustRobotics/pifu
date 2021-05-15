
# About
Pifu(a.k.a. 蚍蜉), is a cross platform package builder.

## Build dependencies
- exe: [nsis](https://nsis.sourceforge.io/)
- AppImage: [appimagetool](https://github.com/AppImage/AppImageKit/releases), libc-bin (for `ldd`)
- rpm: rpm (for `rpmbuild` command)

## Run dependencies
- AppImage: glusterfs-client

## Related projects
- https://github.com/burtonageo/cargo-bundle
- https://github.com/mmstick/cargo-deb
- https://github.com/electron-userland/electron-builder
- https://github.com/jordansissel/fpm
- https://github.com/AppImage/appimagekit
- https://github.com/linuxdeploy/linuxdeploy
- https://github.com/probonopd/linuxdeployqt
- https://github.com/AppImageCrafters/appimage-builder

## Misc
Linux desktop file can be validated by `desktop-file-validate` command,
which is included in `desktop-file-utils` package.
