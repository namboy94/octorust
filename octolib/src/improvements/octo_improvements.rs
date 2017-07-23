/// This file contains improvements over the raw C function wrappers
/// Author: Hermann Krumrey 2017, KIT, Matriculation Number 1789804

use octo_types::*;
use octo_structs::*;
use octo_proxy_claim::*;

/// Initializes a proxy_invade without needing to supply an uninitialized invade_future struct
///
/// # Arguments
///
/// * `tile_id` - tile id of the tile you want to invade
/// * `quantity` - number of PEs you want to aquire
///
/// # Return Value
///
/// A tuple consisting of:
///     1. The invade_future struct initialized here
///     2. The result of the proxy_invade call
pub fn initialize_proxy_invade(tile_id: tile_id_t, quantity: pe_quantity_t) -> (invade_future, bool) {

	// Initialize Struct
    let mut future = invade_future { padding: [0; 64] };

    let result = proxy_invade(tile_id, &mut future, quantity) == 0;
	return (future, result);
}
