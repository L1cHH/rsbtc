mod sha256;
mod types;
mod util;
mod crypto;

use uint::construct_uint;

construct_uint! {
    pub struct U256(4);
}