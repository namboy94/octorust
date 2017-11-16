/*
Copyright 2017 Hermann Krumrey <hermann@krumreyh.com>

This file is part of octolib.

octolib is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

octolib is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with octolib.  If not, see <http://www.gnu.org/licenses/>.
*/

/// Author: Hermann Krumrey <hermann@krumreyh.com> 2017
/// Karlsruher Institut für Technologie, Matriculation number 1789804
/// This fle is based on the IRTSS octo_eth.h file
///
/// Ethernet-communication functions.

use octo_types::*;
use octo_structs::*;

/* TODO
/// Helper macro to create an IP address out of a 4-tuple.
/// Who cares if this is actually implemented as a macro? :-p
static inline uint32_t IP(uint8_t a, uint8_t b, uint8_t c, uint8_t d) {
	return htonl((a << 24) | (b << 16) | (c << 8) | d);
}
*/

extern {

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
    pub fn eth_set_ip_address(address: u32) -> i32;

    /// Opens a one-directional Ethernet communication channel.
    ///
    /// # Arguments
    ///
    /// * `channelID` - Channel ID.
    /// * `mode` -      Channel mode (ETH_MODE_READ or ETH_MODE_WRITE).
    ///
    /// # Return Value
    ///
    /// Descriptor of the opened channel on success, 0 on error. This
    /// descriptor is only valid on the tile where eth_open() was called.
    pub fn eth_open(channelID: u16, mode: i32) -> eth_channel_t;

    /// Changes the data protocol.
    ///
    /// # Arguments
    ///
    /// * `channel` -   Communication channel.
    /// * `transport` - Channel transfer type (ETH_TRANS_CONTROL or ETH_TRANS_CHUNKS).
    pub fn eth_set_transport(channel: eth_channel_t, transport: i32);

    /// Closes an Ethernet communication channel.
    ///
    /// # Arguments
    ///
    /// * `channel` - Communication channel.
    ///
    /// # Return Value
    ///
    /// 0 on success, a negative value on error.
    pub fn eth_close(channel: eth_channel_t) -> i32;

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
    /// The destination buffer must be located in TLM. Moreover, on the
    /// x86guest platform, it cannot be located on the stack for architectural
    /// reasons.
    pub fn eth_receive(channel: eth_channel_t, buffer: *mut c_void, size: buf_size_t, iLet: *mut simple_ilet) -> i32;

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
    /// The source buffer must be located in TLM.
    pub fn eth_send(channel: eth_channel_t, buffer: *const c_void, size: buf_size_t, iLet: *mut simple_ilet) -> i32;

}
