spec aptos_framework::coin {
    spec mint {
        pragma opaque;
        let addr = spec_coin_address<CoinType>();
        modifies global<CoinInfo<CoinType>>(addr);
        ensures [abstract] global<CoinInfo<CoinType>>(addr) == old(global<CoinInfo<CoinType>>(addr));
        aborts_if [abstract] false;
        ensures [abstract] result.value == amount;
    }

    spec coin_address {
        pragma opaque;
        ensures [abstract] result == spec_coin_address<CoinType>();
    }

    spec fun spec_coin_address<CoinType>(): address {
        // Abstracted due to the lack of the `type_info` support.
        @0x0
    }
}
