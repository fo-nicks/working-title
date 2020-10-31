/// The peer module defines traits and structures relevant to Peers.
/// As the name suggegts, Peers are equal participants in the application -
/// any peer can make a direct request of any other peer.
///
/// Peers communicate with each other over some bidirectional communication 
/// channel.  The exact form of this channel is left abstract, but possible
/// examples are TCP, Secure Sockets, SSH, HTTP/S, etc.  Whatever the channel,
/// it is presumed to be generally reliable, though Peers are not expected to
/// be always present.  
/// 
/// In the current design, Peers communicate using a mostly stateless protocol.  
/// At the base level of the protocol are 'commands'.  Commands are a way of
/// passing (structured) data in a single direction.  The underlying transport 
/// is expected to reliably deliver data.
/// 
/// Commands consist of an opcode, indicating which command is to be executed,
/// a command ID, which will be used to identify the command over a certain
/// bound, a time to live (TTL) and the arguments (or data) specific to that 
/// command.  The identifier is also used to tie requests and responses together.  
/// The core idea is that any peer is able to send any command at any time to 
/// any other peer.  Thus, at its core, the protocol is not stateful and asyncronous.
/// The commands themselves are design to account for out-of-order execution and
/// to be idempotent.
/// 
/// Within the commands themselves is the ability to track outstanding commands etc.
/// this is expanded below, in the explanation for the commands.