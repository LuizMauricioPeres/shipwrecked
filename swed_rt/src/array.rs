// swed_rt/src/array.rs
// HbArray — 1-indexed dynamic array, mirroring Harbour's array semantics.
//
// All public methods use the `hb_` prefix so generated code reads clearly
// and never collides with Vec's own method names.

use std::fmt;

// Forward declaration — value.rs imports HbArray, array.rs imports HbValue.
// We break the cycle with a re-export in lib.rs and use the crate path here.
use crate::value::HbValue;

// ---------------------------------------------------------------------------
// HbArray
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Default)]
pub struct HbArray(pub(crate) Vec<HbValue>);

impl HbArray {
    // ── Constructors ────────────────────────────────────────────────────

    pub fn new() -> Self {
        HbArray(Vec::new())
    }

    pub fn with_capacity(n: usize) -> Self {
        HbArray(Vec::with_capacity(n))
    }

    /// `Array(n)` — creates an array of n NIL elements.
    pub fn filled(n: usize) -> Self {
        HbArray(vec![HbValue::Nil; n])
    }

    // ── Core metrics ─────────────────────────────────────────────────────

    /// `LEN(aArr)` — number of elements.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Convenience alias used in codegen (`a.hb_len()`).
    pub fn hb_len(&self) -> HbValue {
        HbValue::Integer(self.0.len() as i64)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    // ── Element access (1-indexed) ────────────────────────────────────────

    /// `aArr[n]` — returns a clone of the element (Harbour semantics: by value).
    /// Returns `HbValue::Nil` for out-of-bounds instead of panicking.
    pub fn hb_get(&self, index: usize) -> HbValue {
        if index == 0 || index > self.0.len() {
            HbValue::Nil
        } else {
            self.0[index - 1].clone()
        }
    }

    /// Used in generated code: `arr.hb_get_val(HbValue::Integer(i))`
    pub fn hb_get_val(&self, index: HbValue) -> HbValue {
        match index {
            HbValue::Integer(i) if i > 0 => self.hb_get(i as usize),
            _ => HbValue::Nil,
        }
    }

    /// `aArr[n] := val` — panics on out-of-bounds (matches Harbour runtime error).
    pub fn hb_set(&mut self, index: usize, value: HbValue) {
        assert!(
            index >= 1 && index <= self.0.len(),
            "Array index {index} out of bounds (size {})",
            self.0.len()
        );
        self.0[index - 1] = value;
    }

    pub fn hb_set_val(&mut self, index: HbValue, value: HbValue) {
        if let HbValue::Integer(i) = index {
            if i >= 1 {
                self.hb_set(i as usize, value);
            }
        }
    }

    // ── Mutations ─────────────────────────────────────────────────────────

    /// `AAdd(aArr, xVal)` — append and return `&mut Self` for chaining.
    pub fn hb_aadd(&mut self, value: HbValue) -> &mut Self {
        self.0.push(value);
        self
    }

    /// `ASize(aArr, nSize)` — resize; fills new slots with NIL.
    pub fn hb_asize(&mut self, new_size: HbValue) {
        let n = match new_size {
            HbValue::Integer(n) if n >= 0 => n as usize,
            _ => return,
        };
        if n < self.0.len() {
            self.0.truncate(n);
        } else {
            self.0.resize(n, HbValue::Nil);
        }
    }

    /// `ADel(aArr, nIndex)` — delete element at 1-based index, shift left,
    /// set last element to NIL (Harbour preserves array size).
    pub fn hb_adel(&mut self, index: HbValue) {
        let i = match index {
            HbValue::Integer(n) if n >= 1 && n as usize <= self.0.len() => n as usize,
            _ => return,
        };
        self.0.remove(i - 1);
        self.0.push(HbValue::Nil);
    }

    /// `AIns(aArr, nIndex)` — insert NIL at 1-based position, drop last element.
    pub fn hb_ains(&mut self, index: HbValue) {
        let i = match index {
            HbValue::Integer(n) if n >= 1 && n as usize <= self.0.len() => n as usize,
            _ => return,
        };
        self.0.insert(i - 1, HbValue::Nil);
        self.0.pop();
    }

    /// `ACopy(aSrc, aDst [, nStart [, nCount [, nDstStart]]])` — partial copy.
    pub fn hb_acopy(
        src: &HbArray,
        dst: &mut HbArray,
        start: usize,
        count: usize,
        dst_start: usize,
    ) {
        for i in 0..count {
            let src_idx = start + i;
            let dst_idx = dst_start + i;
            if src_idx <= src.0.len() && dst_idx >= 1 && dst_idx <= dst.0.len() {
                dst.0[dst_idx - 1] = src.0[src_idx - 1].clone();
            }
        }
    }

    /// `AFill(aArr, xVal [, nStart [, nCount]])` — fill range with value.
    pub fn hb_afill(&mut self, value: &HbValue, start: usize, count: usize) {
        let end = (start + count - 1).min(self.0.len());
        for i in (start - 1)..end {
            self.0[i] = value.clone();
        }
    }

    /// `ASort(aArr)` — in-place sort (strings and numbers; mixed → stable by type).
    pub fn hb_asort(&mut self) {
        self.0.sort_by(|a, b| {
            a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)
        });
    }

    /// `AScan(aArr, xVal)` — linear search; returns 1-based index or 0.
    pub fn hb_ascan(&self, value: &HbValue) -> HbValue {
        for (i, v) in self.0.iter().enumerate() {
            if v == value {
                return HbValue::Integer((i + 1) as i64);
            }
        }
        HbValue::Integer(0)
    }

    // ── Iteration helpers ─────────────────────────────────────────────────

    pub fn iter(&self) -> impl Iterator<Item = &HbValue> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut HbValue> {
        self.0.iter_mut()
    }
}

// ---------------------------------------------------------------------------
// Display
// ---------------------------------------------------------------------------

impl fmt::Display for HbArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        for (i, v) in self.0.iter().enumerate() {
            if i > 0 { write!(f, ", ")?; }
            write!(f, "{v}")?;
        }
        write!(f, "}}")
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aadd_and_len() {
        let mut a = HbArray::new();
        a.hb_aadd(HbValue::Integer(1));
        a.hb_aadd(HbValue::String("two".into()));
        assert_eq!(a.len(), 2);
    }

    #[test]
    fn test_1indexed_get() {
        let mut a = HbArray::new();
        a.hb_aadd(HbValue::String("first".into()));
        assert_eq!(a.hb_get(1), HbValue::String("first".into()));
        assert_eq!(a.hb_get(0), HbValue::Nil);  // invalid
        assert_eq!(a.hb_get(99), HbValue::Nil); // out of bounds
    }

    #[test]
    fn test_asize_grow() {
        let mut a = HbArray::new();
        a.hb_aadd(HbValue::Integer(1));
        a.hb_asize(HbValue::Integer(3));
        assert_eq!(a.len(), 3);
        assert_eq!(a.hb_get(2), HbValue::Nil);
    }

    #[test]
    fn test_asize_shrink() {
        let mut a = HbArray::filled(5);
        a.hb_asize(HbValue::Integer(2));
        assert_eq!(a.len(), 2);
    }

    #[test]
    fn test_adel_preserves_size() {
        let mut a = HbArray::new();
        for i in 1..=3 { a.hb_aadd(HbValue::Integer(i)); }
        a.hb_adel(HbValue::Integer(1));
        assert_eq!(a.len(), 3);
        assert_eq!(a.hb_get(1), HbValue::Integer(2));
        assert_eq!(a.hb_get(3), HbValue::Nil);
    }

    #[test]
    fn test_ascan_found() {
        let mut a = HbArray::new();
        a.hb_aadd(HbValue::String("x".into()));
        a.hb_aadd(HbValue::String("y".into()));
        assert_eq!(a.hb_ascan(&HbValue::String("y".into())), HbValue::Integer(2));
    }

    #[test]
    fn test_ascan_not_found() {
        let a = HbArray::new();
        assert_eq!(a.hb_ascan(&HbValue::Integer(1)), HbValue::Integer(0));
    }

    #[test]
    fn test_asort() {
        let mut a = HbArray::new();
        a.hb_aadd(HbValue::Integer(3));
        a.hb_aadd(HbValue::Integer(1));
        a.hb_aadd(HbValue::Integer(2));
        a.hb_asort();
        assert_eq!(a.hb_get(1), HbValue::Integer(1));
        assert_eq!(a.hb_get(3), HbValue::Integer(3));
    }

    #[test]
    fn test_display() {
        let mut a = HbArray::new();
        a.hb_aadd(HbValue::Integer(1));
        a.hb_aadd(HbValue::String("hi".into()));
        assert_eq!(format!("{a}"), "{1, hi}");
    }
}
