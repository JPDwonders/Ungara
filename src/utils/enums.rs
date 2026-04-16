/// Creates both a full enum and a lightweight "mirror" enum with the same variants.
///
/// This macro generates two enums from a single definition:
/// 1. The original enum with all its variants and associated data
/// 2. A mirror enum with only the variant names as unit variants
///
/// The mirror enum is useful for scenarios where you need to match on variant types
/// without carrying the associated data, or for deriving traits that work better with
/// unit variants.
///
/// # Examples
///
/// ```
/// mirror_enum! {
///     pub enum Color,
///     pub enum ColorKind {
///         Red,
///         Green(u8),
///         Blue { brightness: u8 },
///     }
/// }
///
/// // Generates:
/// // - pub enum Color { Red, Green(u8), Blue { brightness: u8 } }
/// // - pub enum ColorKind { Red, Green, Blue }
/// ```
macro_rules! mirror_enum {
    (
        $(#[$orig_attr:meta])*
        $orig_vis:vis enum $orig_name:ident, 
        $(#[$mirror_attr:meta])*
        $mirror_vis:vis enum $mirror_name:ident { 
            $($variant:ident $(($($any:tt)*))? $( { $($field:ident : $ft:ty),* } )?,)* }
    ) => {
        $(#[$orig_attr])*
        $orig_vis enum $orig_name {
            $($variant $(($($any)*))? $( { $($field : $ft),* } )?,)*
        }

        $(#[$mirror_attr])*
        $mirror_vis enum $mirror_name {
            $($variant,)*
        }

        impl $orig_name {
            $mirror_vis fn kind(&self) -> $mirror_name {
                match self {
                    $( 
                        Self::$variant { .. } => $mirror_name::$variant, 
                    )*
                }
            }
        }
    };
}
