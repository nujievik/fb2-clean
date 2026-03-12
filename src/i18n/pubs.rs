use super::{Lang, Msg};
use std::{
    env, fmt,
    str::FromStr,
    sync::{LazyLock, RwLock},
};

static LANG: LazyLock<RwLock<Lang>> =
    LazyLock::new(|| RwLock::new(get_lang_from_system_locale().unwrap_or_default()));

/// Returns a localized string of [`Msg`].
#[macro_export]
macro_rules! msg {
    ($x:ident) => {
        $crate::Msg::$x.as_str()
    };
}

/// Displays a localized string.
impl fmt::Display for Msg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Msg {
    /// Returns a localized string.
    pub fn as_str(self) -> &'static str {
        match Self::lang() {
            Lang::Rus => self.as_str_rus(),
            _ => self.as_str_eng(),
        }
    }

    /// Returns the current language.
    pub fn lang() -> Lang {
        LANG.read().map(|guard| *guard).unwrap_or_default()
    }

    /// Tries set language.
    ///
    /// # Errors
    /// Returns an error if internal RwLock::write failed.
    pub fn set_lang(lang: Lang) -> Result<(), String> {
        if lang == Self::lang() {
            Ok(())
        } else {
            LANG.write()
                .map(|mut l| *l = lang)
                .map_err(|e| e.to_string())
        }
    }
}

impl FromStr for Lang {
    type Err = String;

    fn from_str(s: &str) -> Result<Lang, Self::Err> {
        get_lang_from_str(s).ok_or_else(|| Msg::NotFoundAValidLangCode.to_string())
    }
}

fn get_lang_from_system_locale() -> Option<Lang> {
    let locale = env::var("LC_ALL")
        .ok()
        .or_else(|| env::var("LANG").ok())
        .or_else(|| env::var("LC_MESSAGES").ok())
        .or_else(|| get_system_locale_fallback())?;

    return get_lang_from_str(&locale);

    fn get_system_locale_fallback() -> Option<String> {
        #[cfg(windows)]
        {
            use std::ffi::OsString;
            use std::os::windows::ffi::OsStringExt;
            use winapi::um::winnls::GetUserDefaultLocaleName;

            const LOCALE_NAME_MAX_LENGTH: usize = 85;
            let mut buffer = [0u16; LOCALE_NAME_MAX_LENGTH];

            let len = unsafe {
                GetUserDefaultLocaleName(buffer.as_mut_ptr(), LOCALE_NAME_MAX_LENGTH as i32)
            };

            if len > 0 {
                let os_str = OsString::from_wide(&buffer[..(len as usize - 1)]);
                os_str.into_string().ok()
            } else {
                None
            }
        }

        #[cfg(unix)]
        {
            None
        }
    }
}

fn get_lang_from_str(s: &str) -> Option<Lang> {
    fn str_to_ascii_words(s: &str) -> impl Iterator<Item = &str> {
        use lazy_regex::{Lazy, Regex, regex};
        static REGEX_ASCII_WORD: &Lazy<Regex> = regex!(r"[a-zA-Z]+");
        REGEX_ASCII_WORD.find_iter(s).map(|mat| mat.as_str())
    }

    let mut buf = [0u8; 3];
    str_to_ascii_words(s).find_map(|s| {
        let len = s.len();
        if !matches!(len, 2 | 3) {
            return None;
        }
        for (dst, src) in buf[..len].iter_mut().zip(s.bytes()) {
            *dst = src.to_ascii_lowercase();
        }

        let lang = match &buf[..len] {
            b"en" | b"eng" => Lang::Eng,
            b"ru" | b"rus" => Lang::Rus,
            _ => return None,
        };
        Some(lang)
    })
}
