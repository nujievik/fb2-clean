macro_rules! impl_msg_as_str {
    ($fn:ident, $( $enum_var:ident => $text:expr ),* $(,)?) => {
        impl $crate::Msg {
            #[inline(always)]
            pub(in crate::i18n) fn $fn(self) -> &'static str {
                match self {
                    $( Self::$enum_var => $text ),*
                }
            }
        }
    };
}

mod pubs;

mod eng;
mod rus;

/// A language of message.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[non_exhaustive]
pub enum Lang {
    #[default]
    Eng,
    Rus,
}

/// A message with localized methods.
#[derive(Copy, Clone, Debug)]
#[non_exhaustive]
pub enum Msg {
    Cleaning,
    CleaningBooks,
    FileIsAlreadyExists,
    NotFoundAValidLangCode,
    NotFoundAnyBookInDirectory,
    NotFoundAnyFb2InArchive,
    Overwriting,
    OverwritingBooks,
    RemovingInputFile,
    RemovingTempDirectory,
    RemovingTempFile,
    Skipping,
    SuccessCleanedAndSavedTo,
    SuccessOverwritedFrom,
    Error,
    Warning,
    Debug,
    Trace,
    GuiStart,
    GuiLanguage,
    GuiSelectToClean,
    GuiDirectory,
    GuiOr,
    GuiFiles,
    GuiSelectDirectoryToClean,
    GuiSelectFilesToClean,
    GuiSelectedToClean,
    GuiErrorSelectToClean,
    GuiSelectSaveDirectory,
    GuiSelectedSaveDirectory,
    GuiErrorSelectSaveDirectory,
    GuiRemoveTags,
    GuiTagsSet,
    GuiMultithreading,
    GuiRecursiveSearch,
    GuiOverwrite,
    GuiStopOnError,
    GuiLog,
    HelpInput,
    HelpOutput,
    HelpRecursive,
    HelpTags,
    HelpZip,
    HelpUnzip,
    HelpOverwrite,
    HelpExitOnError,
    HelpJobs,
    HelpLang,
    HelpVersion,
    HelpHelp,
}
