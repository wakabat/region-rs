use crate::{Protection, Region, Result};

pub fn page_size() -> usize {
  4096
}

pub unsafe fn alloc(_base: *const (), _size: usize, _protection: Protection) -> Result<*const ()> {
  unimplemented!()
}

pub unsafe fn free(_base: *const (), _size: usize) -> Result<()> {
  unimplemented!()
}

pub unsafe fn protect(_base: *const (), _size: usize, _protection: Protection) -> Result<()> {
  // Dummy operation
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
