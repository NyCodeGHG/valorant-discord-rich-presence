mod lockfile;

#[cfg(not(windows))]
compile_error!("This application is only built for windows because valorant only runs on windows machines.");
