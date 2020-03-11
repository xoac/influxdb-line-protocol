//! Measurement names, tag keys, and field keys cannot begin with an underscore _. The _ namespace is reserved for InfluxDB system use.
//!
//! source: https://v2.docs.influxdata.com/v2.0/reference/syntax/line-protocol/#naming-restrictions
//!
//!

use super::error::Error;

#[inline]
fn prevent_start_with_(s: &str) -> Result<(), Error> {
    if s.starts_with('_') {
        Err(Error::StartWithForbieden_)
    } else {
        Ok(())
    }
}

#[inline]
fn prevent_newline(s: &str) -> Result<(), Error> {
    if s.contains('\n') {
        Err(Error::NewLine)
    } else {
        Ok(())
    }
}

#[inline]
pub fn prevent_key(s: &str) -> Result<(), Error> {
    prevent_start_with_(s)?;
    prevent_newline(s)
}

#[inline]
pub fn prevent_tag_value(s: &str) -> Result<(), Error> {
    prevent_newline(s)
}

#[inline]
pub fn prevent_filed_value_string(s: &str) -> Result<(), Error> {
    prevent_newline(s)
}

#[inline]
pub fn check_measurement(s: &str) -> Result<(), Error> {
    prevent_newline(s)
}
