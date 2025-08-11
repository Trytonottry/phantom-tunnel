let seed = mnemonic_to_seed("abuse skull ...", "");
let keys = derive_ed25519(&seed);
let config = reconstruct_config(&keys);