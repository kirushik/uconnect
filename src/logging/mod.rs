use flexi_logger;

pub fn enable_logging(enable_debug: bool) {
  let log_level = if enable_debug {
    Some("uconnect=debug".into())
  } else {
    Some("uconnect=warn".into())
  };
  flexi_logger::init(flexi_logger::LogConfig::new(), log_level).unwrap();
}
