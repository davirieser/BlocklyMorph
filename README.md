
For the following the word "port" refers to a connector of the plugin board, not to a controller pin.

## Problems with shared bus communication

Both I2C and SPI, that should theoretically suit this project because of their shared bus architecture, have fundamental issues when trying to incorporate them.

#### I2C

I2C is a master-slave protocol and provides no support for slaves to initiate communication with the master.
This would be required when a new board is connected.
This would then mean that all devices would have to be masters or the base board would need to periodically query for new devices.
But I2C does not support master to master communication either.
The peoridical query could be done using a "Generall Call" where all new devices would respond.
But this would be trying to fit a square peg into a round hole.

I2C also does not support hot-plugging devices and thus would require additional circuitry.

It also has an non-dynamic addressing scheme which would lead to race conditions and communication collisions.
This could be mitigated by using the same address for all addresses and then checking on a higher layer whether the packet is actually meant for the given device.

I2C also has high baseline power consumption because of the pull-up resistors. 

#### SPI

SPI also is a master-slave protocol and thus would make registration with the base board complicated.
Another problem is that SPI all variations of the bus topology do not work for hot plugging.
- Multidrop configurations would require the blocks to do the routing themselves by setting their neighbours CS.
  The master would select one of its neighbouring blocks and tell it to set the CS lines of one of their neighbouring blocks.
  The master can now address this block and tell it to set the CS line of one of that blocks neighbours.
  This would have to be done recursively for each communication between base and target block.
- Daisy chain configurations cannot ever work since the chain of MISO/MOSI chain cannot be broken.
  One would have to bridge across ports that are not connected, but still enable them to connect using some other way.

#### Other Options 

I2C, SPI and UART are the most widespread communication options on microcontrollers.
Since I2C and SPI are both out, this leaves only peer-to-peer communication using UART.

Here I could only find two distinct variants for communication:
- Parallelism
  - All ports are connected to seperate hardware UART's.
  - This is the easiest way but has significantly higher hardware requirements which are not required as the boards will be in sleep mode for most of the time.
- Time sliced 
  - Only one hardware UART needed for an arbitrary amount of ports.
  - Every block has only one port at a time whose communication lines are actually connected to the hardware UART. 
    Two devices need to synchronize their active ports to start communication. 
    This active port can be switched to another port when communication is finished.
  - This variant is also compatible with the other case if the device with the parallel ports stores messages until they are acknowledged.
    - The non-active ports pull the sending line to high as this is the default state for UART and when they want to communicate they pull the line low which corresponds to the UART start bit.
  - Slightly higher hardware requirements: 1 Demultiplexer in general and 1 resistors, 1 window comparator, 2 transistors, 1 pin more per port.
  - See [PDF](https://github.com/davirieser/BlocklyMorph/blob/dd45b4aaba8524e23da5adf29cdbead551775158/TeX/build/default/default.pdf) 

The first variant could detect whether a block is connected to one of it's port using periodic discovery requests.
But it could also use a window comparator for detecting this which would reduce the computational load of the microcontroller and make it asynchronous. 

