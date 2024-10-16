# Intel Bluetooth Firmware Extracter

Extracts the bluetooth firmware files from
`FMP/Win10_UWDRelease/x64/ibtusb.sys`.
Only tested on that driver and versions 23.70.3
and 23.80.0.

Download `BT-23.80.0-64UWD-Win10-Win11.exe` from Intel
(sha256: `e993a7dd88d868e8f8231618617c397cb31bf874bb5753a09fed5a6dffa5d0c4`)
and install it in a clean WINE prefix to get this driver.

To find the names of each firmware, there is a table
in the driver which pairs an address to the start
of a firmware file to an address to a string describing it.
Use IDA or another tool to observe it and look for the right
firmware.

For driver version 23.80.0, this table starts at
file offset `1B7EEE0h`.
The IML and USB firmwares for my 256v-powered device were
images 17 (`sfi_BLAZARI_B0_IML`)
and 16 (`sfi_BLAZARI_B0_FMP_C0_USB`).

Adding these as `intel/ibt-0190-0291-iml.sfi` and
`intel/ibt-0190-0291-usb.sfi` to `linux-firmware`
enables bluetooth fully on my device.

This tool is terribly designed but it works for this
specific case so it doesn't matter.
