/// Ethernet-communication functions.

use octo_types::*;

pub type eth_channel_t = *mut c_void;

enum ETH_MODE {
	ETH_MODE_READ  = 0x1,
	ETH_MODE_WRITE = 0x2
}

enum ETH_TRANS {
	ETH_TRANS_CHUNKS  = 1,
	ETH_TRANS_CONTROL = 2
}

/* TODO Figure this out
/// Helper macro to create an IP address out of a 4-tuple.
/// Who cares if this is actually implemented as a macro? :-p
static inline uint32_t IP(uint8_t a, uint8_t b, uint8_t c, uint8_t d) {
	return htonl((a << 24) | (b << 16) | (c << 8) | d);
}
*/