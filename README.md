
## API Interface

### Methods

#### IDE to Blocks

- getBlockGraph() : BlockGraph
- setBlockDebugActive(block: BlockId) : boolean
  - Returns whether the block was set to active.
  - May fail if the block has disconnected in the meantime.
- sendGenericMessage(block: BlockId, message: any) : boolean
  - Returns whether the message was sent successfully.
  - May fail if the block has disconnected in the meantime.

#### Blocks to IDE

- callbackGraphChanged()
- sendGenericMessage(from: BlockId, message: any)

#### Types

##### Block Graph

```ts
type BlockId = number | string;
type BlockPortId = number | string;

type BlockGraph = {
    blocks: Block[],
    neighbour_graph: [BlockPort, BlockPort][],
}
type Block = {
    id: BlockId,
    ports: PortId[],
    debug_active: boolean,
    /* Specification of what this block represents e.g. If, Else, Loop, Variable, Print, etc. */
    block_spec: string,
    /* Any values associated with this block */
    block_values: { [string] : any },
}
type BlockSpecification = {
    color: Color,
    type: string,
    has_next_statement: boolean,
    has_previous_statement: boolean,
    inputs: BlockInputSpecification[],
    output_type: string | null,
    tooltip?: string,
    helpUrl?: string,
}
type BlockInputSpecification = {
    name: string, 
    type: string, 
}
enum BlockType {
    Statement,
    Expression,
}
type BlockPort = {
    block: BlockId,
    port: PortId,
}
```

