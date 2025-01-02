use linux_keyutils;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
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
            PythonLinuxKeyutilsError::AccessDenied => {
                PyErr::new::<pyo3::exceptions::PyOSError, _>("Access denied")
            }
            PythonLinuxKeyutilsError::QuotaExceeded => {
                PyErr::new::<pyo3::exceptions::PyOSError, _>("Quota exceeded")
            }
            PythonLinuxKeyutilsError::BadAddress => {
                PyErr::new::<pyo3::exceptions::PyOSError, _>("Bad address")
            }
            PythonLinuxKeyutilsError::InvalidArguments => {
                PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid argument")
            }
            PythonLinuxKeyutilsError::KeyExpired => {
                PyErr::new::<pyo3::exceptions::PyKeyError, _>("Key expired")
            }
            PythonLinuxKeyutilsError::KeyRevoked => {
                PyErr::new::<pyo3::exceptions::PyKeyError, _>("Key revoked")
            }
            PythonLinuxKeyutilsError::KeyRejected => {
                PyErr::new::<pyo3::exceptions::PyKeyError, _>("Key rejected")
            }
            PythonLinuxKeyutilsError::KeyringDoesNotExist => {
                PyErr::new::<pyo3::exceptions::PyKeyError, _>("Keyring does not exist")
            }
            PythonLinuxKeyutilsError::KeyDoesNotExist => {
                PyErr::new::<pyo3::exceptions::PyKeyError, _>("Key does not exist")
            }
            PythonLinuxKeyutilsError::OutOfMemory => {
                PyErr::new::<pyo3::exceptions::PyMemoryError, _>("Out of memory")
            }
            PythonLinuxKeyutilsError::InvalidDescription => {
                PyErr::new::<pyo3::exceptions::PyKeyError, _>("Invalid description")
            }
            PythonLinuxKeyutilsError::InvalidIdentifier => {
                PyErr::new::<pyo3::exceptions::PyKeyError, _>("Invalid identifier")
            }
            PythonLinuxKeyutilsError::OperationNotSupported => {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Operation not supported")
            }
            PythonLinuxKeyutilsError::WriteError => {
                PyErr::new::<pyo3::exceptions::PyOSError, _>("Write error")
            }
            _ => PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Unknown error"),
        }
    }
}

impl From<std::str::Utf8Error> for PythonLinuxKeyutilsError {
    fn from(_: std::str::Utf8Error) -> PythonLinuxKeyutilsError {
        PythonLinuxKeyutilsError::UTF8Error
    }
}

impl From<linux_keyutils::KeyError> for PythonLinuxKeyutilsError {
    fn from(err: linux_keyutils::KeyError) -> PythonLinuxKeyutilsError {
        match err {
            linux_keyutils::KeyError::AccessDenied => PythonLinuxKeyutilsError::AccessDenied,
            linux_keyutils::KeyError::QuotaExceeded => PythonLinuxKeyutilsError::QuotaExceeded,
            linux_keyutils::KeyError::BadAddress => PythonLinuxKeyutilsError::BadAddress,
            linux_keyutils::KeyError::InvalidArguments => {
                PythonLinuxKeyutilsError::InvalidArguments
            }
            linux_keyutils::KeyError::KeyExpired => PythonLinuxKeyutilsError::KeyExpired,
            linux_keyutils::KeyError::KeyRevoked => PythonLinuxKeyutilsError::KeyRevoked,
            linux_keyutils::KeyError::KeyRejected => PythonLinuxKeyutilsError::KeyRejected,
            linux_keyutils::KeyError::KeyringDoesNotExist => {
                PythonLinuxKeyutilsError::KeyringDoesNotExist
            }
            linux_keyutils::KeyError::KeyDoesNotExist => PythonLinuxKeyutilsError::KeyDoesNotExist,
            linux_keyutils::KeyError::OutOfMemory => PythonLinuxKeyutilsError::OutOfMemory,
            linux_keyutils::KeyError::InvalidDescription => {
                PythonLinuxKeyutilsError::InvalidDescription
            }
            linux_keyutils::KeyError::InvalidIdentifier => {
                PythonLinuxKeyutilsError::InvalidIdentifier
            }
            linux_keyutils::KeyError::OperationNotSupported => {
                PythonLinuxKeyutilsError::OperationNotSupported
            }
            linux_keyutils::KeyError::WriteError => PythonLinuxKeyutilsError::WriteError,
            _ => PythonLinuxKeyutilsError::Unknown,
        }
    }
}

#[pyclass(eq, eq_int)]
#[derive(PartialEq)]
enum KeyRingIdentifier {
    Thread,
    Process,
    Session,
    User,
    UserSession,
    Group,
    ReqKeyAuthKey,
}

impl From<&KeyRingIdentifier> for linux_keyutils::KeyRingIdentifier {
    fn from(id: &KeyRingIdentifier) -> linux_keyutils::KeyRingIdentifier {
        match id {
            KeyRingIdentifier::Thread => linux_keyutils::KeyRingIdentifier::Thread,
            KeyRingIdentifier::Process => linux_keyutils::KeyRingIdentifier::Process,
            KeyRingIdentifier::Session => linux_keyutils::KeyRingIdentifier::Session,
            KeyRingIdentifier::User => linux_keyutils::KeyRingIdentifier::User,
            KeyRingIdentifier::UserSession => linux_keyutils::KeyRingIdentifier::UserSession,
            KeyRingIdentifier::Group => linux_keyutils::KeyRingIdentifier::Group,
            KeyRingIdentifier::ReqKeyAuthKey => linux_keyutils::KeyRingIdentifier::ReqKeyAuthKey,
        }
    }
}
fn get_ring(
    key_ring: Option<&KeyRingIdentifier>,
    create: bool,
) -> Result<linux_keyutils::KeyRing, PythonLinuxKeyutilsError> {
    match key_ring {
        Some(ring) => Ok(linux_keyutils::KeyRing::from_special_id(
            linux_keyutils::KeyRingIdentifier::from(ring),
            create,
        )?),
        None => Ok(linux_keyutils::KeyRing::from_special_id(
            linux_keyutils::KeyRingIdentifier::Session,
            create,
        )?),
    }
}

fn _set_secret(
    name: String,
    secret: &[u8],
    key_ring: Option<&KeyRingIdentifier>,
    create: bool,
) -> Result<(), PythonLinuxKeyutilsError> {
    let ring = get_ring(key_ring, create)?;
    ring.add_key(&name, secret)?;
    Ok(())
}

#[pyfunction]
#[pyo3(signature = (name, secret, /, key_ring=None, create=false))]
fn set_secret<'py>(
    name: String,
    secret: &Bound<'py, PyBytes>,
    key_ring: Option<&KeyRingIdentifier>,
    create: Option<bool>,
) -> PyResult<()> {
    _set_secret(
        name,
        secret.as_bytes(),
        key_ring,
        match create {
            Some(value) => value,
            None => false,
        },
    )?;
    Ok(())
}

struct Secret {
    buffer: Zeroizing<[u8; 2048]>,
    length: usize,
}

fn _get_secret(
    name: String,
    key_ring: Option<&KeyRingIdentifier>,
) -> Result<Secret, PythonLinuxKeyutilsError> {
    let ring = get_ring(key_ring, false)?;
    let key = ring.search(&name)?;
    let mut buf = Zeroizing::new([0u8; 2048]);
    let len = key.read(&mut buf)?;
    Ok(Secret {
        buffer: buf,
        length: len,
    })
}

#[pyfunction]
#[pyo3(signature = (name, /, key_ring=None))]
fn get_secret<'py>(
    py: Python<'py>,
    name: String,
    key_ring: Option<&KeyRingIdentifier>,
) -> PyResult<Bound<'py, PyBytes>> {
    let secret = _get_secret(name, key_ring)?;
    Ok(PyBytes::new_bound(py, &secret.buffer[..secret.length]))
}

fn _invalidate_secret(
    name: String,
    key_ring: Option<&KeyRingIdentifier>,
) -> Result<(), PythonLinuxKeyutilsError> {
    let ring = get_ring(key_ring, false)?;
    let key = ring.search(&name)?;
    key.invalidate()?;
    Ok(())
}

#[pyfunction]
#[pyo3(signature = (name, /, key_ring=None))]
fn invalidate_secret(name: String, key_ring: Option<&KeyRingIdentifier>) -> PyResult<()> {
    _invalidate_secret(name, key_ring)?;
    Ok(())
}

#[pymodule]
fn python_linux_keyutils(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(set_secret, m)?)?;
    m.add_function(wrap_pyfunction!(get_secret, m)?)?;
    m.add_function(wrap_pyfunction!(invalidate_secret, m)?)?;
    m.add_class::<KeyRingIdentifier>()?;
    Ok(())
}
