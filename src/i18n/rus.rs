use super::Msg;

#[inline(always)]
pub(super) fn rus(msg: Msg) -> &'static str {
    match msg {
        Msg::ErrUpdLangCode => "Не удалось обновить код языка",
        Msg::ErrWriteJson => "Ошибка записи команды в JSON",
        Msg::FileTypeNotSup => "Тип файла не поддерживается",
        Msg::FromPackage => "Из пакета",
        Msg::InstallTool => "Пожалуйста, установите его, добавьте в системный PATH и перезапустите",
        Msg::MayFailIfCommandLong => "Может привести к ошибке если команда длинная",
        Msg::NoInputMedia => "Не найдены медиа во входной директории",
        Msg::NotFound => "Не найден",
        Msg::RunningCommand => "Выполнение команды",
        Msg::Using => "Используется",
    }
}
