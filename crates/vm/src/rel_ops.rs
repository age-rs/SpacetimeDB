use crate::errors::ErrorVm;
use crate::relation::RelValue;
use spacetimedb_sats::product_value::ProductValue;
use spacetimedb_sats::relation::{FieldExpr, Header, RowCount};
use std::collections::HashMap;
use std::sync::Arc;

pub(crate) trait ResultExt<T> {
    fn unpack_fold(self) -> Result<T, ErrorVm>;
}

/// A trait for dealing with fallible iterators for the database.
pub trait RelOps<'a> {
    fn head(&self) -> &Arc<Header>;
    fn row_count(&self) -> RowCount;
    /// Advances the `iterator` and returns the next [RelValue].
    fn next(&mut self) -> Result<Option<RelValue<'a>>, ErrorVm>;

    /// Applies a function over the elements of the iterator, producing a single final value.
    ///
    /// This is used as the "base" of many methods on `FallibleIterator`.
    #[inline]
    fn try_fold<B, E, F>(&mut self, mut init: B, mut f: F) -> Result<B, E>
    where
        Self: Sized,
        E: From<ErrorVm>,
        F: FnMut(B, RelValue<'_>) -> Result<B, ErrorVm>,
    {
        while let Some(v) = self.next()? {
            init = f(init, v)?;
        }
        Ok(init)
    }

    /// Creates an `Iterator` which uses a closure to determine if a [RelValueRef] should be yielded.
    ///
    /// Given a [RelValueRef] the closure must return true or false.
    /// The returned iterator will yield only the elements for which the closure returns true.
    ///
    /// Note:
    ///
    /// It is the equivalent of a `WHERE` clause on SQL.
    #[inline]
    fn select<P>(self, predicate: P) -> Select<Self, P>
    where
        P: FnMut(&RelValue<'_>) -> Result<bool, ErrorVm>,
        Self: Sized,
    {
        let head = self.head().clone();
        Select::new(self, head, predicate)
    }

    /// Creates an `Iterator` which uses a closure that projects to a new [RelValue] extracted from the current.
    ///
    /// Given a [RelValue] the closure must return a subset of the current one.
    ///
    /// The [Header] is pre-checked that all the fields exist and return a error if any field is not found.
    ///
    /// Note:
    ///
    /// It is the equivalent of a `SELECT` clause on SQL.
    #[inline]
    fn project<P>(self, cols: Vec<FieldExpr>, extractor: P) -> Result<Project<Self, P>, ErrorVm>
    where
        P: for<'b> FnMut(&[FieldExpr], RelValue<'b>) -> Result<RelValue<'b>, ErrorVm>,
        Self: Sized,
    {
        let count = self.row_count();
        let head = self.head().project(&cols)?;
        Ok(Project::new(self, count, Arc::new(head), cols, extractor))
    }

    /// Intersection between the left and the right, both (non-sorted) `iterators`.
    ///
    /// The hash join strategy requires the right iterator can be collected to a `HashMap`.
    /// The left iterator can be arbitrarily long.
    ///
    /// It is therefore asymmetric (you can't flip the iterators to get a right_outer join).
    ///
    /// Note:
    ///
    /// It is the equivalent of a `INNER JOIN` clause on SQL.
    #[inline]
    fn join_inner<Pred, Proj, KeyLhs, KeyRhs, Rhs>(
        self,
        with: Rhs,
        head: Arc<Header>,
        key_lhs: KeyLhs,
        key_rhs: KeyRhs,
        predicate: Pred,
        project: Proj,
    ) -> Result<JoinInner<'a, Self, Rhs, KeyLhs, KeyRhs, Pred, Proj>, ErrorVm>
    where
        Self: Sized,
        Pred: FnMut(&RelValue<'a>, &RelValue<'a>) -> Result<bool, ErrorVm>,
        Proj: FnMut(RelValue<'a>, RelValue<'a>) -> RelValue<'a>,
        KeyLhs: FnMut(&RelValue<'a>) -> Result<ProductValue, ErrorVm>,
        KeyRhs: FnMut(&RelValue<'a>) -> Result<ProductValue, ErrorVm>,
        Rhs: RelOps<'a>,
    {
        Ok(JoinInner::new(head, self, with, key_lhs, key_rhs, predicate, project))
    }

    /// Collect all the rows in this relation into a `Vec<T>` given a function `RelValue<'a> -> T`.
    #[inline]
    fn collect_vec<T>(mut self, mut convert: impl FnMut(RelValue<'a>) -> T) -> Result<Vec<T>, ErrorVm>
    where
        Self: Sized,
    {
        let count = self.row_count();
        let estimate = count.max.unwrap_or(count.min);
        let mut result = Vec::with_capacity(estimate);

        while let Some(row) = self.next()? {
            result.push(convert(row));
        }

        Ok(result)
    }
}

impl<'a, I: RelOps<'a> + ?Sized> RelOps<'a> for Box<I> {
    fn head(&self) -> &Arc<Header> {
        (**self).head()
    }

    fn row_count(&self) -> RowCount {
        (**self).row_count()
    }

    fn next(&mut self) -> Result<Option<RelValue<'a>>, ErrorVm> {
        (**self).next()
    }
}

#[derive(Clone, Debug)]
pub struct Select<I, P> {
    pub(crate) head: Arc<Header>,
    pub(crate) count: RowCount,
    pub(crate) iter: I,
    pub(crate) predicate: P,
}

impl<I, P> Select<I, P> {
    pub fn new(iter: I, head: Arc<Header>, predicate: P) -> Select<I, P> {
        Select {
            iter,
            // NOTE: We could have propagated the upper bound,
            // but this would likely cause over-allocation in `Vec::with_capacity`.
            count: RowCount::unknown(),
            predicate,
            head,
        }
    }
}

impl<'a, I, P> RelOps<'a> for Select<I, P>
where
    I: RelOps<'a>,
    P: FnMut(&RelValue<'a>) -> Result<bool, ErrorVm>,
{
    fn head(&self) -> &Arc<Header> {
        &self.head
    }

    fn row_count(&self) -> RowCount {
        self.count
    }

    fn next(&mut self) -> Result<Option<RelValue<'a>>, ErrorVm> {
        let filter = &mut self.predicate;
        while let Some(v) = self.iter.next()? {
            if filter(&v)? {
                self.count.add_exact(1);
                return Ok(Some(v));
            }
        }
        Ok(None)
    }
}

#[derive(Clone, Debug)]
pub struct Project<I, P> {
    pub(crate) head: Arc<Header>,
    pub(crate) count: RowCount,
    pub(crate) cols: Vec<FieldExpr>,
    pub(crate) iter: I,
    pub(crate) extractor: P,
}

impl<I, P> Project<I, P> {
    pub fn new(iter: I, count: RowCount, head: Arc<Header>, cols: Vec<FieldExpr>, extractor: P) -> Project<I, P> {
        Project {
            iter,
            count,
            cols,
            extractor,
            head,
        }
    }
}

impl<'a, I, P> RelOps<'a> for Project<I, P>
where
    I: RelOps<'a>,
    P: FnMut(&[FieldExpr], RelValue<'a>) -> Result<RelValue<'a>, ErrorVm>,
{
    fn head(&self) -> &Arc<Header> {
        &self.head
    }

    fn row_count(&self) -> RowCount {
        self.count
    }

    fn next(&mut self) -> Result<Option<RelValue<'a>>, ErrorVm> {
        let extract = &mut self.extractor;
        if let Some(v) = self.iter.next()? {
            return Ok(Some(extract(&self.cols, v)?));
        }
        Ok(None)
    }
}

#[derive(Clone, Debug)]
pub struct JoinInner<'a, Lhs, Rhs, KeyLhs, KeyRhs, Pred, Proj> {
    pub(crate) head: Arc<Header>,
    pub(crate) count: RowCount,
    pub(crate) lhs: Lhs,
    pub(crate) rhs: Rhs,
    pub(crate) key_lhs: KeyLhs,
    pub(crate) key_rhs: KeyRhs,
    pub(crate) predicate: Pred,
    pub(crate) projection: Proj,
    map: HashMap<ProductValue, Vec<RelValue<'a>>>,
    filled_rhs: bool,
    left: Option<RelValue<'a>>,
}

impl<'a, Lhs, Rhs, KeyLhs, KeyRhs, Pred, Proj> JoinInner<'a, Lhs, Rhs, KeyLhs, KeyRhs, Pred, Proj> {
    pub fn new(
        head: Arc<Header>,
        lhs: Lhs,
        rhs: Rhs,
        key_lhs: KeyLhs,
        key_rhs: KeyRhs,
        predicate: Pred,
        projection: Proj,
    ) -> Self {
        Self {
            head,
            count: RowCount::unknown(),
            map: HashMap::new(),
            lhs,
            rhs,
            key_lhs,
            key_rhs,
            predicate,
            projection,
            filled_rhs: false,
            left: None,
        }
    }
}

impl<'a, Lhs, Rhs, KeyLhs, KeyRhs, Pred, Proj> RelOps<'a> for JoinInner<'a, Lhs, Rhs, KeyLhs, KeyRhs, Pred, Proj>
where
    Lhs: RelOps<'a>,
    Rhs: RelOps<'a>,
    // TODO(Centril): consider using keys that aren't `ProductValue`s.
    KeyLhs: FnMut(&RelValue<'a>) -> Result<ProductValue, ErrorVm>,
    KeyRhs: FnMut(&RelValue<'a>) -> Result<ProductValue, ErrorVm>,
    Pred: FnMut(&RelValue<'a>, &RelValue<'a>) -> Result<bool, ErrorVm>,
    Proj: FnMut(RelValue<'a>, RelValue<'a>) -> RelValue<'a>,
{
    fn head(&self) -> &Arc<Header> {
        &self.head
    }

    fn row_count(&self) -> RowCount {
        self.count
    }

    fn next(&mut self) -> Result<Option<RelValue<'a>>, ErrorVm> {
        // Consume `Rhs`, building a map `KeyRhs => Rhs`.
        if !self.filled_rhs {
            self.map = HashMap::with_capacity(self.rhs.row_count().min);
            while let Some(row_rhs) = self.rhs.next()? {
                let key_rhs = (self.key_rhs)(&row_rhs)?;
                self.map.entry(key_rhs).or_default().push(row_rhs);
            }
            self.filled_rhs = true;
        }

        loop {
            // Consume a row in `Lhs` and project to `KeyLhs`.
            let lhs = if let Some(left) = &self.left {
                left.clone()
            } else {
                match self.lhs.next()? {
                    None => return Ok(None),
                    Some(x) => {
                        self.left = Some(x.clone());
                        x
                    }
                }
            };
            let k = (self.key_lhs)(&lhs)?;

            // If we can relate `KeyLhs` and `KeyRhs`, we have candidate.
            // If that candidate still has rhs elements, test against the predicate and yield.
            if let Some(rvv) = self.map.get_mut(&k) {
                if let Some(rhs) = rvv.pop() {
                    if (self.predicate)(&lhs, &rhs)? {
                        self.count.add_exact(1);
                        return Ok(Some((self.projection)(lhs, rhs)));
                    }
                }
            }
            self.left = None;
            continue;
        }
    }
}
