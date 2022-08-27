// automatically generated by the FlatBuffers compiler, do not modify
// @generated
extern crate alloc;
extern crate flatbuffers;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::mem;
use core::cmp::Ordering;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
pub enum ScalarStuffOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct ScalarStuff<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for ScalarStuff<'a> {
  type Inner = ScalarStuff<'a>;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table { buf, loc } }
  }
}

impl<'a> ScalarStuff<'a> {
  pub const VT_JUST_I8: flatbuffers::VOffsetT = 4;
  pub const VT_MAYBE_I8: flatbuffers::VOffsetT = 6;
  pub const VT_DEFAULT_I8: flatbuffers::VOffsetT = 8;
  pub const VT_JUST_U8: flatbuffers::VOffsetT = 10;
  pub const VT_MAYBE_U8: flatbuffers::VOffsetT = 12;
  pub const VT_DEFAULT_U8: flatbuffers::VOffsetT = 14;
  pub const VT_JUST_I16: flatbuffers::VOffsetT = 16;
  pub const VT_MAYBE_I16: flatbuffers::VOffsetT = 18;
  pub const VT_DEFAULT_I16: flatbuffers::VOffsetT = 20;
  pub const VT_JUST_U16: flatbuffers::VOffsetT = 22;
  pub const VT_MAYBE_U16: flatbuffers::VOffsetT = 24;
  pub const VT_DEFAULT_U16: flatbuffers::VOffsetT = 26;
  pub const VT_JUST_I32: flatbuffers::VOffsetT = 28;
  pub const VT_MAYBE_I32: flatbuffers::VOffsetT = 30;
  pub const VT_DEFAULT_I32: flatbuffers::VOffsetT = 32;
  pub const VT_JUST_U32: flatbuffers::VOffsetT = 34;
  pub const VT_MAYBE_U32: flatbuffers::VOffsetT = 36;
  pub const VT_DEFAULT_U32: flatbuffers::VOffsetT = 38;
  pub const VT_JUST_I64: flatbuffers::VOffsetT = 40;
  pub const VT_MAYBE_I64: flatbuffers::VOffsetT = 42;
  pub const VT_DEFAULT_I64: flatbuffers::VOffsetT = 44;
  pub const VT_JUST_U64: flatbuffers::VOffsetT = 46;
  pub const VT_MAYBE_U64: flatbuffers::VOffsetT = 48;
  pub const VT_DEFAULT_U64: flatbuffers::VOffsetT = 50;
  pub const VT_JUST_F32: flatbuffers::VOffsetT = 52;
  pub const VT_MAYBE_F32: flatbuffers::VOffsetT = 54;
  pub const VT_DEFAULT_F32: flatbuffers::VOffsetT = 56;
  pub const VT_JUST_F64: flatbuffers::VOffsetT = 58;
  pub const VT_MAYBE_F64: flatbuffers::VOffsetT = 60;
  pub const VT_DEFAULT_F64: flatbuffers::VOffsetT = 62;
  pub const VT_JUST_BOOL: flatbuffers::VOffsetT = 64;
  pub const VT_MAYBE_BOOL: flatbuffers::VOffsetT = 66;
  pub const VT_DEFAULT_BOOL: flatbuffers::VOffsetT = 68;
  pub const VT_JUST_ENUM: flatbuffers::VOffsetT = 70;
  pub const VT_MAYBE_ENUM: flatbuffers::VOffsetT = 72;
  pub const VT_DEFAULT_ENUM: flatbuffers::VOffsetT = 74;

  pub const fn get_fully_qualified_name() -> &'static str {
    "optional_scalars.ScalarStuff"
  }

  #[inline]
  pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    ScalarStuff { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args ScalarStuffArgs
  ) -> flatbuffers::WIPOffset<ScalarStuff<'bldr>> {
    let mut builder = ScalarStuffBuilder::new(_fbb);
    builder.add_default_f64(args.default_f64);
    if let Some(x) = args.maybe_f64 { builder.add_maybe_f64(x); }
    builder.add_just_f64(args.just_f64);
    builder.add_default_u64(args.default_u64);
    if let Some(x) = args.maybe_u64 { builder.add_maybe_u64(x); }
    builder.add_just_u64(args.just_u64);
    builder.add_default_i64(args.default_i64);
    if let Some(x) = args.maybe_i64 { builder.add_maybe_i64(x); }
    builder.add_just_i64(args.just_i64);
    builder.add_default_f32(args.default_f32);
    if let Some(x) = args.maybe_f32 { builder.add_maybe_f32(x); }
    builder.add_just_f32(args.just_f32);
    builder.add_default_u32(args.default_u32);
    if let Some(x) = args.maybe_u32 { builder.add_maybe_u32(x); }
    builder.add_just_u32(args.just_u32);
    builder.add_default_i32(args.default_i32);
    if let Some(x) = args.maybe_i32 { builder.add_maybe_i32(x); }
    builder.add_just_i32(args.just_i32);
    builder.add_default_u16(args.default_u16);
    if let Some(x) = args.maybe_u16 { builder.add_maybe_u16(x); }
    builder.add_just_u16(args.just_u16);
    builder.add_default_i16(args.default_i16);
    if let Some(x) = args.maybe_i16 { builder.add_maybe_i16(x); }
    builder.add_just_i16(args.just_i16);
    builder.add_default_enum(args.default_enum);
    if let Some(x) = args.maybe_enum { builder.add_maybe_enum(x); }
    builder.add_just_enum(args.just_enum);
    builder.add_default_bool(args.default_bool);
    if let Some(x) = args.maybe_bool { builder.add_maybe_bool(x); }
    builder.add_just_bool(args.just_bool);
    builder.add_default_u8(args.default_u8);
    if let Some(x) = args.maybe_u8 { builder.add_maybe_u8(x); }
    builder.add_just_u8(args.just_u8);
    builder.add_default_i8(args.default_i8);
    if let Some(x) = args.maybe_i8 { builder.add_maybe_i8(x); }
    builder.add_just_i8(args.just_i8);
    builder.finish()
  }

  pub fn unpack(&self) -> ScalarStuffT {
    let just_i8 = self.just_i8();
    let maybe_i8 = self.maybe_i8();
    let default_i8 = self.default_i8();
    let just_u8 = self.just_u8();
    let maybe_u8 = self.maybe_u8();
    let default_u8 = self.default_u8();
    let just_i16 = self.just_i16();
    let maybe_i16 = self.maybe_i16();
    let default_i16 = self.default_i16();
    let just_u16 = self.just_u16();
    let maybe_u16 = self.maybe_u16();
    let default_u16 = self.default_u16();
    let just_i32 = self.just_i32();
    let maybe_i32 = self.maybe_i32();
    let default_i32 = self.default_i32();
    let just_u32 = self.just_u32();
    let maybe_u32 = self.maybe_u32();
    let default_u32 = self.default_u32();
    let just_i64 = self.just_i64();
    let maybe_i64 = self.maybe_i64();
    let default_i64 = self.default_i64();
    let just_u64 = self.just_u64();
    let maybe_u64 = self.maybe_u64();
    let default_u64 = self.default_u64();
    let just_f32 = self.just_f32();
    let maybe_f32 = self.maybe_f32();
    let default_f32 = self.default_f32();
    let just_f64 = self.just_f64();
    let maybe_f64 = self.maybe_f64();
    let default_f64 = self.default_f64();
    let just_bool = self.just_bool();
    let maybe_bool = self.maybe_bool();
    let default_bool = self.default_bool();
    let just_enum = self.just_enum();
    let maybe_enum = self.maybe_enum();
    let default_enum = self.default_enum();
    ScalarStuffT {
      just_i8,
      maybe_i8,
      default_i8,
      just_u8,
      maybe_u8,
      default_u8,
      just_i16,
      maybe_i16,
      default_i16,
      just_u16,
      maybe_u16,
      default_u16,
      just_i32,
      maybe_i32,
      default_i32,
      just_u32,
      maybe_u32,
      default_u32,
      just_i64,
      maybe_i64,
      default_i64,
      just_u64,
      maybe_u64,
      default_u64,
      just_f32,
      maybe_f32,
      default_f32,
      just_f64,
      maybe_f64,
      default_f64,
      just_bool,
      maybe_bool,
      default_bool,
      just_enum,
      maybe_enum,
      default_enum,
    }
  }

  #[inline]
  pub fn just_i8(&self) -> i8 {
    self._tab.get::<i8>(ScalarStuff::VT_JUST_I8, Some(0)).unwrap()
  }
  #[inline]
  pub fn maybe_i8(&self) -> Option<i8> {
    self._tab.get::<i8>(ScalarStuff::VT_MAYBE_I8, None)
  }
  #[inline]
  pub fn default_i8(&self) -> i8 {
    self._tab.get::<i8>(ScalarStuff::VT_DEFAULT_I8, Some(42)).unwrap()
  }
  #[inline]
  pub fn just_u8(&self) -> u8 {
    self._tab.get::<u8>(ScalarStuff::VT_JUST_U8, Some(0)).unwrap()
  }
  #[inline]
  pub fn maybe_u8(&self) -> Option<u8> {
    self._tab.get::<u8>(ScalarStuff::VT_MAYBE_U8, None)
  }
  #[inline]
  pub fn default_u8(&self) -> u8 {
    self._tab.get::<u8>(ScalarStuff::VT_DEFAULT_U8, Some(42)).unwrap()
  }
  #[inline]
  pub fn just_i16(&self) -> i16 {
    self._tab.get::<i16>(ScalarStuff::VT_JUST_I16, Some(0)).unwrap()
  }
  #[inline]
  pub fn maybe_i16(&self) -> Option<i16> {
    self._tab.get::<i16>(ScalarStuff::VT_MAYBE_I16, None)
  }
  #[inline]
  pub fn default_i16(&self) -> i16 {
    self._tab.get::<i16>(ScalarStuff::VT_DEFAULT_I16, Some(42)).unwrap()
  }
  #[inline]
  pub fn just_u16(&self) -> u16 {
    self._tab.get::<u16>(ScalarStuff::VT_JUST_U16, Some(0)).unwrap()
  }
  #[inline]
  pub fn maybe_u16(&self) -> Option<u16> {
    self._tab.get::<u16>(ScalarStuff::VT_MAYBE_U16, None)
  }
  #[inline]
  pub fn default_u16(&self) -> u16 {
    self._tab.get::<u16>(ScalarStuff::VT_DEFAULT_U16, Some(42)).unwrap()
  }
  #[inline]
  pub fn just_i32(&self) -> i32 {
    self._tab.get::<i32>(ScalarStuff::VT_JUST_I32, Some(0)).unwrap()
  }
  #[inline]
  pub fn maybe_i32(&self) -> Option<i32> {
    self._tab.get::<i32>(ScalarStuff::VT_MAYBE_I32, None)
  }
  #[inline]
  pub fn default_i32(&self) -> i32 {
    self._tab.get::<i32>(ScalarStuff::VT_DEFAULT_I32, Some(42)).unwrap()
  }
  #[inline]
  pub fn just_u32(&self) -> u32 {
    self._tab.get::<u32>(ScalarStuff::VT_JUST_U32, Some(0)).unwrap()
  }
  #[inline]
  pub fn maybe_u32(&self) -> Option<u32> {
    self._tab.get::<u32>(ScalarStuff::VT_MAYBE_U32, None)
  }
  #[inline]
  pub fn default_u32(&self) -> u32 {
    self._tab.get::<u32>(ScalarStuff::VT_DEFAULT_U32, Some(42)).unwrap()
  }
  #[inline]
  pub fn just_i64(&self) -> i64 {
    self._tab.get::<i64>(ScalarStuff::VT_JUST_I64, Some(0)).unwrap()
  }
  #[inline]
  pub fn maybe_i64(&self) -> Option<i64> {
    self._tab.get::<i64>(ScalarStuff::VT_MAYBE_I64, None)
  }
  #[inline]
  pub fn default_i64(&self) -> i64 {
    self._tab.get::<i64>(ScalarStuff::VT_DEFAULT_I64, Some(42)).unwrap()
  }
  #[inline]
  pub fn just_u64(&self) -> u64 {
    self._tab.get::<u64>(ScalarStuff::VT_JUST_U64, Some(0)).unwrap()
  }
  #[inline]
  pub fn maybe_u64(&self) -> Option<u64> {
    self._tab.get::<u64>(ScalarStuff::VT_MAYBE_U64, None)
  }
  #[inline]
  pub fn default_u64(&self) -> u64 {
    self._tab.get::<u64>(ScalarStuff::VT_DEFAULT_U64, Some(42)).unwrap()
  }
  #[inline]
  pub fn just_f32(&self) -> f32 {
    self._tab.get::<f32>(ScalarStuff::VT_JUST_F32, Some(0.0)).unwrap()
  }
  #[inline]
  pub fn maybe_f32(&self) -> Option<f32> {
    self._tab.get::<f32>(ScalarStuff::VT_MAYBE_F32, None)
  }
  #[inline]
  pub fn default_f32(&self) -> f32 {
    self._tab.get::<f32>(ScalarStuff::VT_DEFAULT_F32, Some(42.0)).unwrap()
  }
  #[inline]
  pub fn just_f64(&self) -> f64 {
    self._tab.get::<f64>(ScalarStuff::VT_JUST_F64, Some(0.0)).unwrap()
  }
  #[inline]
  pub fn maybe_f64(&self) -> Option<f64> {
    self._tab.get::<f64>(ScalarStuff::VT_MAYBE_F64, None)
  }
  #[inline]
  pub fn default_f64(&self) -> f64 {
    self._tab.get::<f64>(ScalarStuff::VT_DEFAULT_F64, Some(42.0)).unwrap()
  }
  #[inline]
  pub fn just_bool(&self) -> bool {
    self._tab.get::<bool>(ScalarStuff::VT_JUST_BOOL, Some(false)).unwrap()
  }
  #[inline]
  pub fn maybe_bool(&self) -> Option<bool> {
    self._tab.get::<bool>(ScalarStuff::VT_MAYBE_BOOL, None)
  }
  #[inline]
  pub fn default_bool(&self) -> bool {
    self._tab.get::<bool>(ScalarStuff::VT_DEFAULT_BOOL, Some(true)).unwrap()
  }
  #[inline]
  pub fn just_enum(&self) -> OptionalByte {
    self._tab.get::<OptionalByte>(ScalarStuff::VT_JUST_ENUM, Some(OptionalByte::None)).unwrap()
  }
  #[inline]
  pub fn maybe_enum(&self) -> Option<OptionalByte> {
    self._tab.get::<OptionalByte>(ScalarStuff::VT_MAYBE_ENUM, None)
  }
  #[inline]
  pub fn default_enum(&self) -> OptionalByte {
    self._tab.get::<OptionalByte>(ScalarStuff::VT_DEFAULT_ENUM, Some(OptionalByte::One)).unwrap()
  }
}

impl flatbuffers::Verifiable for ScalarStuff<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<i8>("just_i8", Self::VT_JUST_I8, false)?
     .visit_field::<i8>("maybe_i8", Self::VT_MAYBE_I8, false)?
     .visit_field::<i8>("default_i8", Self::VT_DEFAULT_I8, false)?
     .visit_field::<u8>("just_u8", Self::VT_JUST_U8, false)?
     .visit_field::<u8>("maybe_u8", Self::VT_MAYBE_U8, false)?
     .visit_field::<u8>("default_u8", Self::VT_DEFAULT_U8, false)?
     .visit_field::<i16>("just_i16", Self::VT_JUST_I16, false)?
     .visit_field::<i16>("maybe_i16", Self::VT_MAYBE_I16, false)?
     .visit_field::<i16>("default_i16", Self::VT_DEFAULT_I16, false)?
     .visit_field::<u16>("just_u16", Self::VT_JUST_U16, false)?
     .visit_field::<u16>("maybe_u16", Self::VT_MAYBE_U16, false)?
     .visit_field::<u16>("default_u16", Self::VT_DEFAULT_U16, false)?
     .visit_field::<i32>("just_i32", Self::VT_JUST_I32, false)?
     .visit_field::<i32>("maybe_i32", Self::VT_MAYBE_I32, false)?
     .visit_field::<i32>("default_i32", Self::VT_DEFAULT_I32, false)?
     .visit_field::<u32>("just_u32", Self::VT_JUST_U32, false)?
     .visit_field::<u32>("maybe_u32", Self::VT_MAYBE_U32, false)?
     .visit_field::<u32>("default_u32", Self::VT_DEFAULT_U32, false)?
     .visit_field::<i64>("just_i64", Self::VT_JUST_I64, false)?
     .visit_field::<i64>("maybe_i64", Self::VT_MAYBE_I64, false)?
     .visit_field::<i64>("default_i64", Self::VT_DEFAULT_I64, false)?
     .visit_field::<u64>("just_u64", Self::VT_JUST_U64, false)?
     .visit_field::<u64>("maybe_u64", Self::VT_MAYBE_U64, false)?
     .visit_field::<u64>("default_u64", Self::VT_DEFAULT_U64, false)?
     .visit_field::<f32>("just_f32", Self::VT_JUST_F32, false)?
     .visit_field::<f32>("maybe_f32", Self::VT_MAYBE_F32, false)?
     .visit_field::<f32>("default_f32", Self::VT_DEFAULT_F32, false)?
     .visit_field::<f64>("just_f64", Self::VT_JUST_F64, false)?
     .visit_field::<f64>("maybe_f64", Self::VT_MAYBE_F64, false)?
     .visit_field::<f64>("default_f64", Self::VT_DEFAULT_F64, false)?
     .visit_field::<bool>("just_bool", Self::VT_JUST_BOOL, false)?
     .visit_field::<bool>("maybe_bool", Self::VT_MAYBE_BOOL, false)?
     .visit_field::<bool>("default_bool", Self::VT_DEFAULT_BOOL, false)?
     .visit_field::<OptionalByte>("just_enum", Self::VT_JUST_ENUM, false)?
     .visit_field::<OptionalByte>("maybe_enum", Self::VT_MAYBE_ENUM, false)?
     .visit_field::<OptionalByte>("default_enum", Self::VT_DEFAULT_ENUM, false)?
     .finish();
    Ok(())
  }
}
pub struct ScalarStuffArgs {
    pub just_i8: i8,
    pub maybe_i8: Option<i8>,
    pub default_i8: i8,
    pub just_u8: u8,
    pub maybe_u8: Option<u8>,
    pub default_u8: u8,
    pub just_i16: i16,
    pub maybe_i16: Option<i16>,
    pub default_i16: i16,
    pub just_u16: u16,
    pub maybe_u16: Option<u16>,
    pub default_u16: u16,
    pub just_i32: i32,
    pub maybe_i32: Option<i32>,
    pub default_i32: i32,
    pub just_u32: u32,
    pub maybe_u32: Option<u32>,
    pub default_u32: u32,
    pub just_i64: i64,
    pub maybe_i64: Option<i64>,
    pub default_i64: i64,
    pub just_u64: u64,
    pub maybe_u64: Option<u64>,
    pub default_u64: u64,
    pub just_f32: f32,
    pub maybe_f32: Option<f32>,
    pub default_f32: f32,
    pub just_f64: f64,
    pub maybe_f64: Option<f64>,
    pub default_f64: f64,
    pub just_bool: bool,
    pub maybe_bool: Option<bool>,
    pub default_bool: bool,
    pub just_enum: OptionalByte,
    pub maybe_enum: Option<OptionalByte>,
    pub default_enum: OptionalByte,
}
impl<'a> Default for ScalarStuffArgs {
  #[inline]
  fn default() -> Self {
    ScalarStuffArgs {
      just_i8: 0,
      maybe_i8: None,
      default_i8: 42,
      just_u8: 0,
      maybe_u8: None,
      default_u8: 42,
      just_i16: 0,
      maybe_i16: None,
      default_i16: 42,
      just_u16: 0,
      maybe_u16: None,
      default_u16: 42,
      just_i32: 0,
      maybe_i32: None,
      default_i32: 42,
      just_u32: 0,
      maybe_u32: None,
      default_u32: 42,
      just_i64: 0,
      maybe_i64: None,
      default_i64: 42,
      just_u64: 0,
      maybe_u64: None,
      default_u64: 42,
      just_f32: 0.0,
      maybe_f32: None,
      default_f32: 42.0,
      just_f64: 0.0,
      maybe_f64: None,
      default_f64: 42.0,
      just_bool: false,
      maybe_bool: None,
      default_bool: true,
      just_enum: OptionalByte::None,
      maybe_enum: None,
      default_enum: OptionalByte::One,
    }
  }
}

pub struct ScalarStuffBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> ScalarStuffBuilder<'a, 'b> {
  #[inline]
  pub fn add_just_i8(&mut self, just_i8: i8) {
    self.fbb_.push_slot::<i8>(ScalarStuff::VT_JUST_I8, just_i8, 0);
  }
  #[inline]
  pub fn add_maybe_i8(&mut self, maybe_i8: i8) {
    self.fbb_.push_slot_always::<i8>(ScalarStuff::VT_MAYBE_I8, maybe_i8);
  }
  #[inline]
  pub fn add_default_i8(&mut self, default_i8: i8) {
    self.fbb_.push_slot::<i8>(ScalarStuff::VT_DEFAULT_I8, default_i8, 42);
  }
  #[inline]
  pub fn add_just_u8(&mut self, just_u8: u8) {
    self.fbb_.push_slot::<u8>(ScalarStuff::VT_JUST_U8, just_u8, 0);
  }
  #[inline]
  pub fn add_maybe_u8(&mut self, maybe_u8: u8) {
    self.fbb_.push_slot_always::<u8>(ScalarStuff::VT_MAYBE_U8, maybe_u8);
  }
  #[inline]
  pub fn add_default_u8(&mut self, default_u8: u8) {
    self.fbb_.push_slot::<u8>(ScalarStuff::VT_DEFAULT_U8, default_u8, 42);
  }
  #[inline]
  pub fn add_just_i16(&mut self, just_i16: i16) {
    self.fbb_.push_slot::<i16>(ScalarStuff::VT_JUST_I16, just_i16, 0);
  }
  #[inline]
  pub fn add_maybe_i16(&mut self, maybe_i16: i16) {
    self.fbb_.push_slot_always::<i16>(ScalarStuff::VT_MAYBE_I16, maybe_i16);
  }
  #[inline]
  pub fn add_default_i16(&mut self, default_i16: i16) {
    self.fbb_.push_slot::<i16>(ScalarStuff::VT_DEFAULT_I16, default_i16, 42);
  }
  #[inline]
  pub fn add_just_u16(&mut self, just_u16: u16) {
    self.fbb_.push_slot::<u16>(ScalarStuff::VT_JUST_U16, just_u16, 0);
  }
  #[inline]
  pub fn add_maybe_u16(&mut self, maybe_u16: u16) {
    self.fbb_.push_slot_always::<u16>(ScalarStuff::VT_MAYBE_U16, maybe_u16);
  }
  #[inline]
  pub fn add_default_u16(&mut self, default_u16: u16) {
    self.fbb_.push_slot::<u16>(ScalarStuff::VT_DEFAULT_U16, default_u16, 42);
  }
  #[inline]
  pub fn add_just_i32(&mut self, just_i32: i32) {
    self.fbb_.push_slot::<i32>(ScalarStuff::VT_JUST_I32, just_i32, 0);
  }
  #[inline]
  pub fn add_maybe_i32(&mut self, maybe_i32: i32) {
    self.fbb_.push_slot_always::<i32>(ScalarStuff::VT_MAYBE_I32, maybe_i32);
  }
  #[inline]
  pub fn add_default_i32(&mut self, default_i32: i32) {
    self.fbb_.push_slot::<i32>(ScalarStuff::VT_DEFAULT_I32, default_i32, 42);
  }
  #[inline]
  pub fn add_just_u32(&mut self, just_u32: u32) {
    self.fbb_.push_slot::<u32>(ScalarStuff::VT_JUST_U32, just_u32, 0);
  }
  #[inline]
  pub fn add_maybe_u32(&mut self, maybe_u32: u32) {
    self.fbb_.push_slot_always::<u32>(ScalarStuff::VT_MAYBE_U32, maybe_u32);
  }
  #[inline]
  pub fn add_default_u32(&mut self, default_u32: u32) {
    self.fbb_.push_slot::<u32>(ScalarStuff::VT_DEFAULT_U32, default_u32, 42);
  }
  #[inline]
  pub fn add_just_i64(&mut self, just_i64: i64) {
    self.fbb_.push_slot::<i64>(ScalarStuff::VT_JUST_I64, just_i64, 0);
  }
  #[inline]
  pub fn add_maybe_i64(&mut self, maybe_i64: i64) {
    self.fbb_.push_slot_always::<i64>(ScalarStuff::VT_MAYBE_I64, maybe_i64);
  }
  #[inline]
  pub fn add_default_i64(&mut self, default_i64: i64) {
    self.fbb_.push_slot::<i64>(ScalarStuff::VT_DEFAULT_I64, default_i64, 42);
  }
  #[inline]
  pub fn add_just_u64(&mut self, just_u64: u64) {
    self.fbb_.push_slot::<u64>(ScalarStuff::VT_JUST_U64, just_u64, 0);
  }
  #[inline]
  pub fn add_maybe_u64(&mut self, maybe_u64: u64) {
    self.fbb_.push_slot_always::<u64>(ScalarStuff::VT_MAYBE_U64, maybe_u64);
  }
  #[inline]
  pub fn add_default_u64(&mut self, default_u64: u64) {
    self.fbb_.push_slot::<u64>(ScalarStuff::VT_DEFAULT_U64, default_u64, 42);
  }
  #[inline]
  pub fn add_just_f32(&mut self, just_f32: f32) {
    self.fbb_.push_slot::<f32>(ScalarStuff::VT_JUST_F32, just_f32, 0.0);
  }
  #[inline]
  pub fn add_maybe_f32(&mut self, maybe_f32: f32) {
    self.fbb_.push_slot_always::<f32>(ScalarStuff::VT_MAYBE_F32, maybe_f32);
  }
  #[inline]
  pub fn add_default_f32(&mut self, default_f32: f32) {
    self.fbb_.push_slot::<f32>(ScalarStuff::VT_DEFAULT_F32, default_f32, 42.0);
  }
  #[inline]
  pub fn add_just_f64(&mut self, just_f64: f64) {
    self.fbb_.push_slot::<f64>(ScalarStuff::VT_JUST_F64, just_f64, 0.0);
  }
  #[inline]
  pub fn add_maybe_f64(&mut self, maybe_f64: f64) {
    self.fbb_.push_slot_always::<f64>(ScalarStuff::VT_MAYBE_F64, maybe_f64);
  }
  #[inline]
  pub fn add_default_f64(&mut self, default_f64: f64) {
    self.fbb_.push_slot::<f64>(ScalarStuff::VT_DEFAULT_F64, default_f64, 42.0);
  }
  #[inline]
  pub fn add_just_bool(&mut self, just_bool: bool) {
    self.fbb_.push_slot::<bool>(ScalarStuff::VT_JUST_BOOL, just_bool, false);
  }
  #[inline]
  pub fn add_maybe_bool(&mut self, maybe_bool: bool) {
    self.fbb_.push_slot_always::<bool>(ScalarStuff::VT_MAYBE_BOOL, maybe_bool);
  }
  #[inline]
  pub fn add_default_bool(&mut self, default_bool: bool) {
    self.fbb_.push_slot::<bool>(ScalarStuff::VT_DEFAULT_BOOL, default_bool, true);
  }
  #[inline]
  pub fn add_just_enum(&mut self, just_enum: OptionalByte) {
    self.fbb_.push_slot::<OptionalByte>(ScalarStuff::VT_JUST_ENUM, just_enum, OptionalByte::None);
  }
  #[inline]
  pub fn add_maybe_enum(&mut self, maybe_enum: OptionalByte) {
    self.fbb_.push_slot_always::<OptionalByte>(ScalarStuff::VT_MAYBE_ENUM, maybe_enum);
  }
  #[inline]
  pub fn add_default_enum(&mut self, default_enum: OptionalByte) {
    self.fbb_.push_slot::<OptionalByte>(ScalarStuff::VT_DEFAULT_ENUM, default_enum, OptionalByte::One);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> ScalarStuffBuilder<'a, 'b> {
    let start = _fbb.start_table();
    ScalarStuffBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<ScalarStuff<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for ScalarStuff<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("ScalarStuff");
      ds.field("just_i8", &self.just_i8());
      ds.field("maybe_i8", &self.maybe_i8());
      ds.field("default_i8", &self.default_i8());
      ds.field("just_u8", &self.just_u8());
      ds.field("maybe_u8", &self.maybe_u8());
      ds.field("default_u8", &self.default_u8());
      ds.field("just_i16", &self.just_i16());
      ds.field("maybe_i16", &self.maybe_i16());
      ds.field("default_i16", &self.default_i16());
      ds.field("just_u16", &self.just_u16());
      ds.field("maybe_u16", &self.maybe_u16());
      ds.field("default_u16", &self.default_u16());
      ds.field("just_i32", &self.just_i32());
      ds.field("maybe_i32", &self.maybe_i32());
      ds.field("default_i32", &self.default_i32());
      ds.field("just_u32", &self.just_u32());
      ds.field("maybe_u32", &self.maybe_u32());
      ds.field("default_u32", &self.default_u32());
      ds.field("just_i64", &self.just_i64());
      ds.field("maybe_i64", &self.maybe_i64());
      ds.field("default_i64", &self.default_i64());
      ds.field("just_u64", &self.just_u64());
      ds.field("maybe_u64", &self.maybe_u64());
      ds.field("default_u64", &self.default_u64());
      ds.field("just_f32", &self.just_f32());
      ds.field("maybe_f32", &self.maybe_f32());
      ds.field("default_f32", &self.default_f32());
      ds.field("just_f64", &self.just_f64());
      ds.field("maybe_f64", &self.maybe_f64());
      ds.field("default_f64", &self.default_f64());
      ds.field("just_bool", &self.just_bool());
      ds.field("maybe_bool", &self.maybe_bool());
      ds.field("default_bool", &self.default_bool());
      ds.field("just_enum", &self.just_enum());
      ds.field("maybe_enum", &self.maybe_enum());
      ds.field("default_enum", &self.default_enum());
      ds.finish()
  }
}
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct ScalarStuffT {
  pub just_i8: i8,
  pub maybe_i8: Option<i8>,
  pub default_i8: i8,
  pub just_u8: u8,
  pub maybe_u8: Option<u8>,
  pub default_u8: u8,
  pub just_i16: i16,
  pub maybe_i16: Option<i16>,
  pub default_i16: i16,
  pub just_u16: u16,
  pub maybe_u16: Option<u16>,
  pub default_u16: u16,
  pub just_i32: i32,
  pub maybe_i32: Option<i32>,
  pub default_i32: i32,
  pub just_u32: u32,
  pub maybe_u32: Option<u32>,
  pub default_u32: u32,
  pub just_i64: i64,
  pub maybe_i64: Option<i64>,
  pub default_i64: i64,
  pub just_u64: u64,
  pub maybe_u64: Option<u64>,
  pub default_u64: u64,
  pub just_f32: f32,
  pub maybe_f32: Option<f32>,
  pub default_f32: f32,
  pub just_f64: f64,
  pub maybe_f64: Option<f64>,
  pub default_f64: f64,
  pub just_bool: bool,
  pub maybe_bool: Option<bool>,
  pub default_bool: bool,
  pub just_enum: OptionalByte,
  pub maybe_enum: Option<OptionalByte>,
  pub default_enum: OptionalByte,
}
impl Default for ScalarStuffT {
  fn default() -> Self {
    Self {
      just_i8: 0,
      maybe_i8: None,
      default_i8: 42,
      just_u8: 0,
      maybe_u8: None,
      default_u8: 42,
      just_i16: 0,
      maybe_i16: None,
      default_i16: 42,
      just_u16: 0,
      maybe_u16: None,
      default_u16: 42,
      just_i32: 0,
      maybe_i32: None,
      default_i32: 42,
      just_u32: 0,
      maybe_u32: None,
      default_u32: 42,
      just_i64: 0,
      maybe_i64: None,
      default_i64: 42,
      just_u64: 0,
      maybe_u64: None,
      default_u64: 42,
      just_f32: 0.0,
      maybe_f32: None,
      default_f32: 42.0,
      just_f64: 0.0,
      maybe_f64: None,
      default_f64: 42.0,
      just_bool: false,
      maybe_bool: None,
      default_bool: true,
      just_enum: OptionalByte::None,
      maybe_enum: None,
      default_enum: OptionalByte::One,
    }
  }
}
impl ScalarStuffT {
  pub fn pack<'b>(
    &self,
    _fbb: &mut flatbuffers::FlatBufferBuilder<'b>
  ) -> flatbuffers::WIPOffset<ScalarStuff<'b>> {
    let just_i8 = self.just_i8;
    let maybe_i8 = self.maybe_i8;
    let default_i8 = self.default_i8;
    let just_u8 = self.just_u8;
    let maybe_u8 = self.maybe_u8;
    let default_u8 = self.default_u8;
    let just_i16 = self.just_i16;
    let maybe_i16 = self.maybe_i16;
    let default_i16 = self.default_i16;
    let just_u16 = self.just_u16;
    let maybe_u16 = self.maybe_u16;
    let default_u16 = self.default_u16;
    let just_i32 = self.just_i32;
    let maybe_i32 = self.maybe_i32;
    let default_i32 = self.default_i32;
    let just_u32 = self.just_u32;
    let maybe_u32 = self.maybe_u32;
    let default_u32 = self.default_u32;
    let just_i64 = self.just_i64;
    let maybe_i64 = self.maybe_i64;
    let default_i64 = self.default_i64;
    let just_u64 = self.just_u64;
    let maybe_u64 = self.maybe_u64;
    let default_u64 = self.default_u64;
    let just_f32 = self.just_f32;
    let maybe_f32 = self.maybe_f32;
    let default_f32 = self.default_f32;
    let just_f64 = self.just_f64;
    let maybe_f64 = self.maybe_f64;
    let default_f64 = self.default_f64;
    let just_bool = self.just_bool;
    let maybe_bool = self.maybe_bool;
    let default_bool = self.default_bool;
    let just_enum = self.just_enum;
    let maybe_enum = self.maybe_enum;
    let default_enum = self.default_enum;
    ScalarStuff::create(_fbb, &ScalarStuffArgs{
      just_i8,
      maybe_i8,
      default_i8,
      just_u8,
      maybe_u8,
      default_u8,
      just_i16,
      maybe_i16,
      default_i16,
      just_u16,
      maybe_u16,
      default_u16,
      just_i32,
      maybe_i32,
      default_i32,
      just_u32,
      maybe_u32,
      default_u32,
      just_i64,
      maybe_i64,
      default_i64,
      just_u64,
      maybe_u64,
      default_u64,
      just_f32,
      maybe_f32,
      default_f32,
      just_f64,
      maybe_f64,
      default_f64,
      just_bool,
      maybe_bool,
      default_bool,
      just_enum,
      maybe_enum,
      default_enum,
    })
  }
}
#[inline]
#[deprecated(since="2.0.0", note="Deprecated in favor of `root_as...` methods.")]
pub fn get_root_as_scalar_stuff<'a>(buf: &'a [u8]) -> ScalarStuff<'a> {
  unsafe { flatbuffers::root_unchecked::<ScalarStuff<'a>>(buf) }
}

#[inline]
#[deprecated(since="2.0.0", note="Deprecated in favor of `root_as...` methods.")]
pub fn get_size_prefixed_root_as_scalar_stuff<'a>(buf: &'a [u8]) -> ScalarStuff<'a> {
  unsafe { flatbuffers::size_prefixed_root_unchecked::<ScalarStuff<'a>>(buf) }
}

#[inline]
/// Verifies that a buffer of bytes contains a `ScalarStuff`
/// and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_scalar_stuff_unchecked`.
pub fn root_as_scalar_stuff(buf: &[u8]) -> Result<ScalarStuff, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root::<ScalarStuff>(buf)
}
#[inline]
/// Verifies that a buffer of bytes contains a size prefixed
/// `ScalarStuff` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `size_prefixed_root_as_scalar_stuff_unchecked`.
pub fn size_prefixed_root_as_scalar_stuff(buf: &[u8]) -> Result<ScalarStuff, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root::<ScalarStuff>(buf)
}
#[inline]
/// Verifies, with the given options, that a buffer of bytes
/// contains a `ScalarStuff` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_scalar_stuff_unchecked`.
pub fn root_as_scalar_stuff_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<ScalarStuff<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root_with_opts::<ScalarStuff<'b>>(opts, buf)
}
#[inline]
/// Verifies, with the given verifier options, that a buffer of
/// bytes contains a size prefixed `ScalarStuff` and returns
/// it. Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_scalar_stuff_unchecked`.
pub fn size_prefixed_root_as_scalar_stuff_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<ScalarStuff<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root_with_opts::<ScalarStuff<'b>>(opts, buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a ScalarStuff and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid `ScalarStuff`.
pub unsafe fn root_as_scalar_stuff_unchecked(buf: &[u8]) -> ScalarStuff {
  flatbuffers::root_unchecked::<ScalarStuff>(buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a size prefixed ScalarStuff and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid size prefixed `ScalarStuff`.
pub unsafe fn size_prefixed_root_as_scalar_stuff_unchecked(buf: &[u8]) -> ScalarStuff {
  flatbuffers::size_prefixed_root_unchecked::<ScalarStuff>(buf)
}
pub const SCALAR_STUFF_IDENTIFIER: &str = "NULL";

#[inline]
pub fn scalar_stuff_buffer_has_identifier(buf: &[u8]) -> bool {
  flatbuffers::buffer_has_identifier(buf, SCALAR_STUFF_IDENTIFIER, false)
}

#[inline]
pub fn scalar_stuff_size_prefixed_buffer_has_identifier(buf: &[u8]) -> bool {
  flatbuffers::buffer_has_identifier(buf, SCALAR_STUFF_IDENTIFIER, true)
}

pub const SCALAR_STUFF_EXTENSION: &str = "mon";

#[inline]
pub fn finish_scalar_stuff_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<ScalarStuff<'a>>) {
  fbb.finish(root, Some(SCALAR_STUFF_IDENTIFIER));
}

#[inline]
pub fn finish_size_prefixed_scalar_stuff_buffer<'a, 'b>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, root: flatbuffers::WIPOffset<ScalarStuff<'a>>) {
  fbb.finish_size_prefixed(root, Some(SCALAR_STUFF_IDENTIFIER));
}
