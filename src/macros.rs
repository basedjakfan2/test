macro_rules! cfg_if {
  ($(
      if #[cfg($($meta:meta),*)] { $($it:item)* }
  ) else * else {
      $($it2:item)*
  }) => {
      cfg_if! {
          @__items
          () ;
          $( ( ($($meta),*) ($($it)*) ), )*
          ( () ($($it2)*) ),
      }
  };

  (
      if #[cfg($($i_met:meta),*)] { $($i_it:item)* }
      $(
          else if #[cfg($($e_met:meta),*)] { $($e_it:item)* }
      )*
  ) => {
      cfg_if! {
          @__items
          () ;
          ( ($($i_met),*) ($($i_it)*) ),
          $( ( ($($e_met),*) ($($e_it)*) ), )*
          ( () () ),
      }
  };

  (@__items ($($not:meta,)*) ; ) => {};
  (@__items ($($not:meta,)*) ; ( ($($m:meta),*) ($($it:item)*) ),
   $($rest:tt)*) => {
      cfg_if! { @__apply cfg(all($($m,)* not(any($($not),*)))), $($it)* }
      cfg_if! { @__items ($($not,)* $($m,)*) ; $($rest)* }
  };

  (@__apply $m:meta, $($it:item)*) => {
      $(#[$m] $it)*
  };
}

macro_rules! s {
  ($($(#[$attr:meta])* pub $t:ident $i:ident { $($field:tt)* })*) => ($(
      s!(it: $(#[$attr])* pub $t $i { $($field)* });
  )*);
  (it: $(#[$attr:meta])* pub union $i:ident { $($field:tt)* }) => (
      compile_error!("unions cannot derive extra traits, use s_no_extra_traits instead");
  );
  (it: $(#[$attr:meta])* pub struct $i:ident { $($field:tt)* }) => (
      __item! {
          #[repr(C)]
          #[cfg_attr(feature = "extra_traits", derive(Debug, Eq, Hash, PartialEq))]
          #[allow(deprecated)]
          $(#[$attr])*
          pub struct $i { $($field)* }
      }
      #[allow(deprecated)]
      impl ::Copy for $i {}
      #[allow(deprecated)]
      impl ::Clone for $i {
          fn clone(&self) -> $i { *self }
      }
  );
}
