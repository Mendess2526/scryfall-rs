//! This module defines [`Param`], which represents a single search parameter
//! for a Scryfall query. For combinations of parameters, see the
//! [`Query`][crate::search::query] module.
//!
//! There are two kinds of `Param`: boolean criteria, and parameters
//! that take a value.
//!
//! Cards and printings are tagged with many different types of criteria
//! by Scryfall. Each of these represents a boolean property that the
//! card either has or does not. Searching by a criterion will only match
//! cards that have the flag. For example,
//! ['is:firstprint'][self::criteria::Criterion::IsFirstPrint] matches only
//! the first printing of a card, and
//! ['has:watermark'][self::criteria::Criterion::HasWatermark] matches printings
//! which have a watermark. For a list of all available criteria, see the
//! [`Criterion`] enum.
//!
//! The rest of the search parameters are comprised of a name and a value, such
//! as ['name:lightning'][self::value::name] or
//! ['year:1995'][self::value::year]. All available value parameters are all
//! available as helper functions defined in the [`value`] module.
use std::fmt;

use url::Url;

use self::compare::CompareOp;
use self::criteria::Criterion;
use self::value::ValueKind;
use crate::search::Search;

pub mod compare;
mod criteria;
pub(super) mod functions;
pub mod value;

/// A filter to provide to the search to reduce the cards returned.
///
/// For more information on available parameters, refer to the
/// [official docs](https://scryfall.com/docs/syntax).
///
/// TODO(msmorgan): More.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Param(ParamImpl);

impl Param {
    fn criterion(prop: Criterion) -> Self {
        Param(ParamImpl::Criterion(prop))
    }

    fn exact(value: impl Into<String>) -> Self {
        Param(ParamImpl::ExactName(value.into()))
    }

    fn value(kind: ValueKind, value: impl ToString) -> Self {
        Param(ParamImpl::Value(kind, value.to_string()))
    }

    fn comparison(kind: ValueKind, op: CompareOp, value: impl ToString) -> Self {
        Param(ParamImpl::Comparison(kind, op, value.to_string()))
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
enum ParamImpl {
    Criterion(Criterion),
    ExactName(String),
    Value(ValueKind, String),
    Comparison(ValueKind, CompareOp, String),
}

impl fmt::Display for Param {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.0 {
            ParamImpl::Criterion(prop) => write!(f, "{}", prop),
            ParamImpl::ExactName(name) => write!(f, "!\"{}\"", name),
            ParamImpl::Value(kind, value) => kind.fmt_value(value.as_str(), f),
            ParamImpl::Comparison(kind, op, value) => kind.fmt_comparison(*op, &*value, f),
        }
    }
}

impl From<Criterion> for Param {
    fn from(prop: Criterion) -> Self {
        Param(ParamImpl::Criterion(prop))
    }
}

impl Search for Param {
    fn write_query(&self, url: &mut Url) -> crate::Result<()> {
        super::write_query_string(self, url)
    }
}