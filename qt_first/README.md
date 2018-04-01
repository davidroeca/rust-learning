### Using the generated qt 5.8.0 bindings

Unfortunately the debian qt package is currently at verison 5.5.1 as I write,
while the rust bindings rely on qt `5.8.0`.

A source build was my work-around to get this to work.

See a [tutorial on building from source](https://www.ics.com/blog/how-compile-qt-source-code-linux)
and reference version [5.8.0](http://download.qt.io/official_releases/qt/5.8/5.8.0/single/).
I'd recommend doing an md5checksum before moving on.

On Ubuntu 16.04 (and Linux Mint 18), install the following
packages:

```bash
sudo apt install \
  bison \
  build-essential \
  flex \
  gperf \
  ibgstreamer-plugins-base0.10-dev \
  libasound2-dev \
  libatkmm-1.6-dev \
  libbz2-dev \
  libcap-dev \
  libcups2-dev \
  libdrm-dev \
  libegl1-mesa-dev \
  libfontconfig1-dev \
  libfreetype6-dev \
  libgcrypt11-dev \
  libglu1-mesa-dev \
  libgstreamer0.10-dev \
  libicu-dev \
  libnss3-dev \
  libpci-dev \
  libpulse-dev \
  libssl-dev \
  libudev-dev \
  libx11-dev \
  libx11-xcb-dev \
  libxcb-composite0 \
  libxcb-composite0-dev \
  libxcb-cursor-dev \
  libxcb-cursor0 \
  libxcb-damage0 \
  libxcb-damage0-dev \
  libxcb-dpms0 \
  libxcb-dpms0-dev \
  libxcb-dri2-0 \
  libxcb-dri2-0-dev \
  libxcb-dri3-0 \
  libxcb-dri3-dev \
  libxcb-ewmh-dev \
  libxcb-ewmh2 \
  libxcb-glx0 \
  libxcb-glx0-dev \
  libxcb-icccm4 \
  libxcb-icccm4-dev \
  libxcb-image0 \
  libxcb-image0-dev \
  libxcb-keysyms1 \
  libxcb-keysyms1-dev \
  libxcb-present-dev \
  libxcb-present0 \
  libxcb-randr0 \
  libxcb-randr0-dev \
  libxcb-record0 \
  libxcb-record0-dev \
  libxcb-render-util0 \
  libxcb-render-util0-dev \
  libxcb-render0 \
  libxcb-render0-dev \
  libxcb-res0 \
  libxcb-res0-dev \
  libxcb-screensaver0 \
  libxcb-screensaver0-dev \
  libxcb-shape0 \
  libxcb-shape0-dev \
  libxcb-shm0 \
  libxcb-shm0-dev \
  libxcb-sync-dev \
  libxcb-sync0-dev \
  libxcb-sync1 \
  libxcb-util-dev \
  libxcb-util0-dev \
  libxcb-util1 \
  libxcb-xevie0 \
  libxcb-xevie0-dev \
  libxcb-xf86dri0 \
  libxcb-xf86dri0-dev \
  libxcb-xfixes0 \
  libxcb-xfixes0-dev \
  libxcb-xinerama0 \
  libxcb-xinerama0-dev \
  libxcb-xkb-dev \
  libxcb-xkb1 \
  libxcb-xprint0 \
  libxcb-xprint0-dev \
  libxcb-xtest0 \
  libxcb-xtest0-dev \
  libxcb-xv0 \
  libxcb-xv0-dev \
  libxcb-xvmc0 \
  libxcb-xvmc0-dev \
  libxcb1 \
  libxcb1-dev \
  libxcomposite-dev \
  libxcursor-dev \
  libxdamage-dev \
  libxext-dev \
  libxfixes-dev \
  libxi-dev \
  libxrandr-dev \
  libxrender-dev \
  libxslt-dev \
  libxss-dev \
  libxtst-dev \
  perl \
  python \
  ruby
```

Then run the following:
* `./configure` pick open source, and accept the license
* `make -j${N}` where N is your number of cores minus 1.
* `sudo make install` which installs it in `/usr/local`
