# Сборка проекта

##### Система

* Ubuntu 16.04

##### Зависимости

+ [Rust][1] > 1.38.0
+ [PostgreSQL][2] > 10.5
+ [DieselCLI][3]

##### Команда

1. Конфигурация базы данных
    1. Установить переменную окружения DATABASE_URL:
        Пример: `DATABASE_URL=postgres://username:password@{host}/{database}`
    2. В папке проекта выполнить команду: `disel migrations run`
        
2. Запуск билда:
    1. Перейти в папку проекта
    2. Выполнить команду: `cargo build`

3. Сформировать `Config.toml` по примеру `Config.toml.example`
4. Запустить журейку одной из следующих команд:
    `/target/debug/rptime -c <path_to_config>`
    `/target/debug/rptime --config <path_to_config>`

[1]: https://www.rust-lang.org/tools/install "Rust"
[2]: https://www.postgresql.org/download/ "PostreSQL"
[3]: https://github.com/diesel-rs/diesel/tree/v1.3.0/diesel_cli "DieselCLI"