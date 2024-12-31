use pyo3::prelude::*;
use linux_keyutils::{KeyRing, KeyRingIdentifier, KeyError};
use zeroize::Zeroizing;

pub enum PythonLinuxKeyutilsError {
  AccessDenied,
  QuotaExceeded,
  BadAddress,
  InvalidArguments,
  KeyExpired,
  KeyRevoked,
  KeyRejected,
  KeyringDoesNotExist,
  KeyDoesNotExist,
  OutOfMemory,
  InvalidDescription,
  InvalidIdentifier,
  OperationNotSupported,
  WriteError,
  UTF8Error,
  Unknown,
}

impl From<PythonLinuxKeyutilsError> for PyErr {
    fn from(err: PythonLinuxKeyutilsError) -> PyErr {
        match err {
            PythonLinuxKeyutilsError::AccessDenied => PyErr::new::<pyo3::exceptions::PyOSError, _>("Access denied"),
            PythonLinuxKeyutilsError::QuotaExceeded => PyErr::new::<pyo3::exceptions::PyOSError, _>("Quota exceeded"),
            PythonLinuxKeyutilsError::BadAddress => PyErr::new::<pyo3::exceptions::PyOSError, _>("Bad address"),
            PythonLinuxKeyutilsError::InvalidArguments => PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid argument"),
            PythonLinuxKeyutilsError::KeyExpired => PyErr::new::<pyo3::exceptions::PyKeyError, _>("Key expired"),
            PythonLinuxKeyutilsError::KeyRevoked => PyErr::new::<pyo3::exceptions::PyKeyError, _>("Key revoked"),
            PythonLinuxKeyutilsError::KeyRejected => PyErr::new::<pyo3::exceptions::PyKeyError, _>("Key rejected"),
            PythonLinuxKeyutilsError::KeyringDoesNotExist => PyErr::new::<pyo3::exceptions::PyKeyError, _>("Keyring does not exist"),
            PythonLinuxKeyutilsError::KeyDoesNotExist => PyErr::new::<pyo3::exceptions::PyKeyError, _>("Key does not exist"),
            PythonLinuxKeyutilsError::OutOfMemory => PyErr::new::<pyo3::exceptions::PyMemoryError, _>("Out of memory"),
            PythonLinuxKeyutilsError::InvalidDescription => PyErr::new::<pyo3::exceptions::PyKeyError, _>("Invalid description"),
            PythonLinuxKeyutilsError::InvalidIdentifier => PyErr::new::<pyo3::exceptions::PyKeyError, _>("Invalid identifier"),
            PythonLinuxKeyutilsError::OperationNotSupported => PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Operation not supported"),
            PythonLinuxKeyutilsError::WriteError => PyErr::new::<pyo3::exceptions::PyOSError, _>("Write error"),
            PythonLinuxKeyutilsError::UTF8Error => PyErr::new::<pyo3::exceptions::PyUnicodeDecodeError, _>("Unicode error"),
            _ =>  PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Unknown error"),
        }
    }
}

impl From<std::str::Utf8Error> for PythonLinuxKeyutilsError {
    fn from(_: std::str::Utf8Error) -> PythonLinuxKeyutilsError {
        PythonLinuxKeyutilsError::UTF8Error
    }
}

impl From<KeyError> for PythonLinuxKeyutilsError {
    fn from(err: KeyError) -> PythonLinuxKeyutilsError {
        match err {
            KeyError::AccessDenied => PythonLinuxKeyutilsError::AccessDenied,
            KeyError::QuotaExceeded => PythonLinuxKeyutilsError::QuotaExceeded,
            KeyError::BadAddress => PythonLinuxKeyutilsError::BadAddress,
            KeyError::InvalidArguments => PythonLinuxKeyutilsError::InvalidArguments,
            KeyError::KeyExpired => PythonLinuxKeyutilsError::KeyExpired,
            KeyError::KeyRevoked => PythonLinuxKeyutilsError::KeyRevoked,
            KeyError::KeyRejected => PythonLinuxKeyutilsError::KeyRejected,
            KeyError::KeyringDoesNotExist => PythonLinuxKeyutilsError::KeyringDoesNotExist,
            KeyError::KeyDoesNotExist => PythonLinuxKeyutilsError::KeyDoesNotExist,
            KeyError::OutOfMemory => PythonLinuxKeyutilsError::OutOfMemory,
            KeyError::InvalidDescription => PythonLinuxKeyutilsError::InvalidDescription,
            KeyError::InvalidIdentifier => PythonLinuxKeyutilsError::InvalidIdentifier,
            KeyError::OperationNotSupported => PythonLinuxKeyutilsError::OperationNotSupported,
            KeyError::WriteError => PythonLinuxKeyutilsError::WriteError,
            _ => PythonLinuxKeyutilsError::Unknown,
        }
    }
}


fn _set_session_secret(name: String, secret: String) -> Result<(), PythonLinuxKeyutilsError> {
    let ring = KeyRing::from_special_id(KeyRingIdentifier::Session, false)?;
    ring.add_key(&name, &secret)?;
    Ok(())
}

#[pyfunction]
fn set_session_secret(name: String, secret: String) -> PyResult<()> {
    _set_session_secret(name, secret)?;
    Ok(())
}

fn _get_session_secret(name: String) -> Result<String, PythonLinuxKeyutilsError> {
    let ring = KeyRing::from_special_id(KeyRingIdentifier::Session, false)?;
    let key = ring.search(&name)?;
    let mut buf = Zeroizing::new([0u8; 2048]);
    let len = key.read(&mut buf)?;
    let utf_str = std::str::from_utf8(&buf[..len])?;

    Ok(utf_str.to_owned())
}

#[pyfunction]
fn get_session_secret(name: String) -> PyResult<String> {
    Ok(_get_session_secret(name)?)
}

fn _invalidate_session_secret(name: String) -> Result<(), PythonLinuxKeyutilsError> {
    let ring = KeyRing::from_special_id(KeyRingIdentifier::Session, false)?;
    let key = ring.search(&name)?;
    key.invalidate()?;
    Ok(())
}

#[pyfunction]
fn invalidate_session_secret(name: String) -> PyResult<()> {
    _invalidate_session_secret(name)?;
    Ok(())
}

#[pymodule]
fn python_linux_keyutils(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(set_session_secret, m)?)?;
    m.add_function(wrap_pyfunction!(get_session_secret, m)?)?;
    m.add_function(wrap_pyfunction!(invalidate_session_secret, m)?)?;
    Ok(())
}
