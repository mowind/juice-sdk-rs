use fixed_hash::*;

use impl_codec::impl_fixed_hash_codec;
use impl_rlp::impl_fixed_hash_rlp;
use impl_serde::impl_fixed_hash_serde;

construct_fixed_hash! { pub struct H768(96); }
impl_fixed_hash_rlp!(H768, 96);
impl_fixed_hash_serde!(H768, 96);
impl_fixed_hash_codec!(H768, 96);
