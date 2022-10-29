# Peugeot-207-CAN-Bus


## Description
Command line interface for the Peugeot 207 Confort CAN Bus 
(This canvery easily be adapted to work with other cars, 
but I don't have any other cars, just clone the repo and 
change the packets in the deserialize module)


## Allowing you to do things like:
: Read and write to the confort CAN Bus
: Figure out what packets do what
: Launch without a CLI and use the program as a library
: Use the debug port to test packets


## Usage:
    -b, --buad (default 115200) The baud rate to use
    -p, --port (default debug) The port to use (If -c is set, this will be ignored), eg: debug, /dev/ttyUSB0
    -m, --mode (default 0) [0] Deserialize mode, [1] Listener mode, [2] Inspector mode
    -c Enable the CLI
    -i, --inspect (default '000') The packet to inspect
    -a Automatically find the port (Will ignore -p)
    -d, --debug-packet (default 'none') Provide a packet to use with the debug port


## Modes 
[1] Deserialize mode: 
    This will deserialize the packets and print them to the console, combined together in one big object

[2] Listener mode: 
    This will print out all the packets as they come in, this is useful for figuring out what packets do what

[3] Inspector mode:
    This will print out the packet you want to inspect, this is useful for figuring out what packets do what
    by printing out the bits, hex and text of the packet you want to inspect


## Dependencies
- Linux:
    -pkg-config
    -libudev-dev