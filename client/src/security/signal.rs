fn self_destruct() {
    secure_wipe(KEYS_FILE);
    secure_wipe(CONFIG_FILE);
    secure_wipe("/tmp/phantom*");
    kill_process();
    // Перезапись памяти мусором
    overwrite_memory_with_noise();
    // Отправка сигнала "я пойман" в сеть (необязательно)
    send_distress_signal();
    std::process::exit(0);
}