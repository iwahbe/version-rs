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
    }
}

#[cfg(not(feature = "serde"))]
#[derive(Eq, PartialEq)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub revision: u32,
}

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "serde")]
#[Derive(Serialize, Deserialize)]
#[derive(Eq, PartialEq)]
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

impl From<(u32, u32, u32)> for Version {
    fn from(elem: (u32, u32, u32)) -> Self {
        Version {
            major: elem.0,
            minor: elem.1,
            revision: elem.2,
        }
    }
}

impl From<(u32, u32)> for Version {
    fn from(elem: (u32, u32)) -> Self {
        Version {
            major: elem.0,
            minor: elem.1,
            revision: 0,
        }
    }
}

impl From<u32> for Version {
    fn from(elem: u32) -> Self {
        Version {
            major: elem,
            minor: 0,
            revision: 0,
        }
    }
}

impl From<(u16, u16, u16)> for Version {
    fn from(elem: (u16, u16, u16)) -> Self {
        Version {
            major: elem.0 as u32,
            minor: elem.1 as u32,
            revision: elem.2 as u32,
        }
    }
}

impl From<(u16, u16)> for Version {
    fn from(elem: (u16, u16)) -> Self {
        Version {
            major: elem.0 as u32,
            minor: elem.1 as u32,
            revision: 0,
        }
    }
}

impl From<u16> for Version {
    fn from(elem: u16) -> Self {
        Version {
            major: elem as u32,
            minor: 0,
            revision: 0,
        }
    }
}

impl From<(u8, u8, u8)> for Version {
    fn from(elem: (u8, u8, u8)) -> Self {
        Version {
            major: elem.0 as u32,
            minor: elem.1 as u32,
            revision: elem.2 as u32,
        }
    }
}

impl From<(u8, u8)> for Version {
    fn from(elem: (u8, u8)) -> Self {
        Version {
            major: elem.0 as u32,
            minor: elem.1 as u32,
            revision: 0,
        }
    }
}

impl From<u8> for Version {
    fn from(elem: u8) -> Self {
        Version {
            major: elem as u32,
            minor: 0,
            revision: 0,
        }
    }
}

impl From<(usize, usize, usize)> for Version {
    fn from(elem: (usize, usize, usize)) -> Self {
        Version {
            major: elem.0 as u32,
            minor: elem.1 as u32,
            revision: elem.2 as u32,
        }
    }
}

impl From<(usize, usize)> for Version {
    fn from(elem: (usize, usize)) -> Self {
        Version {
            major: elem.0 as u32,
            minor: elem.1 as u32,
            revision: 0,
        }
    }
}

impl From<usize> for Version {
    fn from(elem: usize) -> Self {
        Version {
            major: elem as u32,
            minor: 0,
            revision: 0,
        }
    }
}

/// A parse error for Version
pub enum VersionError {
    /// Attempted to parse an empty string
    Empty,
    /// The string contained too many decimals, contains the number of decimals
    TooManyDecimals(usize),
    /// Encountered a num::ParseIntError
    ParseError(std::num::ParseIntError),
}

impl From<std::num::ParseIntError> for VersionError {
    fn from(e: std::num::ParseIntError) -> Self {
        Self::ParseError(e)
    }
}

impl std::str::FromStr for Version {
    type Err = VersionError;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut three = string.split('.');
        let res: Version;
        if let Some(major) = three.next() {
            if let Some(minor) = three.next() {
                if let Some(revision) = three.next() {
                    res = Version::from((major.parse::<u32>()?, minor.parse()?, revision.parse()?));
                } else {
                    res = Version::from((major.parse::<u32>()?, minor.parse()?, 0));
                }
            } else {
                res = Version::from((major.parse::<u32>()?, 0, 0));
            }
        } else {
            return Err(VersionError::Empty);
        }
        if three.next() == None {
            Ok(res)
        } else {
            Err(VersionError::TooManyDecimals(4 + three.count()))
        }
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
