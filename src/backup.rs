// Copyright Rivtower Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::path::PathBuf;

use crate::backup::state::state_backup_inner;
use crate::config::ConsensusType;
use crate::crypto::CryptoType;

pub fn state_backup(
    config_path: PathBuf,
    backup_path: PathBuf,
    height: u64,
    consensus: ConsensusType,
    crypto: CryptoType,
) {
    state_backup_inner(config_path, backup_path, height, consensus, crypto);
}

mod state;
