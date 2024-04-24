
For the following the word "port" refers to a connector of the plugin board, not to a controller pin.

### Problems with shared bus communication

Both I2C and SPI, that should theoretically suit this project because of their shared bus architecture, have fundamental issues when trying to incorporate them.

#### I2C

Because I2C is a master-slave protocol it does not provide any mechanism for a newly connected Block to register with the base board.
It also has an non-dynamic addressing scheme which would lead to race conditions and communication collisions.
I2C also has high baseline power consumption because of the pull-up resistors. 

#### SPI

SPI also is a master-slave protocol and thus would make registration with the base board complicated.
Another problem is that SPI all variations of the bus topology do not work for hot plugging.
- Multidrop configurations would require the blocks to do the routing themselves by setting their neighbours CS.
  This would have to be done recursively for each communication between base and target block.
- Daisy chain configurations cannot ever work since the chain of MISO/MOSI chain cannot be broken.
  One would have to bridge across ports that are not connected, but still enable them to connect using some other way.

### Problems with peer-to-peer communication

The problem with peer-to-peer communication is that it would require one standardized communication interface (probably USART) per port of the block.
It would also require message routing by the blocks themselves by implementing either:
- a routing protocol similar to OSPF (which I have already implemented in a different project)
- predetermining the complete route in the base block

#### Communication Options

- Interrupt based Multiplexing
  - 5 pins
    - VCC
    - GND
    - USART TX
    - USART RX
    - Interrupt
  - The port to which the USART RX and TX are connected to, is dynamically selected.
   - Whenever an interrupt comes in the given port is selected, unless connection with another port is currently active.
   - Port selection can be done using FET transistors and a 2 to 4 multiplexer.
  - Problems:
    - How to recognize when new Board is connected?
      - At startup an interrupt is generated for all directly connected boards, since one has to be connected otherwise it would not have power.
    - How can you recognize whether there is an interrupt on one side, or on both sides at the same time?
      - Some sort of analog circuit?
- Location Beacon Multiplexing
  - 5 pins
    - VCC
    - GND
    - I2C SDA
    - I2C SCL
    - Location Beacon
  - Since the position only has to be determined once, when a board is connected it can try and register itself using I2C.
    While doing this it pulls up the Location Beacons of all it's ports.
  - Problems:
    - During registration how
    - Dynamic I2C Addresses are pretty much needed for this. 
- Parallel port-based
  - 4 pins
    - VCC
    - GND
    - USART TX
    - USART RX
  - All ports are connected to seperate hardware USART's.
  - This is the easiest way but has higher hardware requirements that are in sleep mode for most of the time.
 
