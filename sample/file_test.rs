use versalogrs::NewVersaLog;

fn main() {
    let logger = NewVersaLog(
        "file",
        false,
        false,
        "VersaLog",
        false,
        false,
        false,
        vec![],
        false,
    );

    logger.Info("info", &[]);
    logger.Error("error", &[]);
    logger.Warning("warning.", &[]);
    logger.Debug("debug", &[]);
    logger.Critical("critical", &[]);
}
