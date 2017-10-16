/// Author: Hermann Krumrey <hermann@krumreyh.com> 2017
/// Karlsruher Institut für Technologie, Matriculation number 1789804
/// This fle is based on the IRTSS octo_pull_dma.h file

use octo_types::*;
use octo_structs::*;

extern {

    /// Setup a PullDMA transfer: DMA descriptor is forwarded to source tile
    /// to setup a PushDMA
    /// This function forwards a DMA descriptor to the source Tile of the transfer
    /// and then does a push DMA transfer on the source tile.
    /// On LEON, since DMA descriptors are forwarded and then effectively a push DMA
    /// is performed, \b remote_src, has to be in local address format (0x80000000
    /// style addresses) and \b local_dst, although pointing to local tile, has to
    /// be in global address format (0x40000000 style addresses)
    ///
    /// # Arguments
    ///
    /// * `claim` - DispatchClaim/ProxyClaim describing dispatchable resources on the remote source tile
    /// * `remote_src` - Pointer into TLM of remote tile in local address format
    /// * `local_dst` - Pointer into local TLM in global address format
    /// * `size` - size of the remote/local buffers in bytes
    /// * `src_iLet` - ilet that gets submitted at the source side of the transfer (local tile)
    ///                once the DMA operation is finished
    /// * `dst_iLet` - ilet that gets submitted at the destination side of the transfer (remote tile)
    ///                once the DMA operation is finished
    ///
    /// # Return Value
    ///
    /// Returns 0 on success; On error -1 is returned: Checks consistency
    ///
    /// between local_dst and TID of tile function is executed on
    pub fn dispatch_claim_pull_dma(claim: dispatch_claim_t, remote_src: *const c_void, local_dst: *mut c_void, size: buf_size_t, src_iLet: *mut simple_ilet, dst_iLet: *mut simple_ilet) -> i32;

    /// PullDMA requests also can be specified by using a proxy_claim_t
    pub fn proxy_claim_pull_dma(claim: proxy_claim_t, remote_src: *const c_void, local_dst: *mut c_void, size: buf_size_t, src_iLet: *mut simple_ilet, dst_iLet: *mut simple_ilet) -> i32;

}
