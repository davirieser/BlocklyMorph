#![allow(unused)]

pub mod comm;
use comm::{Deserializer, Serializer};

use serde::{Deserialize, Serialize};

type NodeId = u32;
type NodePort = String;
// Sequence number to make sure that messages are executed in the correct order.
type Clock = u32;

#[derive(Serialize, Deserialize)]
pub struct Message {
    sender: NodeId,
    receiver: NodeId,
    // Sequence number to make sure that messages are executed in the correct order.
    clock: Clock,
    #[serde(skip_serializing_if = "Option::is_none")]
    in_reply_to: Option<Clock>,
    #[serde(flatten)]
    payload: Payload,
}

#[derive(Serialize, Deserialize)]
pub enum Payload {
    // Used for acknowledging that a packet was received.
    // Has the `in_reply_to`-field set to the clock value of the message that is being
    // acknowledged.
    Ack,
    // Batch Version of the `Ack`-Variant.
    // The `in_reply_to`-field is set to `None` for this variant.
    BatchAck(Vec<Clock>),
    // Used for notifying the network that you connected.
    // Has sender and receiver NodeId equal to 0.
    // This message will be converted into the `NewNode` variant by the direct neighbour that receives it.
    // Only sent by non-Master nodes.
    Hello(NodeInfo),
    // Used for requesting a NodeId for a newly connected Node.
    // Needs to remember which node and which port this new node is so that it can be sent back
    // since the new node cannot be addressed otherwise.
    NewNode {
        info: NodeInfo,
        connected_on: NodeIdWithPort,
    },
    NewNodeAck(NodeId),
    // Used by neighbouring nodes to tell each on which ports they are connected.
    PortSync(String),
    // Used for setting a block active e.g. when it is the current instruction that is being
    // executed. Should turn on an LED or something similar depending on the block hardware.
    Activate(bool),
    // Used for telling another node that the messages with the following Clocks were not received.
    // The Node should then retransmit these messages.
    // TODO: Could be implemented as Range Types.
    Missed(Vec<Clock>),
    // Used for signalling to the master that a neighbouring node has disconnected.
    Disconnected(NodeId),
    // Generic Message sent from outside the node network.
    // Will be passed to the generic message handler with the corresponding type.
    Generic {
        r#type: String,
        message: String,
    },
}

#[derive(Serialize, Deserialize)]
pub struct Node {
    id: NodeId,
    status: NodeStatus,
    // Used for keeping track of the sequence number
    sync_status: SyncStatus,
    ports: Vec<NodePort>,
}

#[derive(Serialize, Deserialize)]
pub struct NodeInfo {
    ports: Vec<NodePort>,
    capabilities: Vec<Capability>,
}

// Used for addressing newly connected nodes that do not yet have a NodeId.
// Since they cannot be directly addressed, this stores the NodeId of the neighbour that received
// the `Hello` Message and the port that it received it on, making it addressable.
#[derive(Serialize, Deserialize)]
pub struct NodeIdWithPort {
    id: NodeId,
    port: NodePort,
}

#[derive(Serialize, Deserialize)]
pub struct Capability {
    name: String,
    r#type: CapabilityType,
}

#[derive(Serialize, Deserialize)]
pub enum CapabilityType {
    LED,
    Button,
    Potentiometer,
    LCD,
}

#[derive(Serialize, Deserialize)]
pub enum NodeStatus {
    // This is the start state when a node does not yet have a NodeId and thus cannot communicate.
    Startup,
    // Node is connected and can be reached.
    Active,
    // This status can happen if a node informs the master that one of it's neighbours
    // disconnected but this node has other neighbours that did not yet inform the master.
    // It will be in this state until all neighbours report the disconnect.
    LikelyDisconnected,
    // Node was disconnected and cannot be reached.
    Disconnected,
}

#[derive(Serialize, Deserialize)]
pub enum SyncStatus {
    // All messages that have a lower clock than the current one have arrived.
    Synced(Clock),
    // TODO: Could be implemented as Range Types.
    Missing(Vec<Clock>),
}

#[cfg(feature = "base")]
fn main() {
    println!("Hello from base block!");
}

#[cfg(not(feature = "base"))]
fn main() {
    println!("Hello from outer block!");
}
