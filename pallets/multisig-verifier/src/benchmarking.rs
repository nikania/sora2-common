// This file is part of the SORA network and Polkaswap app.

// Copyright (c) 2020, 2021, Polka Biome Ltd. All rights reserved.
// SPDX-License-Identifier: BSD-4-Clause

// Redistribution and use in source and binary forms, with or without modification,
// are permitted provided that the following conditions are met:

// Redistributions of source code must retain the above copyright notice, this list
// of conditions and the following disclaimer.
// Redistributions in binary form must reproduce the above copyright notice, this
// list of conditions and the following disclaimer in the documentation and/or other
// materials provided with the distribution.
//
// All advertising materials mentioning features or use of this software must display
// the following acknowledgement: This product includes software developed by Polka Biome
// Ltd., SORA, and Polkaswap.
//
// Neither the name of the Polka Biome Ltd. nor the names of its contributors may be used
// to endorse or promote products derived from this software without specific prior written permission.

// THIS SOFTWARE IS PROVIDED BY Polka Biome Ltd. AS IS AND ANY EXPRESS OR IMPLIED WARRANTIES,
// INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL Polka Biome Ltd. BE LIABLE FOR ANY
// DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING,
// BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS;
// OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
// STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::*;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::{RawOrigin, self};
use sp_core::{ecdsa, Pair};
use crate::Pallet as MultisigVerifier;
use bridge_types::EVMChainId;

fn initial_keys() -> Vec<ecdsa::Public> {
    vec![
        ecdsa::Pair::generate_with_phrase(Some("Alice")).0.into(),
        ecdsa::Pair::generate_with_phrase(Some("Bob")).0.into(),
        ecdsa::Pair::generate_with_phrase(Some("Carol")).0.into(),
    ]
}



fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

benchmarks! {

    initialize_evm {
        let network_id = bridge_types::GenericNetworkId::EVM(EVMChainId::from(1));
        let keys = initial_keys();
    }: initialize(RawOrigin::Root, network_id, keys)
    verify {
        assert_last_event::<T>(Event::NetworkInitialized(network_id).into())
    }

    // add_peer {
    //     /* code to set the initial state */
    // }: _(origin, key)
    // verify {
    //     /* optional verification */
    // }

    // remove_peer {
    //     /* code to set the initial state */
    // }: _(origin, key)
    // verify {
    //     /* optional verification */
    // }

    impl_benchmark_test_suite!(MultisigVerifier, crate::mock::new_test_ext(), mock::Test)
}