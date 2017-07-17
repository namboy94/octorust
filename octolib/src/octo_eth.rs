/// Ethernet-communication functions.

use octo_types::*;
use octo_structs::*;

/* TODO Figure this out
/// Helper macro to create an IP address out of a 4-tuple.
/// Who cares if this is actually implemented as a macro? :-p
static inline uint32_t IP(uint8_t a, uint8_t b, uint8_t c, uint8_t d) {
	return htonl((a << 24) | (b << 16) | (c << 8) | d);
}
*/

extern {
    #[link_name="eth_set_ip_address"]
    fn __eth_set_ip_address(address: u32) -> i32;

    #[link_name="eth_open"]
    fn __eth_open(channelID: u16, mode: i32) -> eth_channel_t;

    #[link_name="eth_set_transport"]
    fn __eth_set_transport(channel: eth_channel_t, transport: i32);

    #[link_name="eth_close"]
    fn __eth_close(channel: eth_channel_t) -> i32;

    #[link_name="eth_receive"]
    fn __eth_receive(channel: eth_channel_t, buffer: *mut c_void, size: buf_size_t, iLet: *mut simple_ilet) -> i32;

    #[link_name="eth_send"]
    fn __eth_send(channel: eth_channel_t, buffer: *const c_void, size: buf_size_t, iLet: *mut simple_ilet) -> i32;

}

/// Sets the IP address for the Ethernet port.
/// In the guest-layer implementation, this function will always fail.
///
/// # Arguments
///
/// * `address` - IP address. Can be created using the IP macro.
///
/// # Return Value
///
/// 0 on success, -1 on error.
pub fn eth_set_ip_address(address: u32) -> i32 {
    unsafe {
        __eth_set_ip_address(address)
    }
}

/// Opens a one-directional Ethernet communication channel.
///
/// # Arguments
///
/// * `channelID` - Channel ID.
/// * `mode` -      Channel mode (\c ETH_MODE_READ or \c ETH_MODE_WRITE).
///
/// # Return Value
///
/// Descriptor of the opened channel on success, 0 on error. This
/// descriptor is only valid on the tile where eth_open() was called.
pub fn eth_open(channelID: u16, mode: i32) -> eth_channel_t {
    unsafe {
        __eth_open(channelID, mode)
    }
}

/// Changes the data protocol.
///
/// # Arguments
///
/// * `channel` -   Communication channel.
/// * `transport` - Channel transfer type (\c ETH_TRANS_CONTROL or \c ETH_TRANS_CHUNKS).
pub fn eth_set_transport(channel: eth_channel_t, transport: i32) {
    unsafe {
        __eth_set_transport(channel, transport)
    }
}

/// Closes an Ethernet communication channel.
///
/// # Arguments
///
/// * `channel` - Communication channel.
///
/// # Return Value
///
/// 0 on success, a negative value on error.
pub fn eth_close(channel: eth_channel_t) -> i32 {
    unsafe {
        __eth_close(channel)
    }
}

/// Receives data from an Ethernet communication channel.
///
/// # Arguments
///
/// * `channel` - Communication channel.
/// * `buffer` -  Pointer to the destination buffer. The address must be aligned
///               at a word-size boundary.
/// * `size` -    Size of the destination buffer.
/// * `iLet` -    i-Let to be started locally after the data has arrived.
///
/// # Return Value
///
/// 0 on success, a negative value on error.
///
/// # Note
///
///  The destination buffer must be located in TLM. Moreover, on the
///  x86guest platform, it cannot be located on the stack for architectural
///  reasons.
pub fn eth_receive(channel: eth_channel_t, buffer: *mut c_void, size: buf_size_t, iLet: *mut simple_ilet) -> i32 {
    unsafe {
        __eth_receive(channel, buffer, size, iLet)
    }
}

/// Sends data to an Ethernet communication channel.
///
/// # Arguments
///
/// * `channel` - Communication channel.
/// * `buffer` -  Pointer to the source buffer. The address must be aligned at a
///               word-size boundary. The buffer can be reused or freed
///               immediately after eth_send() returns.
/// * `size` -    Length of the source data.
/// * `iLet` -    i-Let to be started locally after the data has been sent.
///
/// # Return Value
///
/// 0 on success, a negative value on error.
///
/// # Note
///
///  The source buffer must be located in TLM.
pub fn eth_send(channel: eth_channel_t, buffer: *const c_void, size: buf_size_t, iLet: *mut simple_ilet) -> i32 {
    unsafe {
        __eth_send(channel, buffer, size, iLet)
    }
}
