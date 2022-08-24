spec aptos_std::event {
    spec emit_event {
        aborts_if false;
    }
    spec write_to_event_store {
        pragma opaque;
        aborts_if [abstract] false;
    }
}
