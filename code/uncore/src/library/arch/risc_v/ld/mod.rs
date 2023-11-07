extern "C" {
  static __heap__start: u8;
  static __heap__size: u8;
}

pub fn get_heap_bottom_and_size() -> (*const u8, usize) {
  unsafe {
    (&__heap__start as *const u8, &__heap__size as *const u8 as usize)
  }
}
