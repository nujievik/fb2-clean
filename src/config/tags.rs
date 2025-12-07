use std::{collections::HashSet, ffi::OsStr, ops::Deref};

/// Clean tags configuration.
#[derive(Clone, Debug, PartialEq)]
pub struct Tags(pub HashSet<Box<[u8]>>);

impl Deref for Tags {
    type Target = HashSet<Box<[u8]>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Tags {
    /// Always returns `Ok`.
    pub(crate) fn fallible_new(os: impl AsRef<OsStr>) -> Result<Tags, String> {
        Ok(Self::new(os))
    }

    pub(crate) fn new(os: impl AsRef<OsStr>) -> Tags {
        let mut set: HashSet<Box<[u8]>> = HashSet::new();

        let bytes = os.as_ref().as_encoded_bytes();
        let len = bytes.len();
        let mut i = 0;

        for j in 0..=len {
            if j == len || bytes[j] == b',' {
                if i != j {
                    set.insert(bytes[i..j].into());
                }
                i = j + 1;
            }
        }

        Tags(set)
    }
}

impl Default for Tags {
    /// Returns new [`Tags`] with "binary", "coverpage" and "image" bytes.
    /// ```
    /// use fb2_clean::Tags;
    /// use std::collections::HashSet;
    ///
    /// let tags = Tags::default();
    /// assert_eq!(3, tags.len());
    /// for t in ["binary", "coverpage", "image"] {
    ///     assert!(tags.contains(t.as_bytes()));
    /// }
    /// ```
    fn default() -> Tags {
        let mut set = HashSet::with_capacity(3);
        set.insert(b"binary".as_slice().into());
        set.insert(b"coverpage".as_slice().into());
        set.insert(b"image".as_slice().into());
        Tags(set)
    }
}
