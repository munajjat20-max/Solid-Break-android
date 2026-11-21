//! Minimal Android-compatible stub for `arboard` 3.7.x.
//! In Phase 1, clipboard operations return `Error::ClipboardNotSupported` cleanly
//! so SolidCraft compiles for `aarch64-linux-android`.

use std::borrow::Cow;
use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub enum Error {
    ContentNotAvailable,
    ClipboardNotSupported,
    ClipboardOccupied,
    ConversionFailure,
    Unknown { description: String },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ContentNotAvailable => write!(f, "Clipboard content not available"),
            Error::ClipboardNotSupported => write!(f, "Clipboard not supported on this platform"),
            Error::ClipboardOccupied => write!(f, "Clipboard is currently occupied"),
            Error::ConversionFailure => write!(f, "Clipboard data conversion failed"),
            Error::Unknown { description } => write!(f, "Clipboard error: {description}"),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Debug, Clone)]
pub struct ImageData<'a> {
    pub width: usize,
    pub height: usize,
    pub bytes: Cow<'a, [u8]>,
}

impl<'a> ImageData<'a> {
    pub fn to_owned_img(&self) -> ImageData<'static> {
        ImageData {
            width: self.width,
            height: self.height,
            bytes: Cow::Owned(self.bytes.as_ref().to_vec()),
        }
    }
}

pub struct Clipboard;

impl Clipboard {
    pub fn new() -> Result<Self, Error> {
        Ok(Self)
    }

    pub fn get_text(&mut self) -> Result<String, Error> {
        Err(Error::ContentNotAvailable)
    }

    pub fn set_text<'a, T: Into<Cow<'a, str>>>(&mut self, _text: T) -> Result<(), Error> {
        Ok(())
    }

    pub fn get_image(&mut self) -> Result<ImageData<'static>, Error> {
        Err(Error::ContentNotAvailable)
    }

    pub fn set_image(&mut self, _image: ImageData<'_>) -> Result<(), Error> {
        Ok(())
    }

    pub fn clear(&mut self) -> Result<(), Error> {
        Ok(())
    }

    pub fn get(&mut self) -> Get<'_> {
        Get { _clipboard: self }
    }

    pub fn set(&mut self) -> Set<'_> {
        Set { _clipboard: self }
    }

    pub fn clear_with(&mut self) -> Clear<'_> {
        Clear { _clipboard: self }
    }
}

pub struct Get<'a> {
    _clipboard: &'a mut Clipboard,
}

impl<'a> Get<'a> {
    pub fn text(self) -> Result<String, Error> {
        Err(Error::ContentNotAvailable)
    }

    pub fn html(self) -> Result<String, Error> {
        Err(Error::ContentNotAvailable)
    }

    pub fn image(self) -> Result<ImageData<'static>, Error> {
        Err(Error::ContentNotAvailable)
    }

    pub fn file_list(self) -> Result<Vec<PathBuf>, Error> {
        Err(Error::ContentNotAvailable)
    }
}

pub struct Set<'a> {
    _clipboard: &'a mut Clipboard,
}

impl<'a> Set<'a> {
    pub fn text<T: Into<Cow<'a, str>>>(self, _text: T) -> Result<(), Error> {
        Ok(())
    }

    pub fn html<T: Into<Cow<'a, str>>>(self, _html: T, _alt_text: Option<T>) -> Result<(), Error> {
        Ok(())
    }

    pub fn image(self, _image: ImageData<'_>) -> Result<(), Error> {
        Ok(())
    }
}

pub struct Clear<'a> {
    _clipboard: &'a mut Clipboard,
}

impl<'a> Clear<'a> {
    pub fn clear(self) -> Result<(), Error> {
        Ok(())
    }
}
