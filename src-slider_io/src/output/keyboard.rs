use interception::{Interception, KeyState, ScanCode, Stroke};
use log::{error, info};
use std::mem;
use winapi::{
  ctypes::c_int,
  um::winuser::{
    MapVirtualKeyA, SendInput, INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, MAPVK_VK_TO_VSC,
  },
};

use super::{config::KeyboardLayout, output::OutputHandler};

#[rustfmt::skip]
const TASOLLER_KB_MAP: [usize; 41] = [
    0x41 /* A */, 0x31 /* 1 */, 0x5a /* Z */, 0x51 /* Q */, 0x53 /* S */, 0x32 /* 2 */, 0x58 /* X */, 0x57 /* W */, 
    0x44 /* D */, 0x33 /* 3 */, 0x43 /* C */, 0x45 /* E */, 0x46 /* F */, 0x34 /* 4 */, 0x56 /* V */, 0x52 /* R */,
    0x47 /* G */, 0x35 /* 5 */, 0x42 /* B */, 0x54 /* T */, 0x48 /* H */, 0x36 /* 6 */, 0x4e /* N */, 0x59 /* Y */,
    0x4a /* J */, 0x37 /* 7 */, 0x4d /* M */, 0x55 /* U */, 0x4b /* K */, 0x38 /* 8 */, 0xbc /* VK_OEM_COMMA */, 0x49 /* I */,
    0xbf, 0xde, 0xbe, // VK_OEM_2, VK_OEM_7, VK_OEM_PERIOD,
    0xba, 0xdd, 0xdb, // VK_OEM_1, VK_OEM_6, VK_OEM_4
    0x0d, 0x20, 0x1b  // VK_RETURN, VK_SPACE, VK_ESCAPE
];

#[rustfmt::skip]
const YUANCON_KB_MAP: [usize; 41] = [
    0x36 /* 6 */, 0x35 /* 5 */, 0x34 /* 4 */, 0x33 /* 3 */, 0x32 /* 2 */, 0x31 /* 1 */, 0x5a /* Z */, 0x59 /* Y */, 
    0x58 /* X */, 0x57 /* W */, 0x56 /* V */, 0x55 /* U */, 0x54 /* T */, 0x53 /* S */, 0x52 /* R */, 0x51 /* Q */,
    0x50 /* P */, 0x4f /* O */, 0x4e /* N */, 0x4d /* M */, 0x4c /* L */, 0x4b /* K */, 0x4a /* J */, 0x49 /* I */,
    0x48 /* H */, 0x47 /* G */, 0x46 /* F */, 0x45 /* E */, 0x44 /* D */, 0x43 /* C */, 0x42 /* B */, 0x41 /* A */,
    0xbd, 0xbb, 0xdb, // VK_OEM_MINUS, VK_OEM_PLUS, VK_OEM_4,
    0xdd, 0xdc, 0xba, // VK_OEM_6, VK_OEM_5, VK_OEM_1,
    0x0d, 0x20, 0x1b, // VK_RETURN, VK_SPACE, VK_ESCAPE
];

#[rustfmt::skip]
const UMIGURI_KB_MAP: [usize; 41] = [
    0x41 /* A */, 0x31 /* 1 */, 0x5a /* Z */, 0x51 /* Q */, 0x53 /* S */, 0x32 /* 2 */, 0x58 /* X */, 0x57 /* W */, 
    0x44 /* D */, 0x33 /* 3 */, 0x43 /* C */, 0x45 /* E */, 0x46 /* F */, 0x34 /* 4 */, 0x56 /* V */, 0x52 /* R */,
    0x47 /* G */, 0x35 /* 5 */, 0x42 /* B */, 0x54 /* T */, 0x48 /* H */, 0x36 /* 6 */, 0x4e /* N */, 0x59 /* Y */,
    0x4a /* J */, 0x37 /* 7 */, 0x4d /* M */, 0x55 /* U */, 0x4b /* K */, 0x38 /* 8 */, 0x39 /* 9 */, 0x49 /* I */,
    0x30, 0x4f, 0x4c, // 0, O, L
    0x50, 0xbc, 0xbe, // P, VK_OEM_COMMA, VK_OEM_PERIOD,
    0x0d, 0x20, 0x1b  // VK_RETURN, VK_SPACE, VK_ESCAPE
];
#[rustfmt::skip]
const PDFTA_KB_MAP: [usize; 41] = [
    0x5a /* Z */, 0x50 /* P */, 
    0x5a /* Z */, 0x50 /* P */, 
    0x5a /* Z */, 0x4F /* O */, 
    0x5a /* Z */, 0x4F /* O */, 
    0x58 /* X */, 0x49 /* I */, 
    0x58 /* X */, 0x49 /* I */, 
    0x58 /* X */, 0x55 /* U */, 
    0x58 /* X */, 0x55 /* U */,
    0x43 /* C */, 0x52 /* R */, 
    0x43 /* C */, 0x52 /* R */, 
    0x43 /* C */, 0x45 /* E */, 
    0x43 /* C */, 0x45 /* E */,
    0x56 /* V */, 0x57 /* W */, 
    0x56 /* V */, 0x57 /* W */, 
    0x56 /* V */, 0x51 /* Q */, 
    0x56 /* V */, 0x51 /* Q */,
    0x1b, 0x1b, 0x1b, 0x1b, 0x1b, 0x1b, 0x1b, 0x1b, 0x1b  // VK_ESCAPE
];

#[rustfmt::skip]
const TASOLLER_HALF_KB_MAP: [usize; 41] = [
  0x41, 0x41 /* A */, 0x5a, 0x5a /* Z */, 0x53, 0x53 /* S */, 0x58, 0x58 /* X */, 
  0x44, 0x44 /* D */, 0x43, 0x43 /* C */, 0x46, 0x46 /* F */, 0x56, 0x56 /* V */,
  0x47, 0x47 /* G */, 0x42, 0x42 /* B */, 0x48, 0x48 /* H */, 0x4e, 0x4e /* N */,
  0x4a, 0x4a /* J */, 0x4d, 0x4d /* M */, 0x4b, 0x4b /* K */, 0xbc, 0xbc /* VK_OEM_COMMA */,
  0xbf, 0xde, 0xbe, // VK_OEM_2, VK_OEM_7, VK_OEM_PERIOD,
  0xba, 0xdd, 0xdb, // VK_OEM_1, VK_OEM_6, VK_OEM_4
  0x0d, 0x20, 0x1b  // VK_RETURN, VK_SPACE, VK_ESCAPE
];

#[rustfmt::skip]
const EIGHT_K_MAP: [usize; 41] = [
  0x41, 0x41, 0x41, 0x41, // A
  0x53, 0x53, 0x53, 0x53, // S
  0x44, 0x44, 0x44, 0x44, // D
  0x46, 0x46, 0x46, 0x46, // F
  0x4a, 0x4a, 0x4a, 0x4a, // J
  0x4b, 0x4b, 0x4b, 0x4b, // K
  0x4c, 0x4c, 0x4c, 0x4c, // L
  0xba, 0xba, 0xba, 0xba, // VK_OEM_1
  0x20, 0x20, 0x20, 0x20, 0x20, 0x20, // VK_SPACE
  0x00, 0x00, 0x00, // Disabled
];

#[rustfmt::skip]
const SIX_K_MAP: [usize; 41] = [
  0x53, 0x53, 0x53, 0x53, 0x53, 0x53, // S
  0x44, 0x44, 0x44, 0x44, // D
  0x46, 0x46, 0x46, 0x46, 0x46, 0x46, // F
  0x4a, 0x4a, 0x4a, 0x4a, 0x4a, 0x4a, // J
  0x4b, 0x4b, 0x4b, 0x4b, // K
  0x4c, 0x4c, 0x4c, 0x4c, 0x4c, 0x4c, // L
  0x20, 0x20, 0x20, 0x20, 0x20, 0x20, // VK_SPACE
  0x00, 0x00, 0x00, // Disabled
];

#[rustfmt::skip]
const FOUR_K_MAP: [usize; 41] = [
  0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, // D
  0x46, 0x46, 0x46, 0x46, 0x46, 0x46, 0x46, 0x46, // F
  0x4a, 0x4a, 0x4a, 0x4a, 0x4a, 0x4a, 0x4a, 0x4a, // J
  0x4b, 0x4b, 0x4b, 0x4b, 0x4b, 0x4b, 0x4b, 0x4b, // K
  0x20, 0x20, 0x20, 0x20, 0x20, 0x20, // VK_SPACE
  0x00, 0x00, 0x00, // Disabled
];

#[rustfmt::skip]
const VOLTEX_KB_MAP: [usize; 41] = [
  0x57, 0x57, 0x57, 0x57, // W
  0x45, 0x45, 0x45, 0x45, // E
  0x43, 0x44,
  0x43, 0x44,
  0x43, 0x46,      // D
  0x43, 0x46, // C // F
  0x4d, 0x4a, // M // J
  0x4d, 0x4a,      // K
  0x4d, 0x4b,
  0x4d, 0x4b,
  0x4f, 0x4f, 0x4f, 0x4f, // O
  0x50, 0x50, 0x50, 0x50, // P
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Disabled
  0x31, 0x0d, 0x1b, // 1, VK_RETURN, VK_ESCAPE
];

#[rustfmt::skip]
const VOLTEX_KB_MAP_NEARDAYO: [usize; 41] = [
  0x57, 0x57, 0x57, 0x57, // W
  0x45, 0x45, 0x45, 0x45, // E
  0x43, 0x44,
  0x43, 0x44,
  0x43, 0x46,      // D
  0x43, 0x46, // C // F
  0x4d, 0x4a, // M // J
  0x4d, 0x4a,      // K
  0x4d, 0x4b,
  0x4d, 0x4b,
  0x4f, 0x4f, 0x4f, 0x4f, // O
  0x50, 0x50, 0x50, 0x50, // P
  0x57, 0x45, 0x45, 0x4f, 0x4f, 0x50, // Disabled
  0x31, 0x0d, 0x1b, // 1, VK_RETURN, VK_ESCAPE
];

pub struct KeyboardOutput {
  input_to_idx: [usize; 41],
  key_idx_to_keycode: [u16; 41],
  key_idx_to_scancode: [Option<ScanCode>; 41],
  next_keys: [bool; 41],
  last_keys: [bool; 41],

  direct_input: bool,
  interception_handle: Option<Interception>,

  kb_buf: [INPUT; 41],
  kb_direct_buf: [Stroke; 41],
  n_kb_buf: u32,
}

// interception isn't send, but lazy to wrap
unsafe impl Send for KeyboardOutput {}

impl KeyboardOutput {
  pub fn new(layout: KeyboardLayout, direct_input: bool) -> Self {
    let kb_map = match layout {
      KeyboardLayout::Tasoller => &TASOLLER_KB_MAP,
      KeyboardLayout::Yuancon => &YUANCON_KB_MAP,
      KeyboardLayout::Umiguri => &UMIGURI_KB_MAP,
      KeyboardLayout::PDFTA => &PDFTA_KB_MAP,
      KeyboardLayout::TasollerHalf => &TASOLLER_HALF_KB_MAP,
      KeyboardLayout::EightK => &EIGHT_K_MAP,
      KeyboardLayout::SixK => &SIX_K_MAP,
      KeyboardLayout::FourK => &FOUR_K_MAP,
      KeyboardLayout::Voltex => &VOLTEX_KB_MAP,
      KeyboardLayout::Neardayo => &VOLTEX_KB_MAP_NEARDAYO,
    };

    let mut input_to_key_idx = [0 as usize; 41];
    let mut key_idx_to_keycode = [0 as u16; 41];
    let mut key_idx_to_scancode = [None as Option<ScanCode>; 41];
    let mut keycode_to_idx = [0xffff as usize; 256];
    let mut keycode_count: usize = 0;

    for (ground, keycode) in kb_map.iter().enumerate() {
      if keycode_to_idx[*keycode] == 0xffff {
        keycode_to_idx[*keycode] = keycode_count;
        key_idx_to_keycode[keycode_count] = *keycode as u16;
        key_idx_to_scancode[keycode_count] =
          ScanCode::try_from(unsafe { MapVirtualKeyA((*keycode) as u32, MAPVK_VK_TO_VSC) as u16 })
            .ok();
        // info!(
        //   "mapped {:?} to {:?}",
        //   key_idx_to_keycode[keycode_count], key_idx_to_scancode[keycode_count]
        // );
        keycode_count += 1;
      }
      input_to_key_idx[ground] = keycode_to_idx[*keycode]
    }

    let interception_handle = match direct_input {
      true => {
        let inner_handle = Interception::new();

        if inner_handle.is_some() {
          info!("Keyboard emulation with interception loaded");
        } else {
          error!("Keyboard emulation cannot load interception, falling back to SendKeys()");
        }
        inner_handle
      }
      false => None,
    };
    let direct_input = interception_handle.is_some();

    let mut kb_buf = [INPUT {
      type_: INPUT_KEYBOARD,
      u: unsafe { mem::zeroed() },
    }; 41];

    for i in kb_buf.iter_mut() {
      let mut inner = unsafe { i.u.ki_mut() };
      inner.wVk = 0;
      inner.wScan = 0;
      inner.dwFlags = 0;
      inner.time = 0;
      inner.dwExtraInfo = 0;
    }

    let kb_direct_buf = [Stroke::Keyboard {
      code: ScanCode::Esc,
      state: KeyState::UP,
      information: 0,
    }; 41];

    Self {
      input_to_idx: input_to_key_idx,
      key_idx_to_keycode,
      key_idx_to_scancode,
      next_keys: [false; 41],
      last_keys: [false; 41],

      direct_input,
      interception_handle,

      kb_buf,
      kb_direct_buf,
      n_kb_buf: 0,
    }
  }

  fn send(&mut self) {
    self.n_kb_buf = 0;

    for (i, (n, l)) in self
      .next_keys
      .iter_mut()
      .zip(self.last_keys.iter_mut())
      .enumerate()
    {
      let keycode = self.key_idx_to_keycode[i];
      let scancode = self.key_idx_to_scancode[i];

      if (!self.direct_input && keycode == 0) || (self.direct_input && scancode.is_none()) {
        continue;
      }
      match (self.direct_input, *n, *l) {
        (false, true, false) => {
          let inner: &mut KEYBDINPUT = unsafe { self.kb_buf[self.n_kb_buf as usize].u.ki_mut() };
          inner.wVk = keycode;
          inner.dwFlags = 0;
          self.n_kb_buf += 1;
        }
        (false, false, true) => {
          let inner: &mut KEYBDINPUT = unsafe { self.kb_buf[self.n_kb_buf as usize].u.ki_mut() };
          inner.wVk = keycode;
          inner.dwFlags = KEYEVENTF_KEYUP;
          self.n_kb_buf += 1;
        }
        (true, true, false) => {
          // info!("keydown {:?}", scancode);
          let inner: &mut Stroke = &mut self.kb_direct_buf[self.n_kb_buf as usize];
          if let Stroke::Keyboard {
            code,
            state,
            information: _,
          } = inner
          {
            *code = scancode.unwrap();
            *state = KeyState::DOWN;
            self.n_kb_buf += 1;
          }
        }
        (true, false, true) => {
          // info!("keyup {:?}", scancode);
          let inner: &mut Stroke = &mut self.kb_direct_buf[self.n_kb_buf as usize];
          if let Stroke::Keyboard {
            code,
            state,
            information: _,
          } = inner
          {
            *code = scancode.unwrap();
            *state = KeyState::UP;
            self.n_kb_buf += 1;
          }
        }
        _ => {}
      }
      *l = *n;
    }

    if self.n_kb_buf != 0 {
      match self.direct_input {
        false => unsafe {
          SendInput(
            self.n_kb_buf,
            self.kb_buf.as_mut_ptr(),
            mem::size_of::<INPUT>() as c_int,
          );
        },
        true => {
          if let Some(handle) = self.interception_handle.as_mut() {
            handle.send(1, &self.kb_direct_buf[0..self.n_kb_buf as usize]);
          }
        }
      }
    }
  }
}

impl OutputHandler for KeyboardOutput {
  fn tick(&mut self, flat_input: &Vec<bool>) -> bool {
    self.next_keys.fill(false);
    for (idx, x) in flat_input.iter().enumerate() {
      if *x {
        self.next_keys[self.input_to_idx[idx]] = true;
      }
    }
    self.send();
    true
  }

  fn reset(&mut self) {
    self.next_keys.fill(false);
    self.send();
  }
}

impl Drop for KeyboardOutput {
  fn drop(&mut self) {
    self.reset();
  }
}
