# Функционал
### Регистрация
Frontend->Backend
 - email
 - team_name
 - country
 - university
 - Captcha

Backend
 - Генерация токена
 - Запись в таблицу team_info

Backend->Frontend
 - Результат

### Аутентификация
Frontend->Backend
 - team_name
 - token
 - Captcha

Backend
 - Сравненение хэша

Backend->Frontend
 - Результат аутентификации

### Карта
На карте располагаются точки с тасками
При наведении на точку повляются окна с краткой инфой о таске
Окна с краткой инфой о таске подгружаются сразу
##### Окно с краткой инфой о таске
 - task_name
 - points
 - keys_reward
 - keys_condition
 - tags
 - Кнопка «Открыть описание»

Кнопка «Открыть описание» открывает окно с описанием таска
##### Окно с описанием таска
 - task_name
 - points
 - keys_reward
 - keys_condition
 - tags
 - description
 - picture
 - Поле для ввода флага
 - Кнопка «Отправить»
 - Кнопка «Закрыть»
 - task_id

##### Кнопка «Отправить»
Frontend->Backend
 - task_id
 - flag
 - team_id

Backend
 - Проверяет флаг
 - Если ок, то: в team_game записывает task_id, points и изменяет keys_owned, socket update
 - Если нет: шлет нахер

Backend->Frontend
 - Результат

##### Кнопка «Dream Team»
 - Название
 - Аватарка
 - keys_owned

Frontend->Backend
 - team id

Backend->Frontend
 - team_name
 - team_avatar
 - keys_owned

##### Кнопка «Скорборд»
 - Скорборд

Frontend->Backend
 - Запрос на скорборд

Backend->Frontend
 - Скорборд

##### Кнопка «Notifications»
 - Уведомления

Backend->Frontend
 - Уведомления(update socket)

Frontend
 - Фильтрация

### Взаимодействие с бэкендом
 - Отправка уведомлений
 - Открытие таски
 - Управление работой

# БД
### Таблица team_info
 - team_id
 - email
 - name
 - country
 - university
 - hash(token)

### Таблица team_game
 - team_id
 - keys_owned
 - points

### Таблица completed
 - team_id
 - task_id
 - time

### Таблица tasks
 - task_id
 - task_name
 - description
 - flag
 - points
 - keys_reward
 - keys_condition
 - coords

### Таблица game
 - start_time
 - stop_time
 
