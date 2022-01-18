use std::env;
use std::error::Error;
use std::fmt;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Version {
    major: i32,
    minor: i32,
    patch: i32,
    display: String,
}

impl FromStr for Version {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = s.split(&['.', '-'][..]).collect();
        Ok(Version {
            major: v[0].parse::<i32>()?,
            minor: v[1].parse::<i32>()?,
            patch: v[2].parse::<i32>().unwrap_or(-1),
            display: s.to_string(),
        })
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.display)
    }
}

fn pyenv_latest() -> Result<Version, Box<dyn Error>> {
    Ok(Path::new(&env::var("PYENV_ROOT")?)
        .join("versions")
        .read_dir()?
        .filter_map(|entry| Version::from_str(&entry.ok()?.file_name().into_string().ok()?).ok())
        .max()
        .unwrap())
}

fn main() {
    println!("{}", pyenv_latest().unwrap());
}

#[cfg(test)]
mod tests {
    use crate::Version;
    use std::str::FromStr;
    #[test]
    fn version_normal() {
        let s = "1.2.3";
        let v = Version::from_str(&s).unwrap();
        assert_eq!(format!("{}", v), s);
        assert_eq!((v.major, v.minor, v.patch), (1, 2, 3));
    }
    #[test]
    fn version_beta() {
        let s = "1.2.3b4";
        let v = Version::from_str(&s).unwrap();
        assert_eq!(format!("{}", v), s);
        assert_eq!((v.major, v.minor, v.patch), (1, 2, -1));
    }
    #[test]
    fn version_dev() {
        let s = "1.2-dev";
        let v = Version::from_str(&s).unwrap();
        assert_eq!(format!("{}", v), s);
        assert_eq!((v.major, v.minor, v.patch), (1, 2, -1));
    }
}
