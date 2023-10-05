/// Raw display handle for the Sony Playstation Vita operating system.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VitaDisplayHandle {}

impl VitaDisplayHandle {
    pub fn empty() -> Self {
        Self {}
    }

    /// Create a new empty display handle.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use raw_window_handle::VitaDisplayHandle;
    /// let handle = VitaDisplayHandle::new();
    /// ```
    pub fn new() -> Self {
        Self {}
    }
}

/// Raw window handle for the Redox operating system.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VitaWindowHandle {}

impl VitaWindowHandle {
    pub fn empty() -> Self {
        Self {}
    }

    /// Create a new handle to a window.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use raw_window_handle::VitaWindowHandle;
    /// let mut handle = VitaWindowHandle::new();
    /// ```
    pub fn new() -> Self {
        Self {}
    }
}
