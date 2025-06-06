// Copyright 2024 The Drasi Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use drasi_core::interface::FutureElementRef;

use super::StoredElementReference;

#[derive(prost::Message)]
pub struct StoredFutureElementRef {
    #[prost(message, required, tag = "1")]
    pub element_ref: StoredElementReference,
    #[prost(uint64, tag = "2")]
    pub original_time: u64,
    #[prost(uint64, tag = "3")]
    pub due_time: u64,
}

impl From<FutureElementRef> for StoredFutureElementRef {
    fn from(future_ref: FutureElementRef) -> Self {
        let r = &future_ref.element_ref;
        StoredFutureElementRef {
            element_ref: r.into(),
            original_time: future_ref.original_time,
            due_time: future_ref.due_time,
        }
    }
}

#[derive(prost::Message)]
pub struct StoredFutureElementRefWithContext {
    #[prost(message, required, tag = "1")]
    pub future_ref: StoredFutureElementRef,
    #[prost(uint64, tag = "2")]
    pub group_signature: u64,
    #[prost(uint32, tag = "3")]
    pub position_in_query: u32,
}

impl From<(FutureElementRef, u32)> for StoredFutureElementRefWithContext {
    fn from(future_ref: (FutureElementRef, u32)) -> Self {
        let group_signature = future_ref.0.group_signature;
        StoredFutureElementRefWithContext {
            future_ref: future_ref.0.into(),
            group_signature,
            position_in_query: future_ref.1,
        }
    }
}

impl From<&StoredFutureElementRefWithContext> for FutureElementRef {
    fn from(val: &StoredFutureElementRefWithContext) -> Self {
        FutureElementRef {
            element_ref: val.future_ref.element_ref.clone().into(),
            original_time: val.future_ref.original_time,
            due_time: val.future_ref.due_time,
            group_signature: val.group_signature,
        }
    }
}
