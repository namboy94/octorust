/**
 * \file  octo_eth.h
 * \brief Ethernet-communication functions.
 */

#ifndef _OCTO_ETH_H_
#define _OCTO_ETH_H_

#include "octo_ilet.h"
#include <arpa/inet.h>

typedef void *eth_channel_t;

enum {
	ETH_MODE_READ  = 0x1,
	ETH_MODE_WRITE = 0x2
};

enum {
	ETH_TRANS_CHUNKS  = 1,
	ETH_TRANS_CONTROL = 2
};

#ifdef __cplusplus
extern "C" {
#endif

/** Helper macro to create an IP address out of a 4-tuple. */
/* Who cares if this is actually implemented as a macro? :-p */
static inline uint32_t IP(uint8_t a, uint8_t b, uint8_t c, uint8_t d) {
	return htonl((a << 24) | (b << 16) | (c << 8) | d);
}

/**
 * \brief Sets the IP address for the Ethernet port.
 *
 * In the guest-layer implementation, this function will always fail.
 *
 * \param address IP address. Can be created using the IP macro.
 * \return 0 on success, -1 on error.
 */
int eth_set_ip_address(uint32_t address);

/**
 * \brief Opens a one-directional Ethernet communication channel.
 * \param channelID Channel ID.
 * \param mode      Channel mode (\c ETH_MODE_READ or \c ETH_MODE_WRITE).
 * \return Descriptor of the opened channel on success, 0 on error. This
 *         descriptor is only valid on the tile where eth_open() was called.
 */
eth_channel_t eth_open(uint16_t channelID, int mode);

/**
 * \brief Changes the data protocol.
 * \param channel   Communication channel.
 * \param transport Channel transfer type (\c ETH_TRANS_CONTROL or \c ETH_TRANS_CHUNKS).
 */
void eth_set_transport(eth_channel_t channel, int transport);

/**
 * \brief Closes an Ethernet communication channel.
 * \param channel Communication channel.
 * \return 0 on success, a negative value on error.
 */
int eth_close(eth_channel_t channel);

/**
 * \brief Receives data from an Ethernet communication channel.
 * \param channel Communication channel.
 * \param buffer  Pointer to the destination buffer. The address must be aligned
 *                at a word-size boundary.
 * \param size    Size of the destination buffer.
 * \param iLet    i-Let to be started locally after the data has arrived.
 * \return 0 on success, a negative value on error.
 * \note The destination buffer must be located in TLM. Moreover, on the
 *       x86guest platform, it cannot be located on the stack for architectural
 *       reasons.
 */
int eth_receive(eth_channel_t channel, void* buffer, buf_size_t size,
                simple_ilet* iLet);

/**
 * \brief Sends data to an Ethernet communication channel.
 * \param channel Communication channel.
 * \param buffer  Pointer to the source buffer. The address must be aligned at a
 *                word-size boundary. The buffer can be reused or freed
 *                immediately after eth_send() returns.
 * \param size    Length of the source data.
 * \param iLet    i-Let to be started locally after the data has been sent.
 * \return 0 on success, a negative value on error.
 * \note The source buffer must be located in TLM.
 */
int eth_send(eth_channel_t channel, const void* buffer, buf_size_t size,
             simple_ilet* iLet);

#ifdef __cplusplus
}
#endif

#endif /* _OCTO_ETH_H_ */