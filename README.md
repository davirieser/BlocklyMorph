
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

## Problems with peer-to-peer communication

The problem with peer-to-peer communication is that it would require one standardized communication interface (probably UART) per port of the block.
It would also require message routing by the blocks themselves by implementing either:
- a routing protocol similar to OSPF (which I have already implemented in a different project)
- predetermining the complete route in the base block

#### Communication Options for peer-to-peer

- Parallel port-based
  - 4 pins
    - VCC
    - GND
    - UART TX
    - UART RX
  - All ports are connected to seperate hardware UART's.
  - This is the easiest way but has higher hardware requirements that are in sleep mode for most of the time.
- Interrupt based Multiplexing
  - 4 pins
    - VCC
    - GND
    - UART TX
    - UART RX
  - See [PDF]() 
