// Copyright 2015 Glenn McAllisters
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

// Re-export the Schema model in a more friendly namesapce.
pub use self::model::*;
pub use self::ser::{
	to_string
};
pub use self::de::{
	from_str
};
pub use self::builder::{
	ArrayBuilder,
	EnumBuilder,
	FieldBuilder,
	FieldArrayBuilder,
	FixedBuilder,
	MapBuilder,
	RecordBuilder,
	SymbolBuilder,
	UnionBuilder
};

pub mod de;
pub mod error;
pub mod ser;
mod builder;
mod model;
