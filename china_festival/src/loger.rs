use dotenv::dotenv;
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

pub fn init_logs() {
    use std::fs::OpenOptions;

    // Настройка dotenv для переменных окружения
    dotenv().ok();

    // Задаём значение по умолчанию для RUST_LOG, если оно не установлено
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "debug");
    }

    // Открытие или создание файла для логов
    let log_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("log.log")
        .expect("Failed to open or create log.log");

    // Инициализация логгера
    Builder::new()
        .format(move |buf, record| {
            // Форматируем сообщение для логов
            writeln!(
                buf,
                "[{}] {}: {}",
                record.level(),
                record.target(),
                record.args()
            )
        })
        .target(env_logger::Target::Pipe(Box::new(log_file))) // Устанавливаем файл как цель
        .filter(None, LevelFilter::Debug) // Устанавливаем уровень логирования
        .init();

    // Перехват паник
    std::panic::set_hook(Box::new(|info| {
        log::error!("Application panicked: {}", info);
    }));
}
