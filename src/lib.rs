#[cfg(test)]
mod tests {
    use super::Version;
    #[test]
    fn from_str() {
        assert_eq!(Version::new(0, 1, 0), "0.1".parse().ok().unwrap());
        assert_eq!(Version::new(0, 1, 0), "0.1.0".parse().ok().unwrap());
        assert!("0.1.".parse::<Version>().is_err());
        assert!("0..".parse::<Version>().is_err());
        assert!("..".parse::<Version>().is_err());
        assert!("0.1.0.0".parse::<Version>().is_err());
    }

    #[test]
    fn derive() {
        let one: u32 = 1;
        assert_eq!(Version::from((one, 2, 3)), (one, 2, 3).into());
        assert!(Version::from(1) < Version::from(2));
    }
}

#[cfg(not(feature = "serde"))]
#[derive(Eq, PartialEq, Clone)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub revision: u32,
}

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "serde")]
#[derive(Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(into = "String")]
#[serde(try_from = "&str")]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub revision: u32,
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}.{}.{}", self.major, self.minor, self.revision)
    }
}

impl std::fmt::Debug for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "v{}.{}.{}", self.major, self.minor, self.revision)
    }
}

impl std::cmp::PartialOrd for Version {
    fn partial_cmp(&self, r: &Version) -> std::option::Option<std::cmp::Ordering> {
        match self.major.cmp(&r.major) {
            std::cmp::Ordering::Equal => match self.minor.cmp(&r.minor) {
                std::cmp::Ordering::Equal => Some(self.revision.cmp(&r.revision)),
                other @ _ => Some(other),
            },
            other @ _ => Some(other),
        }
    }
}

impl std::cmp::Ord for Version {
    fn cmp(&self, r: &Self) -> std::cmp::Ordering {
        self.partial_cmp(r).unwrap()
    }
}

// conflicting dependency issues
// impl<N: std::convert::Into<u32>> From<N> for Version {
//     fn from(elem: N) -> Self {
//         Version {
//             major: elem.into(),
//             minor: 0,
//             revision: 0,
//         }
//     }
// }

macro_rules! primitive_from {
    ($type:tt) => {
        impl From<$type> for Version {
            fn from(elem: $type) -> Self {
                Version {
                    major: elem as u32,
                    minor: 0,
                    revision: 0,
                }
            }
        }
    };

    ($first:tt, $($rest:tt),+) => {
        primitive_from! {$first}
        primitive_from! {$($rest),+}
    };
}

primitive_from! {u32, u8, u16, i16, i8, i32, usize}

impl<N: std::convert::Into<u32>> From<(N, N)> for Version {
    fn from(elem: (N, N)) -> Self {
        Version {
            major: elem.0.into(),
            minor: elem.1.into(),
            revision: 0,
        }
    }
}

impl<N: std::convert::Into<u32>> From<(N, N, N)> for Version {
    fn from(elem: (N, N, N)) -> Self {
        Version {
            major: elem.0.into(),
            minor: elem.1.into(),
            revision: elem.2.into(),
        }
    }
}

/// A parse error for Version
#[derive(Debug)]
pub enum VersionError {
    /// Attempted to parse an empty string
    Empty,
    /// The string contained too many decimals, contains the number of decimals
    TooManyDecimals(usize),
    /// Encountered a num::ParseIntError
    ParseError(std::num::ParseIntError),
}

impl std::fmt::Display for VersionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            VersionError::Empty => write!(f, "VersionError::Empty"),
            VersionError::TooManyDecimals(d) => {
                write!(f, "VersionError::TooManyDecimals => found {}", d)
            }
            VersionError::ParseError(p) => write!(f, "VersionError::ParseError => {}", p),
        }
    }
}

impl From<std::num::ParseIntError> for VersionError {
    fn from(e: std::num::ParseIntError) -> Self {
        Self::ParseError(e)
    }
}

impl Into<String> for Version {
    fn into(self) -> String {
        format!("{:?}", self)
    }
}

impl std::str::FromStr for Version {
    type Err = VersionError;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut three;
        #[cfg(feature = "lossy")]
        {
            three = string.split(|x| match x {
                '0'..='9' => false,
                _ => true,
            });
        }
        #[cfg(not(feature = "lossy"))]
        {
            three = string.split('.');
        }
        let res: Version;
        if let Some(major) = three.next() {
            if let Some(minor) = three.next() {
                if let Some(revision) = three.next() {
                    #[cfg(feature = "lossy")]
                    {
                        res = Version::from((
                            major.parse::<u32>()?,
                            minor.parse().unwrap_or(0),
                            revision.parse().unwrap_or(0),
                        ));
                    }
                    #[cfg(not(feature = "lossy"))]
                    {
                        res = Version::from((
                            major.parse::<u32>()?,
                            minor.parse()?,
                            revision.parse()?,
                        ));
                    }
                } else {
                    #[cfg(feature = "lossy")]
                    {
                        res = Version::from((major.parse::<u32>()?, minor.parse().unwrap_or(0), 0));
                    }
                    #[cfg(not(feature = "lossy"))]
                    {
                        res = Version::from((major.parse::<u32>()?, minor.parse()?, 0));
                    }
                }
            } else {
                res = Version::from((major.parse::<u32>()?, 0, 0));
            }
        } else {
            return Err(VersionError::Empty);
        }
        #[cfg(not(feature = "lossy"))]
        if three.next() == None {
            Ok(res)
        } else {
            Err(VersionError::TooManyDecimals(4 + three.count()))
        }
        #[cfg(feature = "lossy")]
        Ok(res)
    }
}

impl std::convert::TryFrom<&str> for Version {
    type Error = VersionError;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        std::str::FromStr::from_str(string)
    }
}

impl Version {
    /// Creates a new version
    pub fn new(major: u32, minor: u32, revision: u32) -> Self {
        Version {
            major,
            minor,
            revision,
        }
    }

    /// Copies the version, updating the major number
    pub fn with_major(&self, major: u32) -> Self {
        Self {
            major: major,
            minor: self.minor,
            revision: self.revision,
        }
    }

    /// Copies the version, updating the minor number
    pub fn with_minor(&self, minor: u32) -> Self {
        Self {
            major: self.major,
            minor: minor,
            revision: self.revision,
        }
    }

    /// Copies the version, updating the revision number
    pub fn with_revision(&self, revision: u32) -> Self {
        Self {
            major: self.major,
            minor: self.minor,
            revision: revision,
        }
    }
}
