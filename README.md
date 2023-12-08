# display-binding-rs

Binding template/demo interfacing lvgl-rclib-rs with libafb-rs micro service architecture.

## Building dependencies:

* libafb-rs: git@git.ovh.iot:redpesk/redpesk-common/afb-librust.git
* liblvgl-rs: git@github.com:tux-evse/lvgl-rclib-rs.git

### Compiling for frame-buffer

Default driver is frame-buffer, it is typically what is used for embedded devices.

```
nm /usr/local/lib64/liblv_drivers.so | grep fbdev_init
cargo build
```

### Compiling with GTK emulator

For development and especially for business logic debug with vscode/llgb-gdb, it is far simpler to use GTK than FBDEV.

* Check GTK driver is enabled
* Select GTK by setting ```USE_GTK=1``` environnement variable

```
nm /usr/local/lib64/liblv_drivers.so | grep gtk_init
USE_GTK=1 cargo build
```

## start display-binding

```
display-binding/afb-binding/etc/binding-test.sh
firefox --new-window http://localhost:1234
```

## Demo screen on framebuffer

In order to use GTK frame-buffer emulation, you should
* check that kvgl-rclib-rs is installed with GTK enabled
* USE_GTK=1 cargo build

![LVGL demo screen](Docs/lvgl-demo-screen.png)

## Demo afb-v4 API

![LVGL demo api](Docs/lvgl-demo-api.png)


## Rust LVGL api sample

Check Display::draw_panel within display-lvgl.rs for more samples

Create a label
```Rust
LvglLabel::new("Label-1", "Tux EvSe UI", 50, 400)
            .set_info("Demo Label widget")
            .set_size(300, 100)
            .set_color(LvglColor::rvb(0, 0, 0))
            .set_background(LvglColor::rvb(0xFF, 0xFF, 0xFF))
            .set_border(3, LvglColor::rvb(0, 0xff, 0))
            .finalize();
```

Create a button
```Rust
LvglButton::new("Button-A", "Test-1", 100, 200)
    .set_info("Demo Button 1")
    .set_size(180, 100)
    .finalize();
```

## Faire un screencast du framebuffer

Copy framebuffer and transform it yo PNG. If needed crop image to content with gimp.
```
cp /dev/fb0 > /tmp/screen.data
RESOLUTION="1920x1080"
ffmpeg -vcodec rawvideo -f rawvideo -pix_fmt rgb32 -s $RESOLUTION -i /tmp/screen.data -f image2 -vcodec png screenshot.png
```

## testing without root privilege

1) In order to test without admin privileges, you need access to

* /dev/fb0 generally with 'video' group
* /dev/input0 usually with 'input' group

2) find which virtual console hold /dev/fb0 framebuffer

* switch virtual console with (Alt+Ctrl+F1) (Alt+Ctrl+F2) ...
* after login check with cat /dev/urandom >/dev/fb0

If your screen is repainted, then you are on /dev/fb0 other wise switch to next virtual console.