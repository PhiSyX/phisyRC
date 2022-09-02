/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

#[macro_export]
macro_rules! forever {
	(
		$before:stmt ;
		loop $code:block
		return $after:stmt ;
	) => {
		tokio::spawn(async move {
			$before
			loop $code
			#[allow(unreachable_code)]
			$after
		});
	};
	($code:block) => {
		tokio::spawn(async move { loop $code });
	};
}

#[macro_export]
macro_rules! atomic_ptr {
	($atomic:ident =>
		$(#[$attr:meta])*
		$to_shared:ident $shared_struct:tt
	) => {
		#[derive(Debug)]
		pub struct $to_shared
			$shared_struct

		#[derive(Debug)]
		pub struct $atomic (
			::std::sync::Arc<::std::sync::atomic::AtomicPtr<
				$to_shared
			>>
		);

		impl $atomic {
			pub fn shared(&self) ->
				::std::sync::Arc<::std::sync::atomic::AtomicPtr<
					$to_shared
				>>
			{
				self.0.clone()
			}
		}

		impl Clone for $atomic {
			fn clone(&self) -> Self {
				Self(self.0.clone())
			}
		}

		impl ::std::ops::Deref for $atomic {
			type Target = ::std::sync::Arc<::std::sync::atomic::AtomicPtr<
				$to_shared
			>>;

			fn deref(&self) -> &Self::Target {
				&self.0
			}
		}

		impl AsRef<$to_shared> for $atomic {
			fn as_ref(&self) -> &$to_shared {
				unsafe { &*self.load(
					std::sync::atomic::Ordering::SeqCst)
				}
			}
		}

		impl AsMut<$to_shared> for $atomic {
			fn as_mut(&mut self) -> &mut $to_shared {
				unsafe { &mut *self.load(
					std::sync::atomic::Ordering::SeqCst)
				}
			}
		}
	};
}

#[macro_export]
macro_rules! if_ok_then_err {
	($maybe_expr:expr, $err:expr) => {
		if $maybe_expr.is_ok() {
			return Err($err);
		}
	};
}

#[macro_export]
macro_rules! if_some_then_err {
	($maybe_expr:expr, $err:expr) => {
		if $maybe_expr.is_some() {
			return Err($err);
		}
	};
}
