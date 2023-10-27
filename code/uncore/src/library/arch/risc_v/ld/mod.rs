extern "C" {
  static _sheap: u8;
  static _heap_size: usize;
}

pub fn get_heap_bottom_and_size() -> (*const u8, usize) {
  unsafe {
    (&_sheap as *const u8, &_heap_size as *const usize as usize)
  }
}
