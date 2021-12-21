#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![feature(asm)]

mod custom;

#[skyline::main(name = "MonadoArts")]
pub fn main() {
  custom::install();
}
