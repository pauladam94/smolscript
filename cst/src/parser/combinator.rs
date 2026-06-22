/// Choice between multiple parsers.
/// Returns the first parser that don't error.
///
/// You can use the alt parser by writing :
/// ```rust
/// alt((
///   first_parser,
///   second_parser,
///   ...
/// ))
/// ```
pub mod alt;

/// Sequence of multiple parsers.
///
/// Returns the tuple of the result of all the parsers given.
/// If one parser fail then the sequence fails.
///
/// Suppose having a [block_parser] that parses a block.
/// To parse a block surrounded by parentheses you can write :
///
/// ```rust
/// (
///   "(",
///   block_parser,
///   ")"
/// ).map(|(_, block, _)| block)
/// ```
///
/// The [.map(..)] at the end is because usually you don't the position of the
/// parentheses so here we only return the inside block.
pub mod seq;

///
pub mod cut;

/// Optional use of a parser.
///
/// Returns an [Option] of what the original parser returned.
pub mod opt;

/// Repeats a given parser as long as the parser matches something.
pub mod repeat;

/// Repeats a given parser as long as the parser matches something while
/// parsing a separtor between the elements.
pub mod separated;
