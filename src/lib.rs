use std::{iter::Skip, str::Chars};

mod tests;

pub fn interpreter(code: &str, iterations: usize, w: usize, h: usize) -> String {
  let mut res = vec![vec![false; w]; h];
  let mut ptr = (0usize, 0usize);
  let mut code_chars = code.chars().skip(0);
  let mut jump_offsets = vec![];
  let mut idx = 0;

  for _ in 0..iterations {
    let ins = next_instruction(&mut code_chars, &mut idx);
    match ins {
      'n' => ptr.0 = shift(ptr.0, false, h),
      's' => ptr.0 = shift(ptr.0, true, h),
      'e' => ptr.1 = shift(ptr.1, true, w),
      'w' => ptr.1 = shift(ptr.1, false, w),
      '*' => res[ptr.0][ptr.1] = !res[ptr.0][ptr.1],
      '[' if !res[ptr.0][ptr.1] => {
        code_chars.position(|c| c == ']');
      }
      ']' if res[ptr.0][ptr.1] => {
        let jmp = jump_offsets.pop().unwrap_or(idx);
        code_chars = code.chars().skip(jmp);
        code_chars.next();
        idx = jmp;
      }
      '[' => jump_offsets.push(idx),
      _ => (),
    }

    idx -= 1;
  }

  display(res)
}

fn next_instruction(code: &mut Skip<Chars>, idx: &mut usize) -> char {
  *idx += 1;
  while let Some(ins) = code.next() {
    match ins {
      'n' | 's' | 'e' | 'w' | '*' | '[' | ']' => return ins,
      _ => println!("unknown: {}", ins),
    }
    *idx += 1;
  }
  '\0'
}

fn shift(v: usize, right: bool, max: usize) -> usize {
  if right {
    return if v + 1 < max { v + 1 } else { 0 };
  } else {
    return if v > 0 { v - 1 } else { max - 1 };
  }
}

fn display(v: Vec<Vec<bool>>) -> String {
  let mut res = String::new();
  for i in v {
    for j in i {
      match j {
        true => res.push('1'),
        false => res.push('0'),
      }
    }
    res.push_str("\r\n");
  }
  res.pop();
  res.pop();
  res
}
