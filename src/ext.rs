use std::ptr;
use crate::util::Binding;
use crate::{raw, Odb, Repository, Error};

/// Function to get the raw repository pointer
pub fn repository_raw(repo: &Repository) -> *mut raw::git_repository {
    repo.raw()
}

/// Function to get the raw ODB pointer from a repository
pub fn repository_odb_raw(repo: &Repository) -> Result<*mut raw::git_odb, Error> {
    let mut odb = ptr::null_mut();
    unsafe {
        let error = raw::git_repository_odb(&mut odb, repo.raw());
        if error < 0 {
            return Err(Error::last_error(error));
        }
        Ok(odb)
    }
}

/// Function to get the raw ODB pointer
pub fn odb_raw(odb: &Odb<'_>) -> *mut raw::git_odb {
    odb.raw()
}