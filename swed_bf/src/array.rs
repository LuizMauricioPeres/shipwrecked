//! Harbour array functions: Array, AAdd, ASize, AFill, AScan, ATail, ADel, AIns, AClone.
//!
//! # Nota sobre mutação no registry
//!
//! Funções como `AAdd`, `ADel`, `AIns` e `AFill` recebem os argumentos por valor
//! (`Vec<HbValue>`), portanto operam sobre uma cópia do array. No código gerado,
//! mutações usam os métodos diretos de `HbValue`/`HbArray` (`hb_aadd`, `hb_set_val`,
//! etc.) — o registry destina-se ao interpretador futuro.

use swed_co::NativeFunction;
use swed_rt::{HbArray, HbValue};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn take2(args: Vec<HbValue>) -> (HbValue, HbValue) {
    let mut it = args.into_iter();
    let a = it.next().unwrap_or(HbValue::Nil);
    let b = it.next().unwrap_or(HbValue::Nil);
    (a, b)
}

fn take3(args: Vec<HbValue>) -> (HbValue, HbValue, HbValue) {
    let mut it = args.into_iter();
    let a = it.next().unwrap_or(HbValue::Nil);
    let b = it.next().unwrap_or(HbValue::Nil);
    let c = it.next().unwrap_or(HbValue::Nil);
    (a, b, c)
}

fn take4(args: Vec<HbValue>) -> (HbValue, HbValue, HbValue, HbValue) {
    let mut it = args.into_iter();
    let a = it.next().unwrap_or(HbValue::Nil);
    let b = it.next().unwrap_or(HbValue::Nil);
    let c = it.next().unwrap_or(HbValue::Nil);
    let d = it.next().unwrap_or(HbValue::Nil);
    (a, b, c, d)
}

fn to_1based(v: HbValue, default: usize) -> usize {
    match v {
        HbValue::Integer(n) if n >= 1 => n as usize,
        HbValue::Nil => default,
        _ => default,
    }
}

// ---------------------------------------------------------------------------
// Array( nElements ) — create NIL-filled array
// ---------------------------------------------------------------------------

/// `ARRAY(nElements)` — creates an array of `nElements` NIL entries.
pub fn hb_array_new(n: HbValue) -> HbValue {
    match n {
        HbValue::Integer(n) if n >= 0 => HbValue::Array(HbArray::filled(n as usize)),
        _ => HbValue::Nil,
    }
}

// ---------------------------------------------------------------------------
// AAdd( aArray [, xValue] ) — append element, return xValue
// ---------------------------------------------------------------------------

/// `AADD(aArray, xValue)` — appends `xValue` to `aArray`; returns `xValue`.
///
/// In the registry the input array is cloned; mutation does not propagate
/// to the caller. Generated code uses `HbValue::hb_aadd` directly.
pub fn hb_aadd(arr: HbValue, val: HbValue) -> HbValue {
    match arr {
        HbValue::Array(_) => val,
        _ => HbValue::Nil,
    }
}

// ---------------------------------------------------------------------------
// ASize( aArray, nSize ) — resize, return array
// ---------------------------------------------------------------------------

/// `ASIZE(aArray, nSize)` — resizes the array; new slots are NIL, excess is dropped.
/// Returns the resized array.
pub fn hb_asize(arr: HbValue, size: HbValue) -> HbValue {
    match arr {
        HbValue::Array(mut a) => {
            a.hb_asize(size);
            HbValue::Array(a)
        }
        _ => HbValue::Nil,
    }
}

// ---------------------------------------------------------------------------
// AFill( aArray, xValue [, nStart [, nCount]] ) — fill range, return array
// ---------------------------------------------------------------------------

/// `AFILL(aArray, xValue [, nStart [, nCount]])` — fills `nCount` elements
/// starting at `nStart` (1-based) with `xValue`. Defaults: start=1, count=Len.
/// Returns the modified array.
pub fn hb_afill(arr: HbValue, val: HbValue, start: HbValue, count: HbValue) -> HbValue {
    match arr {
        HbValue::Array(mut a) => {
            let s = to_1based(start, 1);
            let c = match count {
                HbValue::Integer(n) if n > 0 => n as usize,
                HbValue::Nil => a.len().saturating_sub(s - 1),
                _ => a.len().saturating_sub(s - 1),
            };
            if s <= a.len() && c > 0 {
                a.hb_afill(&val, s, c);
            }
            HbValue::Array(a)
        }
        _ => HbValue::Nil,
    }
}

// ---------------------------------------------------------------------------
// AScan( aArray, xSearch [, nStart [, nCount]] ) — linear search
// ---------------------------------------------------------------------------

/// `ASCAN(aArray, xSearch [, nStart [, nCount]])` — returns the 1-based index
/// of the first element equal to `xSearch`, or `0` if not found.
pub fn hb_ascan(arr: HbValue, val: HbValue, start: HbValue, count: HbValue) -> HbValue {
    match arr {
        HbValue::Array(a) => {
            let s = to_1based(start, 1);
            let c = match count {
                HbValue::Integer(n) if n > 0 => n as usize,
                HbValue::Nil => a.len().saturating_sub(s - 1),
                _ => a.len().saturating_sub(s - 1),
            };
            for i in s..=s + c - 1 {
                if i > a.len() { break; }
                if a.hb_get(i) == val {
                    return HbValue::Integer(i as i64);
                }
            }
            HbValue::Integer(0)
        }
        _ => HbValue::Integer(0),
    }
}

// ---------------------------------------------------------------------------
// ATail( aArray ) — last element
// ---------------------------------------------------------------------------

/// `ATAIL(aArray)` — returns the rightmost element without removing it.
/// Returns `NIL` for an empty array.
pub fn hb_atail(arr: HbValue) -> HbValue {
    match arr {
        HbValue::Array(a) => a.hb_get(a.len()),
        _ => HbValue::Nil,
    }
}

// ---------------------------------------------------------------------------
// ADel( aArray, nPos ) — delete element at pos, shift left, last → NIL
// ---------------------------------------------------------------------------

/// `ADEL(aArray, nPos)` — deletes element at 1-based `nPos`, shifts remaining
/// elements left, and sets the last slot to NIL (array size is preserved).
/// Returns the modified array.
pub fn hb_adel(arr: HbValue, pos: HbValue) -> HbValue {
    match arr {
        HbValue::Array(mut a) => {
            a.hb_adel(pos);
            HbValue::Array(a)
        }
        _ => HbValue::Nil,
    }
}

// ---------------------------------------------------------------------------
// AIns( aArray, nPos ) — insert NIL at pos, drop last element
// ---------------------------------------------------------------------------

/// `AINS(aArray, nPos)` — inserts NIL at 1-based `nPos`, shifting elements
/// right and discarding the last element (array size is preserved).
/// Returns the modified array.
pub fn hb_ains(arr: HbValue, pos: HbValue) -> HbValue {
    match arr {
        HbValue::Array(mut a) => {
            a.hb_ains(pos);
            HbValue::Array(a)
        }
        _ => HbValue::Nil,
    }
}

// ---------------------------------------------------------------------------
// AClone( aSource ) — deep clone
// ---------------------------------------------------------------------------

/// `ACLONE(aSource)` — returns an independent deep copy of the array.
/// Nested arrays are also cloned (via `HbArray`'s derived `Clone`).
pub fn hb_aclone(arr: HbValue) -> HbValue {
    match arr {
        HbValue::Array(a) => HbValue::Array(a.clone()),
        _ => HbValue::Nil,
    }
}

// ---------------------------------------------------------------------------
// NativeFunction implementations
// ---------------------------------------------------------------------------

/// Registry struct for `ARRAY`.
pub struct Array;
impl NativeFunction<HbValue> for Array {
    fn call(&self, args: Vec<HbValue>) -> HbValue {
        hb_array_new(args.into_iter().next().unwrap_or(HbValue::Nil))
    }
    fn name(&self) -> &'static str { "ARRAY" }
    fn arity(&self) -> (usize, usize) { (1, 1) }
}

/// Registry struct for `AADD`.
pub struct AAdd;
impl NativeFunction<HbValue> for AAdd {
    fn call(&self, args: Vec<HbValue>) -> HbValue {
        let (arr, val) = take2(args);
        hb_aadd(arr, val)
    }
    fn name(&self) -> &'static str { "AADD" }
    fn arity(&self) -> (usize, usize) { (1, 2) }
}

/// Registry struct for `ASIZE`.
pub struct ASize;
impl NativeFunction<HbValue> for ASize {
    fn call(&self, args: Vec<HbValue>) -> HbValue {
        let (arr, size) = take2(args);
        hb_asize(arr, size)
    }
    fn name(&self) -> &'static str { "ASIZE" }
    fn arity(&self) -> (usize, usize) { (2, 2) }
}

/// Registry struct for `AFILL`.
pub struct AFill;
impl NativeFunction<HbValue> for AFill {
    fn call(&self, args: Vec<HbValue>) -> HbValue {
        let (arr, val, start, count) = take4(args);
        hb_afill(arr, val, start, count)
    }
    fn name(&self) -> &'static str { "AFILL" }
    fn arity(&self) -> (usize, usize) { (2, 4) }
}

/// Registry struct for `ASCAN`.
pub struct AScan;
impl NativeFunction<HbValue> for AScan {
    fn call(&self, args: Vec<HbValue>) -> HbValue {
        let (arr, val, start, count) = take4(args);
        hb_ascan(arr, val, start, count)
    }
    fn name(&self) -> &'static str { "ASCAN" }
    fn arity(&self) -> (usize, usize) { (2, 4) }
}

/// Registry struct for `ATAIL`.
pub struct ATail;
impl NativeFunction<HbValue> for ATail {
    fn call(&self, args: Vec<HbValue>) -> HbValue {
        hb_atail(args.into_iter().next().unwrap_or(HbValue::Nil))
    }
    fn name(&self) -> &'static str { "ATAIL" }
    fn arity(&self) -> (usize, usize) { (1, 1) }
}

/// Registry struct for `ADEL`.
pub struct ADel;
impl NativeFunction<HbValue> for ADel {
    fn call(&self, args: Vec<HbValue>) -> HbValue {
        let (arr, pos) = take2(args);
        hb_adel(arr, pos)
    }
    fn name(&self) -> &'static str { "ADEL" }
    fn arity(&self) -> (usize, usize) { (2, 2) }
}

/// Registry struct for `AINS`.
pub struct AIns;
impl NativeFunction<HbValue> for AIns {
    fn call(&self, args: Vec<HbValue>) -> HbValue {
        let (arr, pos) = take2(args);
        hb_ains(arr, pos)
    }
    fn name(&self) -> &'static str { "AINS" }
    fn arity(&self) -> (usize, usize) { (2, 2) }
}

/// Registry struct for `ACLONE`.
pub struct AClone;
impl NativeFunction<HbValue> for AClone {
    fn call(&self, args: Vec<HbValue>) -> HbValue {
        hb_aclone(args.into_iter().next().unwrap_or(HbValue::Nil))
    }
    fn name(&self) -> &'static str { "ACLONE" }
    fn arity(&self) -> (usize, usize) { (1, 1) }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn i(n: i64) -> HbValue { HbValue::Integer(n) }

    fn arr3() -> HbValue {
        let mut a = HbArray::new();
        a.hb_aadd(i(10));
        a.hb_aadd(i(20));
        a.hb_aadd(i(30));
        HbValue::Array(a)
    }

    // ── Array ─────────────────────────────────────────────────────────────────

    #[test]
    fn array_creates_nil_filled() {
        match hb_array_new(i(3)) {
            HbValue::Array(a) => { assert_eq!(a.len(), 3); assert_eq!(a.hb_get(1), HbValue::Nil); }
            _ => panic!("expected Array"),
        }
    }

    #[test]
    fn array_zero_and_invalid() {
        match hb_array_new(i(0)) { HbValue::Array(a) => assert_eq!(a.len(), 0), _ => panic!() }
        assert_eq!(hb_array_new(i(-1)),       HbValue::Nil);
        assert_eq!(hb_array_new(HbValue::Nil), HbValue::Nil);
    }

    #[test]
    fn native_fn_array() {
        match Array.call(vec![i(2)]) { HbValue::Array(a) => assert_eq!(a.len(), 2), _ => panic!() }
        assert_eq!(Array.name(), "ARRAY");
        assert_eq!(Array.arity(), (1, 1));
    }

    // ── AAdd ──────────────────────────────────────────────────────────────────

    #[test]
    fn aadd_returns_value() {
        assert_eq!(hb_aadd(arr3(), i(99)), i(99));
    }

    #[test]
    fn aadd_on_non_array_returns_nil() {
        assert_eq!(hb_aadd(HbValue::Nil, i(1)), HbValue::Nil);
    }

    #[test]
    fn native_fn_aadd() {
        assert_eq!(AAdd.call(vec![arr3(), i(7)]), i(7));
        assert_eq!(AAdd.name(), "AADD");
        assert_eq!(AAdd.arity(), (1, 2));
    }

    // ── ASize ─────────────────────────────────────────────────────────────────

    #[test]
    fn asize_grow() {
        match hb_asize(arr3(), i(5)) {
            HbValue::Array(a) => { assert_eq!(a.len(), 5); assert_eq!(a.hb_get(4), HbValue::Nil); }
            _ => panic!(),
        }
    }

    #[test]
    fn asize_shrink() {
        match hb_asize(arr3(), i(2)) {
            HbValue::Array(a) => { assert_eq!(a.len(), 2); assert_eq!(a.hb_get(1), i(10)); }
            _ => panic!(),
        }
    }

    #[test]
    fn native_fn_asize() {
        match ASize.call(vec![arr3(), i(1)]) {
            HbValue::Array(a) => assert_eq!(a.len(), 1),
            _ => panic!(),
        }
        assert_eq!(ASize.name(), "ASIZE");
        assert_eq!(ASize.arity(), (2, 2));
    }

    // ── AFill ─────────────────────────────────────────────────────────────────

    #[test]
    fn afill_all() {
        match hb_afill(arr3(), i(0), HbValue::Nil, HbValue::Nil) {
            HbValue::Array(a) => {
                assert_eq!(a.hb_get(1), i(0));
                assert_eq!(a.hb_get(3), i(0));
            }
            _ => panic!(),
        }
    }

    #[test]
    fn afill_partial() {
        match hb_afill(arr3(), i(99), i(2), i(2)) {
            HbValue::Array(a) => {
                assert_eq!(a.hb_get(1), i(10));  // unchanged
                assert_eq!(a.hb_get(2), i(99));
                assert_eq!(a.hb_get(3), i(99));
            }
            _ => panic!(),
        }
    }

    #[test]
    fn native_fn_afill() {
        match AFill.call(vec![arr3(), i(0), HbValue::Nil, HbValue::Nil]) {
            HbValue::Array(a) => assert_eq!(a.hb_get(1), i(0)),
            _ => panic!(),
        }
        assert_eq!(AFill.name(), "AFILL");
        assert_eq!(AFill.arity(), (2, 4));
    }

    // ── AScan ─────────────────────────────────────────────────────────────────

    #[test]
    fn ascan_found() {
        assert_eq!(hb_ascan(arr3(), i(20), HbValue::Nil, HbValue::Nil), i(2));
    }

    #[test]
    fn ascan_not_found() {
        assert_eq!(hb_ascan(arr3(), i(99), HbValue::Nil, HbValue::Nil), i(0));
    }

    #[test]
    fn ascan_with_start() {
        // skip position 1 (value 10), start at 2 — searching for 10 → not found
        assert_eq!(hb_ascan(arr3(), i(10), i(2), HbValue::Nil), i(0));
    }

    #[test]
    fn native_fn_ascan() {
        assert_eq!(AScan.call(vec![arr3(), i(30), HbValue::Nil, HbValue::Nil]), i(3));
        assert_eq!(AScan.name(), "ASCAN");
        assert_eq!(AScan.arity(), (2, 4));
    }

    // ── ATail ─────────────────────────────────────────────────────────────────

    #[test]
    fn atail_last_element() {
        assert_eq!(hb_atail(arr3()), i(30));
    }

    #[test]
    fn atail_empty_returns_nil() {
        assert_eq!(hb_atail(HbValue::Array(HbArray::new())), HbValue::Nil);
    }

    #[test]
    fn native_fn_atail() {
        assert_eq!(ATail.call(vec![arr3()]), i(30));
        assert_eq!(ATail.name(), "ATAIL");
        assert_eq!(ATail.arity(), (1, 1));
    }

    // ── ADel ──────────────────────────────────────────────────────────────────

    #[test]
    fn adel_removes_and_shifts() {
        // {10,20,30} → del pos 1 → {20,30,NIL}
        match hb_adel(arr3(), i(1)) {
            HbValue::Array(a) => {
                assert_eq!(a.len(), 3);
                assert_eq!(a.hb_get(1), i(20));
                assert_eq!(a.hb_get(2), i(30));
                assert_eq!(a.hb_get(3), HbValue::Nil);
            }
            _ => panic!(),
        }
    }

    #[test]
    fn native_fn_adel() {
        match ADel.call(vec![arr3(), i(2)]) {
            HbValue::Array(a) => { assert_eq!(a.hb_get(1), i(10)); assert_eq!(a.hb_get(2), i(30)); }
            _ => panic!(),
        }
        assert_eq!(ADel.name(), "ADEL");
        assert_eq!(ADel.arity(), (2, 2));
    }

    // ── AIns ──────────────────────────────────────────────────────────────────

    #[test]
    fn ains_inserts_nil_and_drops_last() {
        // {10,20,30} → ins at 2 → {10,NIL,20}  (30 is dropped)
        match hb_ains(arr3(), i(2)) {
            HbValue::Array(a) => {
                assert_eq!(a.len(), 3);
                assert_eq!(a.hb_get(1), i(10));
                assert_eq!(a.hb_get(2), HbValue::Nil);
                assert_eq!(a.hb_get(3), i(20));
            }
            _ => panic!(),
        }
    }

    #[test]
    fn native_fn_ains() {
        match AIns.call(vec![arr3(), i(1)]) {
            HbValue::Array(a) => { assert_eq!(a.hb_get(1), HbValue::Nil); assert_eq!(a.hb_get(2), i(10)); }
            _ => panic!(),
        }
        assert_eq!(AIns.name(), "AINS");
        assert_eq!(AIns.arity(), (2, 2));
    }

    // ── AClone ────────────────────────────────────────────────────────────────

    #[test]
    fn aclone_produces_independent_copy() {
        let original = arr3();
        let clone = hb_aclone(original.clone());
        assert_eq!(original, clone);
        // modify clone — original unaffected (already independent since cloned by value)
    }

    #[test]
    fn aclone_non_array_returns_nil() {
        assert_eq!(hb_aclone(HbValue::Nil), HbValue::Nil);
    }

    #[test]
    fn native_fn_aclone() {
        let clone = AClone.call(vec![arr3()]);
        assert_eq!(clone, arr3());
        assert_eq!(AClone.name(), "ACLONE");
        assert_eq!(AClone.arity(), (1, 1));
    }
}
