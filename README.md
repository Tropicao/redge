# Redge

## What is Redge

Redge is a command-line tool for uploading sport activities from a tracker (bike GPS, smart watch, etc) to [Strava](https://www.strava.com/). Most of sport trackers are able to upload activities wireless (with help of a smartphone), but :
* some of them still need a good old fashion USB cable, and as a consequence, an application, not always available for your platform (eg: Linux)
* some devices struggle to upload through a smartphone, wireless, but offer the USB alternative.

Redge aims to ease the USB use case by providing a straightforward command line tool to upload activities from any plugged device.

## Current status

For now, redge only supports uploading activities from [Garmin Edge 200 GPS](https://www.garmin.com/fr-FR/p/90675), and as a consequence, only bike activities.

## How to use
* build redge : `cargo build` (if you do not have `cargo` on your machine, follow [these instructions](https://doc.rust-lang.org/cargo/getting-started/installation.html))
* you must create a "Strava application", which will own rights to interact with Strava, once you have granted it the necessary rights
    * you can do so by visiting https://www.strava.com/settings/api, and following the guidance to create your application
    * once done, note your **client ID** and **client_secret** (warning : the last one must remain secret)
* plug your device to your computer with a USB cable. Wait for it to appear as a device onto your computer (it may take a few seconds, depending on the device)
* then start redge : `./redge`. Since it is the first time you are running it, `redge` will guide you to grant access to the Strava application to your account, by prompting credentials and asking you to authenticate through OAuth
    * You can speed up this process by providing on `redge` command line the necessary arguments : `./redge -i your_client_id -s your_client_secret`
* When authorization is done, `redge` will save locally all needed credentials locally, so every step until this one was only for the first run.
* Then `redge` will find the latest activity on your device/tracker and try to push it to your Strava account

## Miscellaneous
* This project is a rewrite of [edge200_exporter](https://github.com/Tropicao/edge200_exporter), for the sole purpose of familiarizing with Rust
* More features and development tools will come soon, since I will keep this project as a learning tool. For example :
    * listing and selecting the activities to upload
    * support other devices/activities type (for example : running watch)
    * unit testing
    * monitoring of uploading activities
* I will gladly take any comment/suggestion about the code (architecture, coding style, features request, etc). Feel free to open issues and/or pull requests !