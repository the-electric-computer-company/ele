/// Print expressions and their values to stdout
///
/// # Examples
///
/// ```
/// let x = 7;
/// show!(x); // prints "`x` = 7" to stdout
/// ```
///
#[allow(unused_macros)]
macro_rules! show {
  ($($expr:tt,)*) => {
    {
      $(
        eprintln!("`{}` = {:?}", stringify!($expr), $expr);
      )*
    }
  };
}
