# Tour

<img src="https://gitlab.gnome.org/GNOME/gnome-tour/raw/master/data/icons/org.gnome.Tour.svg" width="128" height="128" />
<p>GNOME's Tour & Greeter.</p>

## Screenshots

<div align="center">
![screenshot](data/resources/screenshots/screenshot1.png)
</div>


### Video Feature
Tour uses by default the logo of the distribution based on the info from `/etc/os-release`. The application comes with a feature to replace the logo with a welcome video shipped by the distribution.

To enable the feature, you need to build the application with
```bash
meson _builddir -Dvideo_path=/absolute/path/to/the/video.mp4
```

If you're testing the application using Builder, make sure to change the `config-opts` accordingly & give the application filesystem access so it can play the video file.

Example:

This needs to be added to the `gnome-tour` module
```json
"config-opts" : [
    "-Dvideo_path=/home/username/to/the/video.mp4"
]
```

along with `--filesystem=home` in `finish-args`
