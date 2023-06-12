 
# Консольна программа для отримання прогнозу погоди:
Ця програма дозволяє робити отримувати погоду по таких провайдерах:
1. https://www.weatherapi.com/
2. https://openweathermap.org/
3. https://www.accuweather.com/
4. https://www.aerisweather.com/


# Зміст  
1. [Компіляція](#Компіляція)  
2. [Використання](#Використання)  
3. [Приклади](#Приклади)  

# Компіляція
Виконайте наступні команди для компіляції

~~~bash  
  git clone https://github.com/faunel/weather-rs.git
  cd weather-rs
  cargo build --release
~~~

Потім перейдіть в каталог програми
~~~bash 
  cd target/release
~~~

# Використання

~~~bash  
Використання:
  weather-rs [SUBCOMMAND] [ARG]

OPTIONS:
  -h, --help       Печатає довідку з інформацією
  -V, --version    Печатає версію програми

SUBCOMMANDS:
  conf      Змінює ключ API постачальника 
  get       Отримує дані погоди за адресою населеного пункту
  default   Виставляє провайдера за замовчуванням

ARG: (для команд conf, default)
  weatherapi
  openweathermap,
  accuweather,
  aerisweather,

ARG: (для команди get)
  "назва населеного пункту"
~~~

#
При встановленні нового ключа провайдер за замовчуванням не змінюється, а залишається той що був.
В майбутньому планується додати прапорець --default для підкоманди conf
Щоб при зміні ключа даний провайдер відразу встановлявався за замовчуванням
#

# Приклади

Встановити ключ API для провайдера https://www.weatherapi.com/

~~~bash  
  ./weather-rs conf weatherapi 12345 (де 12345 - це API ключ)
~~~

Встановити за замовчуванням провайдера https://www.weatherapi.com/

~~~bash  
  ./weather-rs default weatherapi
~~~

Отримати погоду для провайдера, який встановлений за замовчуванням

~~~bash  
  ./weather-rs get "Київ"
~~~
