/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

mod irc;

use cucumber::WorldInit;

use self::irc::IrcWorld;

#[tokio::main]
async fn main() {
	IrcWorld::run("./irc").await;
}
