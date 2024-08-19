API_VERSION = 0

log_info("cpu-limiter-rs inited")

function load_fas(pid, pkg)
    io.open("/data/adb/cpulimiter_rs/fas_rs_on", "w"):close()
end

function unload_fas(pid, pkg)
    os.remove("/data/adb/cpulimiter_rs/fas_rs_on")
end
