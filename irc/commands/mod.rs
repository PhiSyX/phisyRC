/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

mod numeric;
mod reply;
mod text;

pub use self::{numeric::*, reply::*, text::*};

#[macro_export]
macro_rules! command {
	(
		impl $incoming_cmd_enum:ident
		$(
			$(#[$attr:meta])*
			<- $command:ident $({ $( $field:ident ),* })?
				$( | $($reply:ident)* )*
		)*
	) => {
	#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
	#[derive(Debug)]
	pub enum $incoming_cmd_enum {
		$(
		$(#[$attr])*
		///
		/// Réponses numériques:
		$($(#[doc = stringify!($reply)])*)*
		$command {
			parameters: Vec<String>,
			$($($field: String),*)?
		}
		),*
	}

	impl $incoming_cmd_enum {
		fn can_take_parameters(command: impl AsRef<str>) -> usize {
			match command.as_ref() {
				$(| $(stringify!($command) => <[()]>::len(&[$(command!($field:())),*]),)*)?
				| _ => 0,
			}
		}

		pub fn is_valid(
			irc_command: &$crate::message::IrcMessageCommand
		) -> Result<Self, $crate::commands::IrcReplies> {
			use $crate::commands::*;
			use $crate::message::*;

			match irc_command {
				// NOTE(phisyx): les commandes numériques ne PEUVENT pas être
				// envoyées par un client.
				| IrcMessageCommand::Numeric { ref code, .. } => {
					Err(
						IrcReplies::Numeric(
							IrcCommandNumeric::ERR_UNKNOWNCOMMAND {
								command: code.to_owned()
							},
						)
					)
				}

				| IrcMessageCommand::Text {
					ref command,
					ref parameters,
				}  => {
					match command.to_uppercase().as_str() {
						$(| cmd @ stringify!($command) => {
							let size = Self::can_take_parameters(cmd);

							let fields: Vec<&str> = vec![$( $(stringify!($field)),*)?];

							let autofill: std::collections::HashMap<_, _> = fields.iter()
								.zip(parameters.iter())
								.collect();

							$($(
							if autofill[&stringify!($field)].is_empty() {
								return Err(
									IrcReplies::Numeric(
										IrcCommandNumeric::ERR_NEEDMOREPARAMS {
											command: command.to_owned(),
										}
									)
								);
							}
							)*)?

							let mut rest_of_parameters = parameters.clone();
							rest_of_parameters.drain(0..size);

							Ok(Self::$command {
								parameters: rest_of_parameters,
								$($($field: autofill[&stringify!($field)].to_owned()),*)?
							})
						},)*
						| _ => Err(
							IrcReplies::Numeric(
								IrcCommandNumeric::ERR_UNKNOWNCOMMAND {
									command: command.to_owned()
								}
							)
						)
					}
				}
			}
		}
	}

	impl core::fmt::Display for $incoming_cmd_enum {
		fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
			match self {
				$(| Self::$command { .. } => write!(f, "{}", stringify!($command)),)*

				#[allow(unreachable_patterns)]
				| _ => write!(f, "Unknown"),
			}
		}
	}
	};

	// length fields
    ($_t:tt : $sub:expr) => {$sub};
}

#[macro_export]
macro_rules! numeric {
	(
		impl $numeric_enum:ident
		$(
        $(#[$attr:meta])*
        | $code:tt <-> $numeric:ident $({ $(
            $(#[$attr_field:meta])*
            $field:ident
        ),* })?
                => $str:literal
        )*
	) => {
	#[derive(Debug)]
	#[allow(non_camel_case_types)]
	pub enum $numeric_enum {
		$(
		$(#[$attr])*
		$numeric $({
			$(
			$(#[$attr_field])*
			$field : String
			),*
		})?
		),*
	}

	impl $numeric_enum {
		pub fn code(&self) -> &'static str {
			match self {
				$( | Self::$numeric { .. } => stringify!($code) ),*
			}
		}
	}

	impl ::core::fmt::Display for $numeric_enum {
		fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
			use ::std::borrow::Cow;

			let msg: Cow<str> = match self {
				$(
				| Self::$numeric $({$($field),*}),*
					=> Cow::from(format!($str))
				),*
			};

			write!(f, "{}", msg)
		}
	}
	};
}
