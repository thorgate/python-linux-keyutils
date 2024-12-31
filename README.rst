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

Existing `keyring https://pypi.org/project/keyring/` package is very powerful, but somewhat complex
and heavy.

`keyctl https://pypi.org/project/keyctl/` uses subprocess instead of system call, which introduces
possible points of failure and requires keyctl utility.

This package uses rust and PyO3 to make system calls directly to the kernel.

############
Usage
############

Use following code snippet for inspiration::

  import python_linux_keyutils

  python_linux_keyutils.set_session_secret("secret_name","secret_value")
  python_linux_keyutils.get_session_secret("secret_name")
  python_linux_keyutils.invalidate_session_secret("secret_name")


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
