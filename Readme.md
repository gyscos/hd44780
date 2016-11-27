A rust library to controll LCD modules based on the [Hitachi HD44780].
This will also work with compatible controllers, like the Samsung KS0066.

Those often have 2 lines of 16 characters.

This library uses `#![no_std]` and is therefore suitable for use in microcontrollers.

To debug it on your computer, you may want to use the [hd44780_simulator] library.

[Hitachi HD44780]: https://www.google.com/search?q=Hitachi+HD44780&tbm=isch
[hd44780_simulator]: https://github.com/gyscos/hd44780_simulator
