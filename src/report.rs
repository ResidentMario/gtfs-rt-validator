use std::fmt;

use crate::err::{ErrKind, ValidationError};
use crate::transit_realtime;

use prost::Message;

pub struct ErrorReport {
    errors: Vec<ValidationError>,
}

impl fmt::Display for ErrorReport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Error report with {} errors.", self.errors.len())?;
        if self.errors.len() == 0 {
            return Ok(());
        }

        writeln!(f, "Errors are:")?;
        for error in &self.errors {
            writeln!(f, "{}", error)?;
        }
        return Ok(());
    }
}

impl ErrorReport {
    pub fn from_buffer(buf: &[u8]) -> ErrorReport {
        let feed_message = transit_realtime::FeedMessage::decode(buf).map_err(|e| {
            ValidationError::wrap(
                ErrKind::InvalidDataError,
                e,
                format!("Error parsing the file"),
            )
        });
        let _ = match feed_message {
            Ok(p) => p,
            Err(e) => return ErrorReport { errors: vec![e] },
        };

        // TODO: parse for other kinds of errors

        return ErrorReport { errors: vec![] };
    }
}
