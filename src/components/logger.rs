use crossterm::{execute, style::Print, terminal, ExecutableCommand};
use std::{io::{stdout, Result}, sync::Arc, sync::Mutex};

#[derive(Debug, Clone)]
pub enum LogType {
    Message,
    Warning,
    Error,
}

#[derive(Debug, Clone)]
pub struct Log {
    message: String,
    log_type: LogType,
    display: bool
}

impl Log {
    pub fn new(message: String, log_type: LogType, display: bool ) -> Self {
        Self { message, log_type, display }
    } 
}

#[derive(Debug, Clone)]
pub struct Logger {
    logs: Arc<Mutex<Vec<Log>>>
}

impl Logger {
    pub fn new() -> Self {
        let logs = Arc::new(Mutex::new(vec![]));
        Logger { logs }
    }

    pub fn log(&mut self, log: Log) {
        self.logs.lock().unwrap().push(log);
    }

    pub fn clear(&mut self) {
        self.logs.lock().unwrap().clear();
    }

    pub fn render(&self, mode: OutputMode) {
        match mode {
            OutputMode::Default => output_mode_default(&self),
            OutputMode::Result => output_mode_result(&self)
        }
    }

    pub fn count_log_type(&self, _log_type: LogType) -> usize {
        self.logs.lock().unwrap().iter().filter(|log|
            match _log_type {
                LogType::Message => matches!(log.log_type, LogType::Message),
                LogType::Warning => matches!(log.log_type, LogType::Warning),
                LogType::Error => matches!(log.log_type, LogType::Error)
            })
    .count()
    }
}

fn output_mode_default(logger: &Logger) {
    for log in logger.logs.lock().unwrap().iter() {
        match log.log_type {
            LogType::Message => {
                if log.display == true {
                    execute!(
                        stdout(),
                        Print(format!("\nLog: {}", log.message))
                    ).unwrap();
                }
                
            }
            LogType::Error => {
                if log.display == true {
                    execute!(
                        stdout(),
                        Print(format!("\nError: {}", log.message))
                    ).unwrap();
                }
            }
            LogType::Warning => {
                if log.display == true {
                    execute!(
                        stdout(),
                        Print(format!("\nWarning: {}", log.message))
                    ).unwrap();
                }
            }
        }
    }
}

fn output_mode_result(logger: &Logger) {
    let message_count = logger.count_log_type(LogType::Message);
    let warning_count = logger.count_log_type(LogType::Warning);
    let error_count = logger.count_log_type(LogType::Error);
    execute!(
        stdout(),
        Print(format!("\n[{}] Succeded", message_count)),
        Print(format!("\n[{}] Failed!", warning_count)),
        Print(format!("\n[{}] Errors found!", error_count))
    ).unwrap();
}

pub enum OutputMode {
    Default,
    Result,
}

pub fn logo() {
    let logo = "
    ██████╗ ██╗██████╗ ██████╗ ██████╗ 
    ██╔══██╗██║██╔══██╗██╔══██╗██╔══██╗
    ██████╔╝██║██████╔╝██████╔╝██████╔╝
    ██╔══██╗██║██╔═══╝ ██╔═══╝ ██╔══██╗
    ██║  ██║██║██║     ██║     ██║  ██║
    ╚═╝  ╚═╝╚═╝╚═╝     ╚═╝     ╚═╝  ╚═╝";
    execute!(
        stdout(),
        Print(format!("{}", logo)),
        Print(format!("\nTool created by: Hackerbo\n"))
    ).unwrap();    
}
