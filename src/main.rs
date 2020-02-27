use std::{thread, time};
use text_io::read;

struct Words {
    _1: String,
    _2: String,
    _3: String,
    _4: String,
    _5: String,
    _6: String,
    _7: String,
    _8: String,
    _1_q: String,
    _2_q: String,
    _3_q: String,
    _4_q: String,
    _5_q: String,
    _6_q: String,
    _7_q: String,
    _8_q: String,
}

fn main() {
    let _1 = String::from("в");
    let _2 = String::from("е");
    let _3 = String::from("н");
    let _4 = String::from("т");
    let _5 = String::from("и");
    let _6 = String::from("л");
    let _7 = String::from("я");
    let _8 = String::from("ц");
    let _1_q = String::from("1");
    let _2_q = String::from("2");
    let _3_q = String::from("3");
    let _4_q = String::from("4");
    let _5_q = String::from("5");
    let _6_q = String::from("6");
    let _7_q = String::from("7");
    let _8_q = String::from("8");
    let mut game_words = Words {
        _1,
        _2,
        _3,
        _4,
        _5,
        _6,
        _7,
        _8,
        _1_q,
        _2_q,
        _3_q,
        _4_q,
        _5_q,
        _6_q,
        _7_q,
        _8_q,
    };
    println!("Привет 👋\nДавай дружить?😜");
    loop {
        let answer_friend: String = read!();
        match answer_friend.to_lowercase().as_str() {
            "ok" => break,
            "ок" => break,
            "давай" => break,
            "ну давай" => break,
            "нудавай" => break,
            "окей" => break,
            "хорошо" => break,
            "нет" => println!("И тебе совсем не хочется получить подарок на День рождения? 😥"),
            _ => println!("Извини, но я не понимаю твоего ответа. 😥"),
        }
    }
    think(1);
    println!("Отлиииично.\nМеня зовут Татарин бот, а тебя??🤔");
    let name: String = read!();
    think(1);
    println!("👋 👋 👋 Здравствуй, {}!!! Приятно познакомиться.", name);
    think(2);
    println!("Я совершенно случайно узнал, что у тебя сегодня день рождения. Это правда?");
    let yes_no: String = read!();
    match yes_no.to_lowercase().as_str() {
        "no" => {
            think(1);
            println!("Тогда хорошего дня. До новых встреч.");
            return;
        }
        "нет" => {
            think(1);
            println!("Тогда хорошего дня. До новых встреч.");
            return;
        }
        "неа" => {
            think(1);
            println!("Тогда хорошего дня. До новых встреч.");
            return;
        }
        _ => {
            think(1);
            println!("Как я удачно зашел.");
        }
    }
    think(2);
    println!("Получается, с меня подарочек");
    think(2);
    println!("Такого хватит??? --->  🎁 ?");
    think(2);
    println!("Боюсь нет");
    think(2);
    println!("Хмммм....");
    think(3);
    println!("А Такой...?");
    think(1);
    println!(
        "
                              !     !     !
   (          (    *         |V|   |V|   |V|        )   *   )       (
    )   *      )             | |   | |   | |        (       (   *    )
   (          (           (*******************)    *       *    )    *
   (     (    (           (    *         *    )               )    (
    )   * )    )          (   \\|/       \\|/   )         *    (      )
   (     (     *          (<<<<<<<<<*>>>>>>>>>)               )    (
    )     )        ((*******************************))       (  *   )
   (     (   *     ((         HAPPY BIRTHDAY!!!!    ))      * )    (
    ) *   )        ((   *    *   *    *    *    *   ))   *   (      )
   (     (         ((  \\|/  \\|/ \\|/  \\|/  \\|/  \\|/  ))        )    (
   *)     )        ((^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^))       (      )
   (     (   (**********************************************) )  * ("
    );
    think(2);
    println!("Нееее, то же не то");
    think(3);
    println!("Придуууумал!!!! У меня есть для тебя отличный подарок.");
    think(2);
    println!("Но что бы его получить, придется.....");
    think(2);
    println!("Попотеть...");
    think(2);
    println!("if you know what i mean");
    think(3);
    println!("Давай начнем.");
    rules(&mut game_words);
    loop {
        if check_words(&mut game_words) {
            break;
        }
        let next_char_number: String = read!();
        if !next_char_number.parse::<f64>().is_ok() {
            println!("Надо писать циферку, а не буковку :)")
        } else {
            match next_char_number.parse::<u8>().unwrap() {
                1 => {
                    _1_word(&mut game_words);
                    print_words(&mut game_words);
                }
                2 => {
                    _2_word(&mut game_words);
                    print_words(&mut game_words);
                }
                3 => {
                    _3_word(&mut game_words);
                    print_words(&mut game_words);
                }
                4 => {
                    _4_word(&mut game_words);
                    print_words(&mut game_words);
                }
                5 => {
                    _5_word(&mut game_words);
                    print_words(&mut game_words);
                }
                6 => {
                    _6_word(&mut game_words);
                    print_words(&mut game_words);
                }
                7 => {
                    _7_word(&mut game_words);
                    print_words(&mut game_words);
                }
                8 => {
                    _8_word(&mut game_words);
                    print_words(&mut game_words);
                }
                0 => {
                    rules(&mut game_words);
                }
                _ => {
                    println!(
                        "Кажется номера такой буквы нету.\nЕсли хочешь перечитать правила, введи 0"
                    );
                    print_words(&mut game_words);
                }
            }
        }
    }
    think(2);
    println!(
        "{}, ты молодец. Наше слово ВЕНТИЛЯЦИЯ.\nЭто было не просто, но еще не конец",
        name
    );
    think(2);
    println!("Давай подумаем вместе, сколько вентиляций в квартире (кондиционер не считаем)????");
    loop {
        let count_vent = read!();
        match count_vent {
            3 => {
                println!("Отлиииично. Мне кажется, тебе нужно проверить каждую (только не ломай) и ты найдешь свой подарок. 🎥 ");
                break;
            }
            _ => println!("Подумай еще"),
        }
    }
    println!("Хорошего дня рождения, с любовью, твой татарин.😘 😘 😘");
}

fn think(sec: u64) {
    let time_to_answer = time::Duration::from_secs(sec);
    thread::sleep(time_to_answer);
}

fn rules(game_words: &mut Words) {
    think(3);
    println!("###################################################");
    println!("#######______________________________________######");
    println!("#######_____________ПРАВИЛА__________________######");
    println!("#######______________________________________######");
    println!("###################################################");
    think(3);
    println!("Перед тобой слово из 10 букв");
    println!(
        "[{}][{}][{}][{}][{}][{}][{}][{}][{}][{}]",
        game_words._1_q,
        game_words._2_q,
        game_words._3_q,
        game_words._4_q,
        game_words._5_q,
        game_words._6_q,
        game_words._7_q,
        game_words._8_q,
        game_words._7_q,
        game_words._8_q,
    );
    think(4);
    println!("Что бы получить подарок, нужно отгадать это слово");
    think(3);
    println!();
    println!("Каждая буква таит в себе загадку,\nвводи номер буквы, которую ты хочешь отгадать.\nЖелаю удачи.");
    think(5);
    println!();
    println!("__________________________________________________________");
    println!("P.S. Нумерация не ошибочная) Буквы в слове повторяются.\nОтгадаешь одну из букв, откроются все такие же,\nпрям как в Поле Чудес");
    println!("__________________________________________________________");
    think(5);
    println!("Поехали. Я Жду номер буквы 👇");
}

fn _1_word(game_words: &mut Words) {
    let sudoku_url = "https://sudoku.bestcrosswords.ru/generator?id=7368&level=3";
    let sudoku_game = "\n
---------------------------------------------------
    СУДОКУ
    -------------------------------
    |[ ][ ][6]|[ ][4][ ]|[ ][ ][1]|
    |[ ][5][9]|[ ][ ][6]|[ ][3][ ]|
    |[ ][ ][ ]|[ ][9][7]|[5][ ][8]|
    |---------|---------|---------|
    |[ ][ ][ ]|[ ][ ][1]|[8][7][ ]|
    |[ ][4][1]|[ ][7][ ]|[2][ ][ ]|
    |[ ][8][ ]|[ ][ ][ ]|[3][ ][4]|
    |---------|---------|---------|
    |[ ][ ][3]|[1][8][4]|[7][ ][ ]|
    |[5][ ][ ]|[ ][ ][3]|[ ][4][2]|
    |[ ][7][ ]|[ ][2][ ]|[ ][8][ ]|
    -------------------------------
----------------------------------------------------
    \n";
    println!("{}", sudoku_game);
    think(2);
    println!("Перед тобой игра судоку.");
    think(2);
    println!("Задача очень простая. Найди сумму всех чисел по диагонали (с верхего левого до нижнего правого угла)\nи напиши ответ мне. Можно пользоваться листочком.");
    let mut count = 0;
    loop {
        let answer = read!();
        match answer {
            41 => {
                println!("✅ Умница. Все верно.");
                println!();
                game_words._1_q = String::from(game_words._1.as_str());
                think(3);
                println!(
                    "Можно приступать к следующей букве. Напиши номер буквы 👇 и отгадай загадку."
                );
                break;
            }
            _ => {
                println!("❌ Подумай еще");
                count += 1;
                if count > 3 {
                    println!("Даю тебе подсказку. {}", sudoku_url);
                }
            }
        }
    }
}
fn _2_word(game_words: &mut Words) {
    println!("Задача очень простая, отгадай загадку.");
    think(3);
    println!("Какого числа день рождения у отца твоего татарина?");
    let mut count = 0;
    loop {
        let answer = read!();
        match answer {
            26 => {
                println!("✅ Умница. Все верно.");
                think(3);
                game_words._2_q = String::from(game_words._2.as_str());
                println!(
                    "Можно приступать к следующей букве. Напиши номер буквы 👇 и отгадай загадку."
                );
                break;
            }
            _ => {
                println!("❌ Подумай еще");
                count += 1;
                if count == 3 {
                    println!("Даю тебе подсказку, это июль месяц");
                } else if count == 5 {
                    println!("Даю тебе подсказку, это число можно получить из числа ДР татарина");
                } else if count == 6 {
                    println!("Даю тебе подсказку, 24 сентября + два месяца назад");
                }
            }
        }
    }
}
fn _3_word(game_words: &mut Words) {
    println!("Задача очень простая, отгадай загадку.");
    println!("🧑‍🚀 Что нельзя сделать в космосе?");
    let mut count = 0;
    loop {
        let answer: String = read!();
        match answer.to_lowercase().as_str() {
            "повеситься" => {
                println!("✅ Умница. Все верно.");
                game_words._3_q = String::from(game_words._3.as_str());
                println!(
                    "Можно приступать к следующей букве. Напиши номер буквы 👇 и отгадай загадку."
                );
                break;
            }
            _ => {
                println!("❌ Подумай еще");
                count += 1;
                if count == 3 {
                    println!("Даю тебе подсказку, в этом учавствует веревка");
                } else if count == 5 {
                    println!("Даю тебе подсказку, в этом еще учавствует стул");
                } else if count == 6 {
                    println!("Даю тебе подсказку, тут присутствует мыло");
                }
            }
        }
    }
}
fn _4_word(game_words: &mut Words) {
    println!("Прежде чем начнем, сделай 15 приседаний.\nТолько обязательно, без этого ничего не выйдет,\nкак закончишь, напиши 'готово'");
    let answer: String = read!();
    game_words._4_q = String::from(game_words._4.as_str());
    println!("Устала?");
    think(1);
    println!("Это было все задание 😆 😆 😆");
    think(2);
    println!("Можно приступать к следующей букве. Напиши номер буквы 👇 и отгадай загадку.");
}
fn _5_word(game_words: &mut Words) {
    println!("Задача очень простая, отгадай загадку. 🕵");
    think(3);
    println!(
        "
    -----------------------------------------------
    В перерыве в классе оставалось девять учеников.
    Один из них разбил окно.
    На вопрос учителя были получены следующие ответы:

        - Якоб. Это сделал Джек.
        - Боб. Это неправда.
        - Мария. Я его разбила.
        - Джон. Сделала это либо Мария, либо Анна.
        - Джек. Боб лжет.
        - Том. Это была Мария.
        - Лео. Нет. Мария окно не разбивала.
        - Анна. Ни Мария, ни я этого не делали.
        - Розалия. Анна права, но Джек также не виновен.

    Если из этих девяти высказываний три, и только три истинны, кто разбил окно?🕵 🕵 🕵
    -------------------------------------------------"
    );
    let mut count = 0;
    loop {
        let answer: String = read!();
        match answer.to_lowercase().as_str() {
            "анна" => {
                println!("✅ Умница. Все верно.");
                game_words._5_q = String::from(game_words._5.as_str());
                println!(
                    "Можно приступать к следующей букве. Напиши номер буквы 👇 и отгадай загадку."
                );
                break;
            }
            _ => {
                println!("❌ Подумай еще");
                count += 1;
                if count == 3 {
                    println!("Даю тебе подсказку, это девочка");
                } else if count == 5 {
                    println!("Даю тебе подсказку, девочка с самым маленьким кол-вом букв в имени");
                }
            }
        }
    }
}
fn _6_word(game_words: &mut Words) {
    println!("Задача очень простая. Отгадай загадку.");
    println!(
        "
    Спутница самых первых дней,
    Каждый день приходим мы к ней;
    В очень давние времена
    Нелегко согревалась она."
    );
    println!();
    let mut count = 0;
    loop {
        let answer: String = read!();
        match answer.to_lowercase().as_str() {
            "кровать" => {
                println!("✅ Умница. Все верно.");
                game_words._6_q = String::from(game_words._6.as_str());
                println!(
                    "Можно приступать к следующей букве. Напиши номер буквы 👇 и отгадай загадку."
                );
                break;
            }
            _ => {
                println!("❌ Подумай еще");
                count += 1;
                if count == 3 {
                    println!("Даю тебе подсказку, я всегда люблю что бы она была убрана");
                } else if count == 5 {
                    println!("Даю тебе подсказку, на ней лежат подушки");
                } else if count == 6 {
                    println!("Даю тебе подсказку, на ней спят");
                }
            }
        }
    }
}
fn _7_word(game_words: &mut Words) {
    println!("Задача очень простая. Отгадай загадку.");
    println!(
        "
    – Она красная?
    – Нет, черная.
    – А почему она сейчас белая?
    – Потому, что еще зеленая.

    О чем речь?"
    );
    let mut count = 0;
    loop {
        let answer: String = read!();
        match answer.to_lowercase().as_str() {
            "смородина" => {
                println!("✅ Умница. Все верно.");
                game_words._7_q = String::from(game_words._7.as_str());
                println!(
                    "Можно приступать к следующей букве. Напиши номер буквы 👇 и отгадай загадку."
                );
                break;
            }
            _ => {
                println!("❌ Подумай еще");
                count += 1;
                if count == 3 {
                    println!("Даю тебе подсказку, это ягода");
                } else if count == 5 {
                    println!("Даю тебе подсказку, ягода растет в подлесном");
                } else if count == 6 {
                    println!("Даю тебе подсказку, как виноград, только очень маленькая ягода");
                }
            }
        }
    }
}
fn _8_word(game_words: &mut Words) {
    println!("Сложная задачка.\nПридется попотеть.");
    think(2);
    println!("Имеем парковку с номерами");
    println!();
    println!(
        "
    |    |    |    |    |    |    |
    |    |    |    |    |    |    |
    | 16 | 06 | 68 | 88 | 🚗  | 98 |
    |____|____|____|____|____|____|
    "
    );
    think(3);
    println!("Какая цифра скрывается под автомобилем?");
    let mut count = 0;
    loop {
        let answer = read!();
        match answer {
            87 => {
                println!("✅ Умница. Все верно.");
                game_words._8_q = String::from(game_words._8.as_str());
                println!(
                    "Можно приступать к следующей букве. Напиши номер буквы 👇 и отгадай загадку."
                );
                break;
            }
            _ => {
                println!("❌ Подумай еще");
                count += 1;
                if count == 3 {
                    println!("Даю тебе подсказку, цифры означают не то что ты видишь");
                } else if count == 5 {
                    println!("Даю тебе подсказку, цифры перевернуты");
                }
            }
        }
    }
}

fn check_words(game_words: &mut Words) -> bool {
    if game_words._1 == game_words._1_q
        && game_words._2 == game_words._2_q
        && game_words._3 == game_words._3_q
        && game_words._4 == game_words._4_q
        && game_words._5 == game_words._5_q
        && game_words._6 == game_words._6_q
        && game_words._7 == game_words._7_q
        && game_words._8 == game_words._8_q
    {
        return true;
    }
    return false;
}

fn print_words(game_words: &mut Words) {
    println!(
        "[{}][{}][{}][{}][{}][{}][{}][{}][{}][{}]",
        game_words._1_q,
        game_words._2_q,
        game_words._3_q,
        game_words._4_q,
        game_words._5_q,
        game_words._6_q,
        game_words._7_q,
        game_words._8_q,
        game_words._5_q,
        game_words._7_q,
    );
}
