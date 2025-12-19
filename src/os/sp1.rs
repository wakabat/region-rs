use crate::{Protection, Region, Result};

use sp1_primitives::consts::{PAGE_SIZE, PROT_EXEC, PROT_NONE, PROT_READ, PROT_WRITE};
use sp1_zkvm::lib::mprotect::mprotect;

pub fn page_size() -> usize {
  PAGE_SIZE
}

fn convert(protection: Protection) -> u8 {
  let mut r = PROT_NONE;
  if protection & Protection::READ == Protection::READ {
    r |= PROT_READ;
  }
  if protection & Protection::WRITE == Protection::WRITE {
    r |= PROT_WRITE;
  }
  if protection & Protection::EXECUTE == Protection::EXECUTE {
    r |= PROT_EXEC;
  }
  r
}

// For our use case, we are ignoring base address
pub unsafe fn alloc(_base: *const (), size: usize, protection: Protection) -> Result<*const ()> {
  let layout = std::alloc::Layout::from_size_align(size, page_size()).unwrap();
  let p = unsafe { std::alloc::alloc(layout) };
  mprotect(p as _, size, convert(protection));
  Ok(p as _)
}

pub unsafe fn free(base: *const (), size: usize) -> Result<()> {
  mprotect(base as _, size, PROT_READ | PROT_WRITE);
  let layout = std::alloc::Layout::from_size_align(size, page_size()).unwrap();
  unsafe { std::alloc::dealloc(base as _, layout) };
  Ok(())
}

pub unsafe fn protect(base: *const (), size: usize, protection: Protection) -> Result<()> {
  mprotect(base as _, size, convert(protection));
  Ok(())
}

pub fn lock(_base: *const (), _size: usize) -> Result<()> {
  // Dummy operation
  Ok(())
}

pub fn unlock(_base: *const (), _size: usize) -> Result<()> {
  // Dummy operation
  Ok(())
}

pub struct QueryIter {}

impl QueryIter {
  pub fn new(_origin: *const (), _size: usize) -> Result<Self> {
    Ok(QueryIter {})
  }

  pub fn upper_bound(&self) -> usize {
    usize::MAX
  }
}

impl Iterator for QueryIter {
  type Item = Result<Region>;

  fn next(&mut self) -> Option<Self::Item> {
    None
  }
}
