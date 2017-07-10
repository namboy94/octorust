/**
 * \brief Setup a PullDMA transfer: DMA descriptor is forwarded to source tile
 * to setup a PushDMA
 *
 * This function forwards a DMA descriptor to the source Tile of the transfer
 * and then does a push DMA transfer on the source tile.
 *
 * On LEON, since DMA descriptors are forwarded and then effectively a push DMA
 * is performed, \b remote_src, has to be in local address format (0x80000000
 * style addresses) and \b local_dst, although pointing to local tile, has to
 * be in global address format (0x40000000 style addresses)
 *
 * \param claim DispatchClaim/ProxyClaim describing dispatchable resources on the remote source tile
 * \param remote_src Pointer into TLM of remote tile in local address format
 * \param local_dst Pointer into local TLM in global address format
 * \param size size of the remote/local buffers in bytes
 * \param src_iLet ilet that gets submitted at the source side of the transfer (local tile) once the DMA operation is finished
 * \param dst_iLet ilet that gets submitted at the destination side of the transfer (remote tile) once the DMA operation is finished
 * \return Returns 0 on success; On error -1 is returned: Checks consistency
 * between \b local_dst and TID of tile function is executed on
 */
int dispatch_claim_pull_dma(dispatch_claim_t claim, const void* remote_src, void* local_dst, buf_size_t size, simple_ilet* src_iLet, simple_ilet* dst_iLet);

/** 
 * PullDMA requests also can be specified by using a proxy_claim_t
 * \copydoc dispatch_claim_pull_dma
 */
int proxy_claim_pull_dma(proxy_claim_t claim, const void* remote_src, void* local_dst, buf_size_t size, simple_ilet* src_iLet, simple_ilet* dst_iLet);
