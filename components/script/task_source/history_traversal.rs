/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use crossbeam_channel::Sender;
use script_runtime::{ScriptChan, CommonScriptMsg};
use script_thread::MainThreadScriptMsg;

#[derive(JSTraceable)]
pub struct HistoryTraversalTaskSource(pub Sender<MainThreadScriptMsg>);

impl ScriptChan for HistoryTraversalTaskSource {
    fn send(&self, msg: CommonScriptMsg) -> Result<(), ()> {
        Ok(self.0.send(MainThreadScriptMsg::Common(msg)))
    }

    fn clone(&self) -> Box<ScriptChan + Send> {
        Box::new(HistoryTraversalTaskSource((&self.0).clone()))
    }
}
