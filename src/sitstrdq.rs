/*
 * Copyright 2016 - 2019 Andreas Nordal
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use crate::situation::Situation;
use crate::situation::Transition;
use crate::situation::WhatNow;
use crate::situation::flush;

use crate::commonstrcmd::CommonStrCmdResult;
use crate::commonstrcmd::common_str_cmd;

pub struct SitStrDq {}

impl Situation for SitStrDq {
	fn whatnow(&mut self, horizon: &[u8], is_horizon_lengthenable: bool) -> WhatNow {
		for (i, &a) in horizon.iter().enumerate() {
			if a == b'\"' {
				return WhatNow{tri: Transition::Pop, pre: i, len: 1, alt: None};
			}
			match common_str_cmd(horizon, i, is_horizon_lengthenable, false) {
				CommonStrCmdResult::None => {}
				CommonStrCmdResult::Some(x) |
				CommonStrCmdResult::OnlyWithQuotes(x) => { return x; }
			}
		}
		flush(horizon.len())
	}
	fn get_color(&self) -> u32 {
		0x00_ff0000
	}
}

#[cfg(test)]
use crate::testhelpers::*;
#[cfg(test)]
use crate::sitcmd::SitNormal;
#[cfg(test)]
use crate::sitmagic::push_magic;

#[test]
fn test_sit_strdq() {
	let found_cmdsub = WhatNow{
		tri: Transition::Push(Box::new(SitNormal{
			end_trigger: u16::from(b')'), end_replace: None,
		})), pre: 0, len: 2, alt: None
	};
	sit_expect!(SitStrDq{}, b"", &flush(0));
	sit_expect!(SitStrDq{}, b"$", &flush(0), &flush(1));
	sit_expect!(SitStrDq{}, b"$(", &flush(0), &found_cmdsub);
	sit_expect!(SitStrDq{}, b"$( ", &found_cmdsub);
	sit_expect!(SitStrDq{}, b"$((", &push_magic(0, 2, b')'));
}
