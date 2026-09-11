macro_rules! impl_msg_as_str {
    ($fn:ident, $( $enum_var:ident => $text:expr ),* $(,)?) => {
        impl $crate::Msg {
            #[inline(always)]
            pub(in crate::i18n) fn $fn(self) -> &'static str {
                #[allow(deprecated)]
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

    Error,
    Warning,
    Debug,
    Trace,

    CleaningBooks,
    Cleaning,
    SuccessfullyCleanedTo,

    OverwritingBooks,
    Overwriting,
    SuccessfullyOverwritedFrom,
    RemovingInputFile,
    RemovingTempDirectory,
    RemovingTempFile,

    FileAlreadyExists,
    #[deprecated]
    FileIsAlreadyExists,
    NotFoundAValidLangCode,
    NotFoundAnyBookInDirectory,
    NotFoundAnyFb2InArchive,
    Skipping,
    #[deprecated]
    SuccessCleanedAndSavedTo,
    #[deprecated]
    SuccessOverwritedFrom,
}
