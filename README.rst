===============================
Linux kernel keyutils binddings
===============================

Linux kernel provides a secure storage for sensitive data. This package provides a way to set, retrieve
and invalidate key-value pairs in the kernel keyring in session scope.

Package is in early stage of development, and using keyrings other than session storage is currently
unsupported.

####
Why?
####

Existing `keyring <https://pypi.org/project/keyring/>`_ package is very powerful, but somewhat complex
and heavy.

`keyctl <https://pypi.org/project/keyctl/>`_ uses subprocess instead of system call, which introduces
possible points of failure and requires keyctl utility.

This package uses rust and PyO3 to make system calls directly to the kernel.

############
Usage
############

Use following code snippet for inspiration::

  from python_linux_keyutils import get_secret, set_secret, invalidate_secret, KeyRingIdentifier

  # By default, Session keyring is used
  set_secret("test_key", b"test value")
  print(get_secret("test_key"))
  # b'test value'

  # You can also specify a different keyring
  set_secret("test_key_2", b"\0\0\0", key_ring=KeyRingIdentifier.User)
  print(get_secret("test_key_2", key_ring=KeyRingIdentifier.User))
  # b'\x00\x00\x00'

  # set_secret doesn't automatically create keyring if it doesn't exist, but this can be changed with
  # `create` keyword argument
  set_secret("test_key_3", b"Hello kernel secrets", key_ring=KeyRingIdentifier.Process)
  # Raises KeyError
  set_secret("test_key_3", b"Hello kernel secrets", key_ring=KeyRingIdentifier.Process, create=True)
  get_secret("test_key_3", key_ring=KeyRingIdentifier.Process)
  # b'Hello kernel secrets'

**********
Exceptions
**********

The module may raise following exceptions

- **OSError**: If system call fails due to access being denied, quota exceeded, bad address, write error, etc.
- **ValueError**: If key name is invalid
- **KeyError**: If key doesn't exist, or is expired, or keyring doesn't exist
- **MemoryError**: If memory allocation fails
- **RuntimeError**: If underlying rust library reports that operation is not supported

############
Contributing
############

Contributions are what make the open source community such an amazing place to learn, inspire, and create.
Any contributions you make are greatly appreciated.

**********
Developing
**********

See `maturin documentation https://github.com/PyO3/maturin` for more information on how to run the project locally
in development mode.

**********
Opening MR
**********

1. Clone the Project
2. Create your Feature Branch (``git checkout -b feature/AmazingFeature``)
3. Commit your Changes (``git commit -m 'Add some AmazingFeature'``)
4. Push to the Branch (``git push origin feature/AmazingFeature``)
5. Open a Merge Request

#######
License
#######

Distributed under the MIT License. See LICENSE for more information.
