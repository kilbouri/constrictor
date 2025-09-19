type Hook = fn() -> ();

pub struct ScopeGuard(Hook);

impl ScopeGuard {
    pub fn new(f: Hook) -> Self {
        Self(f)
    }
}

impl Drop for ScopeGuard {
    fn drop(&mut self) {
        (self.0)()
    }
}
