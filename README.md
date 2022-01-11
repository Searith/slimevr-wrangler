# SlimeVR Wrangler

Use joycons (+) as SlimeVR trackers, enabling you to make a full body system with your existing devices. Combine with DIY SlimeVR trackers, or phones using Owotrack.

![Screenshot of the app running and tracking a single Joycon](screenshot.png)

(+) More devices planned to be added.

## Setup
You need bluetooth on your computer.
* Download and set up [SlimeVR](https://docs.slimevr.dev/slimevr-setup.html)
* Download [SlimeVR Wrangler](https://github.com/carl-anders/slimevr-wrangler/releases/latest/download/slimevr-wrangler.exe)
* Start both the SlimeVR server and SlimeVR Wrangler 
* Connect your Joycon trackers to the computer ([Guide for Windows](https://www.digitaltrends.com/gaming/how-to-connect-a-nintendo-switch-controller-to-a-pc/))
* Make sure the SlimeVR server is running, then press "Search for Joycons" inside SlimeVR Wrangler
* If you're lucky the Joycon should show up in the window!
* Follow the SlimeVR documentation to set up the new tracker, with the direction below:
* To use more than 2 joycons you must have a local IP that is port forwarding to your computers local IP on port 6969, try a phone with Fwd. Fwd might not be on Iphones so an alternative might be required.
* For more than 1 Joycon you must change the variables in src/joycon/integration.rs
* Variables that you need to change are JOYCON1SERIAL JOYCON2SERIAL JOYCON3SERIAL PORTFORWARDIP
* To connect joycons to the slime vr server you must open this with the IP setting set to local IP, connect your joycons, close out, set IP to the port forwarded IP, then connect your joycons again.
* Joycons may disconnect, might just be my bad bluetooth

### Left joycon

The rail that attaches to the switch device should be pointed toward your feet, with the joystick pointing forward.

In SlimeVR server, set direction to "forward".

You can also play around with different ways of mounting the joycon, with different directions in the SlimeVR server.

### Right joycon

Same with left Joycon but the joystick faces backwards.

## Issues

Many! This is a **pre-alpha** version, and there's no guarantees about anything.

* Rotation tracking is bad! - Yup, sorry. In the future there will be settings to help fine tune the tracking. I suggest binding a button to reset.
* It stops tracking when I turn around! - Bluetooth does not have a good range, you might have better luck with a different bluetooth adapter.
* It crashes when I reconnect the Joycon! - Yeah, working on fixing this. You'll just have to restart the SlimeVR Wrangler program.
* Probably more.

# License
GPLv3

If you contribute to this, you also agree to license your contributions with MIT License & Apache License, Version 2.0. This is because I might replace the GPL dependency in the future with something else.
