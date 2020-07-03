#[cfg(test)]
mod tests {
    use super::Version;
    #[test]
    fn from_str() {
        assert_eq!(Version::new(0, 1, 0), Version::from_str("0.1").unwrap());
        assert_eq!(Version::new(0, 1, 0), Version::from_str("0.1.0").unwrap());
        assert_eq!(None, Version::from_str("0.1."));
        assert_eq!(None, Version::from_str("0.."));
        assert_eq!(None, Version::from_str(".."));
        assert_eq!(None, Version::from_str("0.1.0.0"));
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

impl Version {
    pub fn from_str(string: &str) -> Option<Self> {
        let mut three = string.split('.');
        let res: Version;
        if let Some(major) = three.next() {
            if let Some(minor) = three.next() {
                if let Some(revision) = three.next() {
                    res = Version::from((
                        major.parse::<u32>().ok()?,
                        minor.parse().ok()?,
                        revision.parse().ok()?,
                    ));
                } else {
                    res = Version::from((major.parse::<u32>().ok()?, minor.parse().ok()?, 0));
                }
            } else {
                res = Version::from((major.parse::<u32>().ok()?, 0, 0));
            }
        } else {
            return None;
        }
        if three.next() == None {
            Some(res)
        } else {
            None
        }
    }
    pub fn new(major: u32, minor: u32, revision: u32) -> Self {
        Version {
            major,
            minor,
            revision,
        }
    }
    pub fn with_major(&self, major: u32) -> Self {
        Self {
            major: major,
            minor: self.minor,
            revision: self.revision,
        }
    }
    pub fn with_minor(&self, minor: u32) -> Self {
        Self {
            major: self.major,
            minor: minor,
            revision: self.revision,
        }
    }
    pub fn with_revision(&self, revision: u32) -> Self {
        Self {
            major: self.major,
            minor: self.minor,
            revision: revision,
        }
    }
}
