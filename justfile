rootdir := ''
prefix := '/usr'
debug := '0'


export NAME := 'cosmic-ext-applet-netspeed'
export APPID := 'io.github.khanra17.' + NAME

cargo-target-dir := env('CARGO_TARGET_DIR', 'target')
bin-src := cargo-target-dir / if debug == '1' { 'debug' / NAME } else { 'release' / NAME }

base-dir := absolute_path(clean(rootdir / prefix))
share-dst := base-dir / 'share'

bin-dst := base-dir / 'bin' / NAME
desktop-dst := share-dst / 'applications' / APPID + '.desktop'
icon-dst := share-dst / 'icons/hicolor/scalable/apps' / APPID + '-symbolic.svg'

default: build-release

build-debug *args:
  cargo build {{args}}

build-release *args:
  cargo build --release {{args}}

install:
  install -Dm0755 {{bin-src}} {{bin-dst}}
  install -Dm0644 res/desktop_entry.desktop {{desktop-dst}}
  install -Dm0644 res/app_icon.svg {{icon-dst}}

uninstall:
  rm {{bin-dst}}
  rm {{desktop-dst}}
  rm {{icon-dst}}

clean:
  cargo clean

run:
  flatpak run io.github.khanra17.cosmic-ext-applet-netspeed
