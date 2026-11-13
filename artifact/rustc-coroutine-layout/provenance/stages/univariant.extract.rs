//@edition: 2024
#![feature(new_range_api)]



use std::fmt::Debug;
use std::hash::Hash;
use std::slice::SliceIndex;

// //== ./../rustc_hashes/src/lib.rs

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Hash64 {
    inner: u64,
}

impl Hash64 {
    pub const ZERO: Hash64 = Hash64 { inner: 0 };

    #[inline]
    pub fn new(n: u64) -> Self {
        Self { inner: n }
    }

    #[inline]
    pub fn as_u64(self) -> u64 {
        self.inner
    }

    #[inline]
    #[thrust::trusted]
    #[thrust::callable]
    pub fn wrapping_add(self, other: Self) -> Self {
        Self {
            inner: self.inner.wrapping_add(other.inner),
        }
    }
}

// //== ./../rustc_index/src/idx.rs (from bitset.rs, stage 2/4)

pub trait Idx: Copy + 'static + Eq + PartialEq + Debug + Hash {
    fn new(idx: usize) -> Self;

    fn index(self) -> usize;

    #[inline]
    fn increment_by(&mut self, amount: usize) {
        *self = self.plus(amount);
    }

    #[inline]
    #[must_use = "Use `increment_by` if you wanted to update the index in-place"]
    fn plus(self, amount: usize) -> Self {
        Self::new(self.index() + amount)
    }
}

impl Idx for usize {
    #[inline]
    fn new(idx: usize) -> Self {
        idx
    }
    #[inline]
    fn index(self) -> usize {
        self
    }
}

impl Idx for u32 {
    #[inline]
    fn new(idx: usize) -> Self {
        assert!(idx <= u32::MAX as usize);
        idx as u32
    }
    #[inline]
    fn index(self) -> usize {
        self as usize
    }
}

/// Own iterator standing in for `(start..end).map(I::new)`: yields
pub struct IdxRange<I: Idx> {
    start: usize,
    end: usize,
    marker: PhantomData<I>,
}

impl<I: Idx> IdxRange<I> {
    fn new(start: usize, end: usize) -> IdxRange<I> {
        IdxRange {
            start,
            end,
            marker: PhantomData,
        }
    }
}

impl<I: Idx> Iterator for IdxRange<I> {
    type Item = I;

    fn next(&mut self) -> Option<I> {
        if self.start < self.end {
            let n = self.start;
            self.start += 1;
            Some(I::new(n))
        } else {
            None
        }
    }
}

    type Output: SliceIndex<T>;
    fn into_slice_idx(self) -> Self::Output;
}

impl<I: Idx, T> IntoSliceIdx<I, [T]> for I {
    type Output = usize;
    #[inline]
    fn into_slice_idx(self) -> Self::Output {
        self.index()
    }
}

// //== ./../rustc_index/src/slice.rs (from eligibility.rs, stage 2)

#[derive(PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct IndexSlice<I: Idx, T> {
    _marker: PhantomData<fn(&I)>,
    pub raw: [T],
}

pub struct SliceIter<'a, T> {
    raw: &'a [T],
    pos: usize,
}

impl<'a, T> Iterator for SliceIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if self.pos < self.raw.len() {
            let item = &self.raw[self.pos];
            self.pos += 1;
            Some(item)
        } else {
            None
        }
    }
}

pub struct IterEnumerated<'a, I: Idx, T> {
    raw: &'a [T],
    pos: usize,
    marker: PhantomData<I>,
}

impl<'a, I: Idx, T> Iterator for IterEnumerated<'a, I, T> {
    type Item = (I, &'a T);

    fn next(&mut self) -> Option<(I, &'a T)> {
        if self.pos < self.raw.len() {
            let n = self.pos;
            self.pos += 1;
            Some((I::new(n), &self.raw[n]))
        } else {
            None
        }
    }
}

impl<I: Idx, T> IndexSlice<I, T> {
    #[inline]
    pub const fn from_raw(raw: &[T]) -> &Self {
        let ptr: *const [T] = raw;

        unsafe { &*(ptr as *const Self) }
    }

    #[inline]
    pub fn from_raw_mut(raw: &mut [T]) -> &mut Self {
        let ptr: *mut [T] = raw;

        unsafe { &mut *(ptr as *mut Self) }
    }

    #[inline]
    pub const fn len(&self) -> usize {
        self.raw.len()
    }

    #[inline]
    pub fn next_index(&self) -> I {
        I::new(self.len())
    }

    #[inline]
    pub fn iter(&self) -> SliceIter<'_, T> {
        SliceIter {
            raw: &self.raw,
            pos: 0,
        }
    }

    #[inline]
    pub fn iter_enumerated(&self) -> IterEnumerated<'_, I, T> {
        let _ = I::new(self.len());
        IterEnumerated {
            raw: &self.raw,
            pos: 0,
            marker: PhantomData,
        }
    }

    #[inline]
    pub fn indices(&self) -> IdxRange<I> {
        let _ = I::new(self.len());
        IdxRange::new(0, self.len())
    }
}

impl<I: Idx, T, R: IntoSliceIdx<I, [T]>> Index<R> for IndexSlice<I, T> {
    type Output = <R::Output as SliceIndex<[T]>>::Output;

    #[inline]
    fn index(&self, index: R) -> &Self::Output {
        &self.raw[index.into_slice_idx()]
    }
}

impl<I: Idx, T, R: IntoSliceIdx<I, [T]>> IndexMut<R> for IndexSlice<I, T> {
    #[inline]
    fn index_mut(&mut self, index: R) -> &mut Self::Output {
        &mut self.raw[index.into_slice_idx()]
    }
}

// //== ./../rustc_index/src/vec.rs (from eligibility.rs, stage 2)

use std::borrow::{Borrow, BorrowMut};
use std::ops::{Deref, DerefMut};

#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct IndexVec<I: Idx, T> {
    pub raw: Vec<T>,
    _marker: PhantomData<fn(&I)>,
}

impl<I: Idx, T> IndexVec<I, T> {
    #[inline]
    pub const fn new() -> Self {
        IndexVec::from_raw(Vec::new())
    }

    #[inline]
    pub const fn from_raw(raw: Vec<T>) -> Self {
        IndexVec {
            raw,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn from_elem<S>(elem: T, universe: &IndexSlice<I, S>) -> Self
    where
        T: Clone,
    {
        IndexVec::from_raw(vec![elem; universe.len()])
    }

    #[inline]
    pub fn push(&mut self, d: T) -> I {
        let idx = self.next_index();
        self.raw.push(d);
        idx
    }
}

impl<I: Idx, T> Deref for IndexVec<I, T> {
    type Target = IndexSlice<I, T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<I: Idx, T> DerefMut for IndexVec<I, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}

impl<I: Idx, T> Borrow<IndexSlice<I, T>> for IndexVec<I, T> {
    fn borrow(&self) -> &IndexSlice<I, T> {
        self
    }
}

impl<I: Idx, T> BorrowMut<IndexSlice<I, T>> for IndexVec<I, T> {
    fn borrow_mut(&mut self) -> &mut IndexSlice<I, T> {
        self
    }
}

impl<I: Idx, T> FromIterator<T> for IndexVec<I, T> {
    #[inline]
    fn from_iter<J>(iter: J) -> Self
    where
        J: IntoIterator<Item = T>,
    {
        IndexVec::from_raw(Vec::from_iter(iter))
    }
}

impl<'a, I: Idx, T> IntoIterator for &'a IndexVec<I, T> {
    type Item = &'a T;
    type IntoIter = SliceIter<'a, T>;

    #[inline]
    fn into_iter(self) -> SliceIter<'a, T> {
        self.iter()
    }
}

// //== ./src/layout.rs / ./src/layout/simple.rs value types (verbatim from


}

#[derive(Copy, Clone, /*Debug,*/ PartialEq, Eq)]





}

    }

    #[thrust::trusted]


        }

            } else {
        }

        } else {
        })
    }
}

// //== ./src/lib.rs (verbatim from the target)

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct ReprFlags(u8);

impl ReprFlags {
    pub const IS_C: ReprFlags = ReprFlags(1 << 0);
    pub const IS_SIMD: ReprFlags = ReprFlags(1 << 1);
    pub const IS_TRANSPARENT: ReprFlags = ReprFlags(1 << 2);

    pub const IS_LINEAR: ReprFlags = ReprFlags(1 << 3);

    pub const RANDOMIZE_LAYOUT: ReprFlags = ReprFlags(1 << 4);

    pub const PASS_INDIRECTLY_IN_NON_RUSTIC_ABIS: ReprFlags = ReprFlags(1 << 5);
    pub const IS_SCALABLE: ReprFlags = ReprFlags(1 << 6);

    pub const FIELD_ORDER_UNOPTIMIZABLE: ReprFlags = ReprFlags(
        ReprFlags::IS_C.bits()
            | ReprFlags::IS_SIMD.bits()
            | ReprFlags::IS_SCALABLE.bits()
            | ReprFlags::IS_LINEAR.bits(),
    );
    pub const ABI_UNOPTIMIZABLE: ReprFlags =
        ReprFlags(ReprFlags::IS_C.bits() | ReprFlags::IS_SIMD.bits());

    pub const fn bits(&self) -> u8 {
        self.0
    }

    #[thrust::trusted]
    #[thrust::callable]
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    #[thrust::trusted]
    #[thrust::callable]
    pub const fn intersects(&self, other: Self) -> bool {
        self.0 & other.0 != 0
    }
}

#[derive(Copy, Clone, /*Debug,*/ Eq, PartialEq)]
pub enum IntegerType {
    Pointer(bool),

    Fixed(Integer, bool),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ScalableElt {
    ElementCount(u16),

    Container,
}

#[derive(Copy, Clone, /*Debug,*/ Eq, PartialEq, Default)]
pub struct ReprOptions {
    pub int: Option<IntegerType>,
    pub align: Option<Align>,
    pub pack: Option<Align>,
    pub flags: ReprFlags,

    pub scalable: Option<ScalableElt>,

    pub field_shuffle_seed: Hash64,
}

impl ReprOptions {
    #[inline]
    #[thrust::trusted]
    #[thrust::callable]
    pub fn transparent(&self) -> bool {
        self.flags.contains(ReprFlags::IS_TRANSPARENT)
    }

    #[thrust::trusted]
    #[thrust::callable]
    pub fn inhibit_newtype_abi_optimization(&self) -> bool {
        self.flags.intersects(ReprFlags::ABI_UNOPTIMIZABLE)
    }

    #[thrust::trusted]
    #[thrust::callable]
    pub fn inhibit_struct_field_reordering(&self) -> bool {
        self.flags.intersects(ReprFlags::FIELD_ORDER_UNOPTIMIZABLE) || self.int.is_some()
    }

    #[thrust::trusted]
    #[thrust::callable]
    pub fn can_randomize_type_layout(&self) -> bool {
        !self.inhibit_struct_field_reordering() && self.flags.contains(ReprFlags::RANDOMIZE_LAYOUT)
    }
}

// //== ./src/layout.rs / ./src/layout/coroutine.rs value types (verbatim)


}

    }
}

// //== ./src/layout/simple.rs (verbatim, stays trusted per the stage plan)

impl<FieldIdx: Idx, VariantIdx: Idx> LayoutData<FieldIdx, VariantIdx> {
    #[thrust::trusted]
    #[thrust::callable]
    pub fn scalar_pair<C: HasDataLayout>(cx: &C, a: Scalar, b: Scalar) -> Self {
        let dl = cx.data_layout();
        let b_align = b.align(dl).abi;
        let align = a.align(dl).abi.max(b_align).max(dl.aggregate_align);
        let b_offset = a.size(dl).align_to(b_align);
        let size = (b_offset + b.size(dl)).align_to(align);

        let largest_niche = Niche::from_scalar(dl, b_offset, b)
            .into_iter()
            .chain(Niche::from_scalar(dl, Size::ZERO, a))
            .max_by_key(|niche| niche.available(dl));

        let combined_seed = a.size(dl).bytes().wrapping_add(b.size(dl).bytes());

        LayoutData {
            variants: Variants::Single {
                index: VariantIdx::new(0),
            },
            fields: FieldsShape::Arbitrary {
                offsets: [Size::ZERO, b_offset].into(),
                in_memory_order: [FieldIdx::new(0), FieldIdx::new(1)].into(),
            },
            backend_repr: BackendRepr::ScalarPair(a, b),
            largest_niche,
            uninhabited: false,
            align: AbiAlign::new(align),
            size,
            max_repr_align: None,
            unadjusted_abi_align: align,
            randomization_seed: Hash64::new(combined_seed),
        }
    }
}

// //== ./src/layout.rs: LayoutCalculator (verbatim; `univariant`/

enum NicheBias {
    Start,
    End,
}

#[derive(Copy, Clone, /*Debug,*/ PartialEq, Eq)]
pub enum LayoutCalculatorError<F> {
    UnexpectedUnsized(F),

    SizeOverflow,

    EmptyUnion,

    ReprConflict,

    ZeroLengthSimdType,

    OversizedSimdType { max_lanes: u64 },

    NonPrimitiveSimdType(F),
}

type LayoutCalculatorResult<FieldIdx, VariantIdx, F> =
    Result<LayoutData<FieldIdx, VariantIdx>, LayoutCalculatorError<F>>;

#[derive(Clone, Copy /*Debug*/)]
pub struct LayoutCalculator<Cx> {
    pub cx: Cx,
}

impl<Cx: HasDataLayout> LayoutCalculator<Cx> {
    #[thrust::trusted]
    pub fn univariant<
        'a,
        FieldIdx: Idx,
        VariantIdx: Idx,
    >(
        &self,
        fields: &IndexSlice<FieldIdx, F>,
        repr: &ReprOptions,
        kind: StructKind,
    ) -> LayoutCalculatorResult<FieldIdx, VariantIdx, F> {
        let dl = self.cx.data_layout();
        let layout = self.univariant_biased(fields, repr, kind, NicheBias::Start);

        if let Ok(layout) = &layout {
            if !matches!(kind, StructKind::MaybeUnsized) {
                if let Some(niche) = layout.largest_niche {
                    let head_space = niche.offset.bytes();
                    let niche_len = niche.value.size(dl).bytes();
                    let tail_space = layout.size.bytes() - head_space - niche_len;

                    if fields.len() > 1 && head_space != 0 && tail_space > 0 {
                        let alt_layout = self
                            .univariant_biased(fields, repr, kind, NicheBias::End)
                            .unwrap_without_debug();
                        let alt_head_space = alt_niche.offset.bytes();
                        let alt_niche_len = alt_niche.value.size(dl).bytes();
                        let alt_tail_space =
                            alt_layout.size.bytes() - alt_head_space - alt_niche_len;

                        debug_assert_eq!(layout.size.bytes(), alt_layout.size.bytes());

                        let prefer_alt_layout =
                            alt_head_space > head_space && alt_head_space > tail_space;

                        if prefer_alt_layout {
                            return Ok(alt_layout);
                        }
                    }
                }
            }
        }
        layout
    }

    #[thrust::trusted]
    #[thrust::callable]
    fn univariant_biased<
        'a,
        FieldIdx: Idx,
        VariantIdx: Idx,
    >(
        &self,
        fields: &IndexSlice<FieldIdx, F>,
        repr: &ReprOptions,
        kind: StructKind,
        niche_bias: NicheBias,
    ) -> LayoutCalculatorResult<FieldIdx, VariantIdx, F> {
        let dl = self.cx.data_layout();
        let pack = repr.pack;
        let mut align = if pack.is_some() {
            dl.i8_align
        } else {
            dl.aggregate_align
        };
        let mut max_repr_align = repr.align;
        let mut in_memory_order: IndexVec<u32, FieldIdx> = fields.indices().collect();
        let optimize_field_order = !repr.inhibit_struct_field_reordering();
        let end = if let StructKind::MaybeUnsized = kind {
            fields.len() - 1
        } else {
            fields.len()
        };
        let optimizing = &mut in_memory_order.raw[..end];
        let fields_excluding_tail = &fields.raw[..end];

        let field_seed = fields_excluding_tail.iter().fold(Hash64::ZERO, |acc, f| {
            acc.wrapping_add(f.randomization_seed)
        });

        if optimize_field_order && fields.len() > 1 {
            if repr.can_randomize_type_layout() && cfg!(feature = "randomize") {
                #[cfg(feature = "randomize")]
                {
                    use rand::SeedableRng;
                    use rand::seq::SliceRandom;

                    let mut rng = rand_xoshiro::Xoshiro128StarStar::seed_from_u64(
                        field_seed.wrapping_add(repr.field_shuffle_seed).as_u64(),
                    );

                    optimizing.shuffle(&mut rng);
                }
            } else {
                let max_field_align = fields_excluding_tail
                    .iter()
                    .map(|f| f.align.bytes())
                    .max()
                    .unwrap_or(1);
                let largest_niche_size = fields_excluding_tail
                    .iter()
                    .filter_map(|f| f.largest_niche)
                    .map(|n| n.available(dl))
                    .max()
                    .unwrap_or(0);

                let alignment_group_key = |layout: &F| {
                    if let Some(pack) = pack {
                        layout.align.abi.min(pack).bytes()
                    } else {
                        let align = layout.align.bytes();
                        let size = layout.size.bytes();
                        let niche_size = layout.largest_niche.map(|n| n.available(dl)).unwrap_or(0);

                        let size_as_align = align.max(size).trailing_zeros();
                        let size_as_align = if largest_niche_size > 0 {
                            match niche_bias {
                                NicheBias::Start => {
                                    max_field_align.trailing_zeros().min(size_as_align)
                                }

                                NicheBias::End if niche_size == largest_niche_size => {
                                    align.trailing_zeros()
                                }
                                NicheBias::End => size_as_align,
                            }
                        } else {
                            size_as_align
                        };
                        size_as_align as u64
                    }
                };

                match kind {
                    StructKind::AlwaysSized | StructKind::MaybeUnsized => {
                        optimizing.sort_by_key(|&x| {
                            let f = &fields[x];
                            let field_size = f.size.bytes();
                            let niche_size = f.largest_niche.map_or(0, |n| n.available(dl));
                            let niche_size_key = match niche_bias {
                                NicheBias::Start => !niche_size,

                                NicheBias::End => niche_size,
                            };
                            let inner_niche_offset_key = match niche_bias {
                                NicheBias::Start => f.largest_niche.map_or(0, |n| n.offset.bytes()),
                                NicheBias::End => f.largest_niche.map_or(0, |n| {
                                    !(field_size - n.value.size(dl).bytes() - n.offset.bytes())
                                }),
                            };

                            (
                                cmp::Reverse(alignment_group_key(f)),
                                niche_size_key,
                                inner_niche_offset_key,
                            )
                        });
                    }

                    StructKind::Prefixed(..) => {
                        optimizing.sort_by_key(|&x| {
                            let f = &fields[x];
                            let niche_size = f.largest_niche.map_or(0, |n| n.available(dl));
                            (alignment_group_key(f), niche_size)
                        });
                    }
                }
            }
        }

        let mut unsized_field = None::<&F>;
        let mut offsets = IndexVec::from_elem(Size::ZERO, fields);
        let mut offset = Size::ZERO;
        let mut largest_niche = None;
        let mut largest_niche_available = 0;
        if let StructKind::Prefixed(prefix_size, prefix_align) = kind {
            let prefix_align = if let Some(pack) = pack {
                prefix_align.min(pack)
            } else {
                prefix_align
            };
            align = align.max(prefix_align);
            offset = prefix_size.align_to(prefix_align);
        }
        for &i in &in_memory_order {
            let field = &fields[i];
            if let Some(unsized_field) = unsized_field {
                return Err(LayoutCalculatorError::UnexpectedUnsized(*unsized_field));
            }

            if field.is_unsized() {
                if let StructKind::MaybeUnsized = kind {
                    unsized_field = Some(field);
                } else {
                    return Err(LayoutCalculatorError::UnexpectedUnsized(*field));
                }
            }

            let field_align = if let Some(pack) = pack {
                field.align.min(AbiAlign::new(pack))
            } else {
                field.align
            };
            offset = offset.align_to(field_align.abi);
            align = align.max(field_align.abi);
            max_repr_align = max_repr_align.max(field.max_repr_align);

            offsets[i] = offset;

            if let Some(mut niche) = field.largest_niche {
                let available = niche.available(dl);

                let prefer_new_niche = match niche_bias {
                    NicheBias::Start => available > largest_niche_available,

                    NicheBias::End => available >= largest_niche_available,
                };
                if prefer_new_niche {
                    largest_niche_available = available;
                    niche.offset += offset;
                    largest_niche = Some(niche);
                }
            }

            offset = offset
                .checked_add(field.size, dl)
                .ok_or(LayoutCalculatorError::SizeOverflow)?;
        }

        let unadjusted_abi_align = align;
        if let Some(repr_align) = repr.align {
            align = align.max(repr_align);
        }

        let align = align;

        let min_size = offset;
        let size = min_size.align_to(align);

        if size.bytes() >= dl.obj_size_bound() {
            return Err(LayoutCalculatorError::SizeOverflow);
        }
        let mut layout_of_single_non_zst_field = None;
        let sized = unsized_field.is_none();
        let mut abi = BackendRepr::Memory { sized };

        let optimize_abi = !repr.inhibit_newtype_abi_optimization();

        if sized && size.bytes() > 0 {
            let mut non_zst_fields = fields.iter_enumerated().filter(|&(_, f)| !f.is_zst());

            match (
                non_zst_fields.next(),
                non_zst_fields.next(),
                non_zst_fields.next(),
            ) {
                (Some((i, field)), None, None) => {
                    layout_of_single_non_zst_field = Some(field);

                    if offsets[i].bytes() == 0 && align == field.align.abi && size == field.size {
                        match field.backend_repr {
                            BackendRepr::Scalar(_) | BackendRepr::SimdVector { .. }
                                if optimize_abi =>
                            {
                                abi = field.backend_repr;
                            }

                            BackendRepr::ScalarPair(..) => {
                                abi = field.backend_repr;
                            }
                            _ => {}
                        }
                    }
                }

                (Some((i, a)), Some((j, b)), None) => match (a.backend_repr, b.backend_repr) {
                    (BackendRepr::Scalar(a), BackendRepr::Scalar(b)) => {
                        let ((i, a), (j, b)) = if offsets[i] < offsets[j] {
                            ((i, a), (j, b))
                        } else {
                            ((j, b), (i, a))
                        };
                        let pair = LayoutData::<FieldIdx, VariantIdx>::scalar_pair(&self.cx, a, b);
                        let pair_offsets = match pair.fields {
                            FieldsShape::Arbitrary {
                                ref offsets,
                                ref in_memory_order,
                            } => {
                                assert_eq!(
                                    in_memory_order.raw,
                                    [FieldIdx::new(0), FieldIdx::new(1)]
                                );
                                offsets
                            }
                            FieldsShape::Primitive
                            | FieldsShape::Array { .. }
                            | FieldsShape::Union(..) => {
                                panic!("encountered a non-arbitrary layout during enum layout")
                            }
                        };
                        if offsets[i] == pair_offsets[FieldIdx::new(0)]
                            && offsets[j] == pair_offsets[FieldIdx::new(1)]
                            && align == pair.align.abi
                            && size == pair.size
                        {
                            abi = pair.backend_repr;
                        }
                    }
                    _ => {}
                },

                _ => {}
            }
        }
        let uninhabited = fields.iter().any(|f| f.is_uninhabited());

        let unadjusted_abi_align = if repr.transparent() {
            match layout_of_single_non_zst_field {
                Some(l) => l.unadjusted_abi_align,
                None => align,
            }
        } else {
            unadjusted_abi_align
        };

        let seed = field_seed.wrapping_add(repr.field_shuffle_seed);

        Ok(LayoutData {
            variants: Variants::Single {
                index: VariantIdx::new(0),
            },
            fields: FieldsShape::Arbitrary {
                offsets,
                in_memory_order,
            },
            backend_repr: abi,
            largest_niche,
            uninhabited,
            align: AbiAlign::new(align),
            size,
            max_repr_align,
            unadjusted_abi_align,
            randomization_seed: seed,
        })
    }
}

// //== Thrust model declarations

}

}

impl<'a, T> thrust_models::Model for SliceIter<'a, T> {
    type Ty = Self;
}
impl<'a, I: Idx, T> thrust_models::Model for IterEnumerated<'a, I, T> {
    type Ty = Self;
}
impl<I: Idx, T> thrust_models::Model for IndexVec<I, T> {
    type Ty = Self;
}
impl<Cx> thrust_models::Model for LayoutCalculator<Cx> {
    type Ty = Self;
}
impl thrust_models::Model for PointerSpec {
    type Ty = Self;
}
impl thrust_models::Model for TargetDataLayout {
    type Ty = Self;
}
impl thrust_models::Model for Endian {
    type Ty = Self;
}
impl thrust_models::Model for Size {
    type Ty = Self;
}
impl thrust_models::Model for Align {
    type Ty = Self;
}
impl thrust_models::Model for AbiAlign {
    type Ty = Self;
}
impl thrust_models::Model for Integer {
    type Ty = Self;
}
impl thrust_models::Model for Float {
    type Ty = Self;
}
impl thrust_models::Model for Primitive {
    type Ty = Self;
}
impl thrust_models::Model for WrappingRange {
    type Ty = Self;
}
impl thrust_models::Model for Scalar {
    type Ty = Self;
}
    type Ty = Self;
}
impl thrust_models::Model for AddressSpace {
    type Ty = Self;
}
impl thrust_models::Model for NumScalableVectors {
    type Ty = Self;
}
impl thrust_models::Model for BackendRepr {
    type Ty = Self;
}
impl<FieldIdx: Idx, VariantIdx: Idx> thrust_models::Model for Variants<FieldIdx, VariantIdx> {
    type Ty = Self;
}
impl<VariantIdx: Idx> thrust_models::Model for TagEncoding<VariantIdx> {
    type Ty = Self;
}
impl<FieldIdx: Idx, VariantIdx: Idx> thrust_models::Model for LayoutData<FieldIdx, VariantIdx> {
    type Ty = Self;
}
impl thrust_models::Model for StructKind {
    type Ty = Self;
}
impl<FieldIdx: Idx> thrust_models::Model for VariantLayout<FieldIdx> {
    type Ty = Self;
}
}

    }
}

    }
}
