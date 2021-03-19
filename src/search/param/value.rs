//! TODO(msmorgan): Module docs.

use std::fmt;

use crate::search::param::compare::{compare_op_str, Compare, CompareOp};
use crate::search::param::Param;

/// The type of parameter that this is. Corresponds to the name before the ':'
/// or other operator.
///
/// Refer to [the syntax documentation](https://scryfall.com/docs/syntax) for details on the
/// available parameter types.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ValueKind(pub(super) ValueKindImpl);

impl ValueKind {
    pub(super) fn fmt_value(&self, value: &str, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self, value)
    }

    pub(super) fn fmt_comparison(
        &self,
        op: CompareOp,
        value: &str,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        write!(f, "{}{}{}", self, compare_op_str(Some(op)), value)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub(super) enum ValueKindImpl {
    Color,
    ColorIdentity,
    Type,
    Oracle,
    FullOracle,
    Keyword,
    Mana,
    Devotion,
    Produces,
    Rarity,
    InRarity,
    Set,
    InSet,
    Number,
    Block,
    SetType,
    InSetType,
    Cube,
    Format,
    Banned,
    Restricted,
    Cheapest,
    Artist,
    Flavor,
    Watermark,
    BorderColor,
    Frame,
    Date,
    Game,
    InGame,
    Language,
    InLanguage,
    Name,
    NumericComparable(NumProperty),
}

/// These properties can be compared against one another.
///
/// For example `power(gt(NumericProperty::Toughness)`.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum NumProperty {
    /// The card's power. Only creature cards have this.
    Power,
    /// The card's toughness. Only creature cards have this.
    Toughness,
    /// The card's power plus its toughness. Only creatures cards have this.
    PowTou,
    /// The card's starting loyalty. Only planeswalker cards have this.
    ///
    /// The value '0' will match non-numeric loyalties such as 'X'.
    Loyalty,
    /// The card's converted mana cost. Cards without a mana cost have a
    /// converted mana cost of '0'.
    Cmc,
    /// The number of artists who contributed to this printing of the card.
    ///
    /// *Note*: This is not the same as the number of unique artists for a
    /// particular card.
    ArtistCount,
    /// This card's current nonfoil market price in US Dollars.
    Usd,
    /// This card's current foil market price in US Dollars.
    UsdFoil,
    /// This card's current market price in Euros.
    Eur,
    /// This card's current market price in MTGO Tickets.
    Tix,
    /// The number of different illustrations among prints of this card.
    IllustrationCount,
    /// The number of different prints of this card, including both paper and
    /// digital-exclusive sets.
    PrintCount,
    /// The number of different sets this card has appeared in, including both
    /// paper and digital-exclusive sets.
    SetCount,
    /// The number of different prints of this card in paper.
    PaperPrintCount,
    /// The number of different sets this card has appeared in, paper only.
    PaperSetCount,
    /// The release year of this printing.
    Year,
}

const fn numeric_property_str(prop: NumProperty) -> &'static str {
    match prop {
        NumProperty::Power => "power",
        NumProperty::Toughness => "toughness",
        NumProperty::PowTou => "powtou",
        NumProperty::Loyalty => "loyalty",
        NumProperty::Cmc => "cmc",
        NumProperty::ArtistCount => "artists",
        NumProperty::Usd => "usd",
        NumProperty::UsdFoil => "usdfoil",
        NumProperty::Eur => "eur",
        NumProperty::Tix => "tix",
        NumProperty::IllustrationCount => "illustrations",
        NumProperty::PrintCount => "prints",
        NumProperty::SetCount => "sets",
        NumProperty::PaperPrintCount => "paperprints",
        NumProperty::PaperSetCount => "papersets",
        NumProperty::Year => "year",
    }
}

impl fmt::Display for NumProperty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(numeric_property_str(*self))
    }
}

impl fmt::Display for ValueKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match &self.0 {
                ValueKindImpl::Color => "color",
                ValueKindImpl::ColorIdentity => "identity",
                ValueKindImpl::Type => "type",
                ValueKindImpl::Oracle => "oracle",
                ValueKindImpl::FullOracle => "fulloracle",
                ValueKindImpl::Keyword => "keyword",
                ValueKindImpl::Mana => "mana",
                ValueKindImpl::Devotion => "devotion",
                ValueKindImpl::Produces => "produces",
                ValueKindImpl::Rarity => "rarity",
                ValueKindImpl::Set => "set",
                ValueKindImpl::Number => "number",
                ValueKindImpl::Block => "block",
                ValueKindImpl::SetType => "settype",
                ValueKindImpl::Cube => "cube",
                ValueKindImpl::Format => "format",
                ValueKindImpl::Banned => "banned",
                ValueKindImpl::Restricted => "restricted",
                ValueKindImpl::Cheapest => "cheapest",
                ValueKindImpl::Artist => "artist",
                ValueKindImpl::Flavor => "flavor",
                ValueKindImpl::Watermark => "watermark",
                ValueKindImpl::BorderColor => "border",
                ValueKindImpl::Frame => "frame",
                ValueKindImpl::Date => "date",
                ValueKindImpl::Game => "game",
                ValueKindImpl::Language => "language",
                ValueKindImpl::InRarity
                | ValueKindImpl::InSet
                | ValueKindImpl::InSetType
                | ValueKindImpl::InGame
                | ValueKindImpl::InLanguage => "in",
                ValueKindImpl::Name => "name",
                ValueKindImpl::NumericComparable(np) => numeric_property_str(*np),
            }
        )
    }
}

/// The base trait for a parameter value. The `into_param` function handles
/// converting the type into a [`Param`].
pub trait ParamValue: fmt::Debug + fmt::Display {
    /// Convert this value into a [`Param`] with the specified `kind`.
    fn into_param(self, kind: ValueKind) -> Param
    where
        Self: Sized,
    {
        Param::value(kind, self)
    }
}

/// A numeric value for a parameter.
///
/// Searchable parameters which directly use a `NumericValue` argument include
/// [`color_count()`] and [`collector_number()`]. Other parameters, such as
/// [`power()`] and [`toughness()`], can be directly compared against one
/// another. See [`NumericComparableValue`] for more information.
///
/// This trait is implemented for all numeric primitive types.
pub trait NumericValue: ParamValue {}

macro_rules! impl_numeric_values {
    ($($Ty:ty,)*) => {
        $(
            impl ParamValue for $Ty {}
            impl NumericValue for $Ty {}
            impl NumericComparableValue for $Ty {}
        )*
    };
}

#[rustfmt::skip]
impl_numeric_values!(
    usize, u8, u16, u32, u64, u128,
    isize, i8, i16, i32, i64, i128,
    f32, f64,
);

/// A numeric value for a parameter, supporting [comparison
/// operators][super::compare].
///
/// Parameters with a `NumericComparableValue` include [`power()`],
/// [`toughness()`],
pub trait NumericComparableValue: ParamValue {}

impl<T: NumericComparableValue> NumericComparableValue for Compare<T> {}

impl ParamValue for NumProperty {
    fn into_param(self, kind: ValueKind) -> Param {
        numeric_property_str(self).into_param(kind)
    }
}
impl NumericComparableValue for NumProperty {}

/// This is the base type for
pub trait TextValue: ParamValue {}

/// Helper struct for a quoted value. The `Display` impl for this struct
/// surrounds the value in quotes. Representations that contain quotes are
/// not supported.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Quoted<T>(T);

impl<T: fmt::Display> fmt::Display for Quoted<T> {
    // TODO(msmorgan): This breaks if the value has quotes in it.
    //     Scryfall does not support quote escaping.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\"", self.0)
    }
}

impl ParamValue for Quoted<String> {
    fn into_param(self, kind: ValueKind) -> Param {
        Param::value(kind, self)
    }
}
impl TextValue for Quoted<String> {}

impl ParamValue for String {
    fn into_param(self, kind: ValueKind) -> Param {
        Quoted(self).into_param(kind)
    }
}
impl TextValue for String {}

impl ParamValue for &str {
    fn into_param(self, kind: ValueKind) -> Param {
        self.to_string().into_param(kind)
    }
}
impl TextValue for &str {}

/// TODO(msmorgan): Docs.
pub trait TextOrRegexValue: ParamValue {}

impl<T: TextValue> TextOrRegexValue for T {}

/// `Regex` is a newtype for String, indicating that the string represents a
/// regular expression and should be surrounded by slashes instead of quotes.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Regex(pub String);

impl fmt::Display for Regex {
    // TODO(msmorgan): Escapes.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "/{}/", self.0)
    }
}

impl ParamValue for Regex {}
impl TextOrRegexValue for Regex {}

/// A color value represents one or more colors, or colorless/multicolored.
/// Supports [comparison operators][super::compare].
///
/// `ColorValue` is the argument type for the functions [`color()`] and
/// [`color_identity()`].
///
/// This type is implemented for [`Color`][crate::card::Color],
/// [`Colors`][crate::card::Colors], [`Multicolored`][crate::card::
/// Multicolored], and all [`TextValue`] types.
pub trait ColorValue: ParamValue {}

impl<T: ColorValue> ColorValue for Compare<T> {}

impl ParamValue for crate::card::Color {}
impl ColorValue for crate::card::Color {}

impl ParamValue for crate::card::Colors {}
impl ColorValue for crate::card::Colors {}

impl ParamValue for crate::card::Multicolored {}
impl ColorValue for crate::card::Multicolored {}

impl<T: TextValue> ColorValue for T {}

/// Devotion works differently than other color parameters. All the color
/// symbols must match and the symbols can be hybrid mana.
pub trait DevotionValue: ParamValue {}

/// A representation of a permanent's devotion to one or two colors.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Devotion(crate::card::Color, Option<crate::card::Color>, usize);

impl fmt::Display for Devotion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let count = self.2;
        if count == 0 {
            // This is invalid syntax, but prevents false positives. The query "devotion:"
            // returns cards with a name containing "devotion".
            write!(f, "0")
        } else {
            let color_a = self.0;
            for _ in 0..=count {
                match self.1 {
                    Some(color_b) if color_b != color_a => {
                        write!(f, "{{{}/{}}}", color_a, color_b)
                    },
                    _ => write!(f, "{{{}}}", color_a),
                }?;
            }
            Ok(())
        }
    }
}

impl ParamValue for Devotion {}
impl DevotionValue for Devotion {}

impl DevotionValue for Compare<Devotion> {}

impl Devotion {
    /// Constructs a `Devotion` object with the given color and devotion count.
    pub fn monocolor(color: crate::card::Color, count: usize) -> Self {
        Devotion(color, None, count)
    }

    /// Constructs a `Devotion` object representing devotion to two colors with
    /// the given count.
    pub fn hybrid(color_a: crate::card::Color, color_b: crate::card::Color, count: usize) -> Self {
        Devotion(color_a, Some(color_b), count)
    }
}

/// A value representing the rarity of a printing. Supports [comparison
/// operators][super::compare].
///
/// Parameter functions with a `RarityValue` argument include [`rarity()`]
/// and [`in_rarity()`].
///
/// This trait is implemented for `String`, `&str`, and the
/// [`Rarity`][crate::card::Rarity] enum.
///
/// # Example
///
/// ```rust
/// # use scryfall::search::prelude::*;
/// use scryfall::card::Rarity;
/// # fn main() -> scryfall::Result<()> {
/// // Get the most expensive Common card, in USD.
/// let card = SearchOptions::new()
///     .query(rarity(Rarity::Common).and(cheapest("usd")))
///     .sort(SortOrder::Usd, SortDirection::Descending)
///     .unique(UniqueStrategy::Cards)
///     .search()?
///     .next()
///     .unwrap()?;
///
/// assert!(card.prices.usd.is_some());
/// # Ok(())
/// # }
/// ```
pub trait RarityValue: ParamValue {}

impl<T: TextValue> RarityValue for T {}

impl ParamValue for crate::card::Rarity {}
impl RarityValue for crate::card::Rarity {}

impl RarityValue for Compare<crate::card::Rarity> {}
impl<T: TextValue> RarityValue for Compare<T> {}

/// A value representing the name or code of the set a printing appears in.
///
/// Parameters with a `SetValue` argument include [`set()`] and [`in_set()`].
///
/// This trait is implemented for `String`, `&str`, and
/// [`SetCode`][crate::set::SetCode].
///
/// # Example
///
/// ```rust
/// # use scryfall::search::prelude::*;
/// # fn main() -> scryfall::Result<()> {
/// // Get a random Abzan card from Khans of Tarkir.
/// let card = set("ktk").and(name("abzan")).random()?;
/// assert!(card.name.to_lowercase().contains("abzan"));
/// # Ok(())
/// # }
/// ```
pub trait SetValue: ParamValue {}

impl<T: TextValue> SetValue for T {}

impl ParamValue for crate::set::SetCode {}
impl SetValue for crate::set::SetCode {}

/// A value representing a draft cube from MTGO, such as the
/// [Vintage Cube](https://scryfall.com/cubes/vintage).
///
/// `CubeValue` is used as the value type for [`cube()`].
///
/// This trait is implemented for `String` and `&str`.
pub trait CubeValue: ParamValue {}

impl<T: TextValue> CubeValue for T {}

/// A value representing a constructed format, such as Standard or Commander.
///
/// Parameters with a `FormatValue` argument include [`format()`], [`banned()`],
/// and [`restricted()`].
///
/// This trait is implemented for `String` and `&str`, as well as the
/// [`Format`][crate::format::Format] enum.
///
/// ```rust
/// # use scryfall::search::prelude::*;
/// # fn main() -> scryfall::Result<()> {
/// use scryfall::format::Format;
/// // Find a card that's restricted in Vintage whose name contains 'recall'.
/// let card = restricted(Format::Vintage)
///     .and(name("recall"))
///     .search_all()?
///     .into_iter()
///     .next()
///     .unwrap();
/// assert_eq!(card.name, "Ancestral Recall");
/// # Ok(())
/// # }
/// ```
pub trait FormatValue: ParamValue {}

impl<T: TextValue> FormatValue for T {}

impl ParamValue for crate::format::Format {}
impl FormatValue for crate::format::Format {}

/// A value representing a currency which has prices available on Scryfall.
///
/// `CurrencyValue` is used as an argument for the [`cheapest`] parameter.
///
/// This trait is implemented for `String` and `&str`.
pub trait CurrencyValue: ParamValue {}

impl<T: TextValue> CurrencyValue for T {}

/// A value representing a type of Magic set, such as a core set or a duel deck.
///
/// `SetTypeValue` is used as the argument type for [`set_type()`] and
/// [`in_set_type()`].
///
/// This trait is implemented for the [`SetType`][crate::set::SetType] enum
/// and all [`TextValue`] types.
pub trait SetTypeValue: ParamValue {}

impl ParamValue for crate::set::SetType {}
impl SetTypeValue for crate::set::SetType {}

impl<T: TextValue> SetTypeValue for T {}

/// A value representing a border color, such as black, white, or silver.
///
/// `BorderColorValue` is used as the argument type for [`border_color()`].
///
/// This trait is implemented for the [`BorderColor`][crate::card::BorderColor]
/// and all [`TextValue`] types.
pub trait BorderColorValue: ParamValue {}

impl<T: TextValue> BorderColorValue for T {}

impl ParamValue for crate::card::BorderColor {}
impl BorderColorValue for crate::card::BorderColor {}

/// A value representing card frames and frame effects.
///
/// `FrameValue` is the argument type for [`frame()`] and [`frame_effect()`].
///
/// This trait is implemented for the enums [`Frame`][crate::card::Frame]
/// and [`FrameEffect`][crate::card::FrameEffect], as well as all [`TextValue`]
/// types.
pub trait FrameValue: ParamValue {}

impl<T: TextValue> FrameValue for T {}

impl ParamValue for crate::card::FrameEffect {}
impl FrameValue for crate::card::FrameEffect {}

impl ParamValue for crate::card::Frame {}
impl FrameValue for crate::card::Frame {}

/// A parameter that represents a date. A set code can also be used used to
/// stand for the date that set was released. Supports
/// [comparison operators][super::compare].
///
/// `DateValue` is the argument type for [`date()`].
///
/// This trait is implemented for [`chrono::NaiveDate`],
/// [`SetCode`][crate::set::SetCode], and any [`TextValue`] such as `String` or
/// `&str`. When searching with a string, it must either be a valid set code or
/// a date in the format `yyyy[-mm[-dd]]`.
pub trait DateValue: ParamValue {}

impl<T: DateValue> DateValue for Compare<T> {}

impl<T: SetValue> DateValue for T {}

impl ParamValue for chrono::NaiveDate {
    fn into_param(self, kind: ValueKind) -> Param
    where
        Self: Sized,
    {
        Param::value(kind, self.format("%Y-%m-%d").to_string())
    }
}
impl DateValue for chrono::NaiveDate {}

/// A parameter that specifies a game that the card appears in.
///
/// `GameValue` is the argument type for [`game()`] and [`in_game()`].
///
/// This trait is implemented for the [`Game`][crate::card::Game] enum, and for
/// all [`TextValue`] types, such as `String` and `&str`.
pub trait GameValue: ParamValue {}

impl<T: TextValue> GameValue for T {}

impl ParamValue for crate::card::Game {}
impl GameValue for crate::card::Game {}

/// TODO(msmorgan): Docs.
pub trait LanguageValue: ParamValue {}

impl<T: TextValue> LanguageValue for T {}