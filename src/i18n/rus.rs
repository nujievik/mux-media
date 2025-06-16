use super::Msg;

pub(super) fn rus(msg: &Msg) -> String {
    match msg {
        Msg::ErrUpdLangCode => "Не удалось обновить код языка".into(),
        Msg::FailSetPaths { s, s1 } => format!(
            "'{}' из пакета '{}' не установлен. Пожалуйста, установите его, добавьте в системный PATH и перезапустите",
            s, s1
        ),
        Msg::FailWriteJson { s } => format!(
            "Не удалось записать команду в JSON: {}. Используем CLI (может привести к ошибке, если команда слишком длинная)",
            s
        ),
        Msg::NoInputFiles => "Не найдены файлы с треками во входящей директории".to_string(),
        Msg::RunningCommand => "Выполнение команды".into(),
        Msg::Using => "Используется".into(),
    }
}
