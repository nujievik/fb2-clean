use indexmap::IndexSet;
use std::{ffi::OsStr, fmt, ops::Deref};

/// Remove tags configuration.
#[derive(Clone, Debug, PartialEq)]
pub struct Tags(pub IndexSet<Box<[u8]>>);

impl Deref for Tags {
    type Target = IndexSet<Box<[u8]>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Tags {
    pub(crate) fn new(os: impl AsRef<OsStr>) -> Tags {
        let mut set: IndexSet<Box<[u8]>> = IndexSet::new();

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
    ///
    /// let tags = Tags::default();
    /// assert_eq!(3, tags.len());
    /// for t in ["binary", "coverpage", "image"] {
    ///     assert!(tags.contains(t.as_bytes()));
    /// }
    /// ```
    fn default() -> Tags {
        let mut set: IndexSet<Box<[u8]>> = IndexSet::with_capacity(3);
        set.insert(b"binary".as_slice().into());
        set.insert(b"coverpage".as_slice().into());
        set.insert(b"image".as_slice().into());
        Tags(set)
    }
}

impl fmt::Display for Tags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        for (i, t) in (1..).zip(self.0.iter()) {
            write!(f, "{}", String::from_utf8_lossy(t))?;
            if i < self.0.len() {
                write!(f, ",")?;
            }
        }
        Ok(())
    }
}
