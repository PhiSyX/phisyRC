/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

#[macro_export]
macro_rules! text {
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
	#[derive(Clone)]
	#[derive(PartialEq, Eq)]
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
		pub fn params(&self) -> Vec<String> {
			match self {
				$(
					| Self::$command {
						parameters,
						$($( $field ),*)?
					}=> {
						let mut new_params = vec![$($( $field.to_owned() ),*)?];
						new_params.extend(parameters.clone());
						return new_params;
					}
				),*
			}
		}
	}

	impl $crate::IncomingCommand<$incoming_cmd_enum> for ::irc_msg::Command
	{
		fn can_take_parameters(cmd_s: impl AsRef<str>) -> usize {
			match cmd_s.as_ref() {
				$($( | stringify!($command) => <[()]>::len(&[$(text!($field:())),*]),)*)?
				| _ => 0,
			}
		}

		fn is_valid(&self) -> $crate::Result<$incoming_cmd_enum> {
			match self {
				// NOTE(phisyx): les commandes numériques ne PEUVENT pas être
				// envoyées par un client.
				| Self::Numeric { ref code, .. } => {
					Err(
						$crate::Error::Numeric(
							$crate::Numeric::ERR_UNKNOWNCOMMAND {
								command: code.to_owned()
							}.into(),
						)
					)
				}

				| Self::Text {
					ref command,
					ref parameters,
				}  => {
					match command.to_uppercase().as_str() {
					$(| cmd @ stringify!($command) => {
						let size = <Self as $crate::IncomingCommand<$incoming_cmd_enum>>::can_take_parameters(cmd);

						// NOTE(phisyx): dans le cas où la taille des arguments
						// utilisateurs est plus petite que la taille des
						// arguments qu'est censé prendre la commande.
						//
						// Par exemple commande: "USER <user>"
						if parameters.len() < size {
							return Err(
								$crate::Error::Numeric(
									$crate::Numeric::ERR_NEEDMOREPARAMS {
										command: command.to_owned(),
									}.into(),
								)
							);
						}

						let fields: Vec<&str> = vec![$( $(stringify!($field)),*)?];

						#[allow(unused_variables)]
						let autofill: std::collections::HashMap<_, _> = fields.iter()
							.zip(parameters.iter())
							.collect();

						$($(
						if autofill[&stringify!($field)].is_empty() {
							return Err(
								$crate::Error::Numeric(
									$crate::Numeric::ERR_NEEDMOREPARAMS {
										command: command.to_owned(),
									}.into(),
								)
							);
						}
						)*)?

						let mut rest_of_parameters = parameters.0.clone();
						rest_of_parameters.drain(0..size);

						Ok($incoming_cmd_enum::$command {
							parameters: rest_of_parameters.to_vec(),
							$($($field: autofill[&stringify!($field)].to_owned()),*)?
						})
					},)*

						| _ => Err(
							$crate::Error::Numeric(
								$crate::Numeric::ERR_UNKNOWNCOMMAND {
									command: command.to_owned()
								}.into(),
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
	($_t:tt : $sub:expr) => {
		$sub
	};
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
	#[derive(Clone)]
	#[derive(Debug)]
	#[derive(PartialEq, Eq)]
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
			let msg = match self {
				$(
				| Self::$numeric $({$($field),*}),*
					=> format!($str)
				),*
			};
			write!(f, "{msg}")
		}
	}
	};
}
