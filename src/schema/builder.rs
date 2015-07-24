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

use std::collections::BTreeMap;

use super::model::*;
use serde::json::value::Value;

pub struct FieldBuilder {
	field: BTreeMap<String, Value>,
}

impl FieldBuilder {
	pub fn new() -> FieldBuilder {
		FieldBuilder { field: BTreeMap::new() }
	}

	pub fn unwrap(self) -> Value {
		Value::Object(self.field)
	}

	pub fn field_type(mut self, field_type: Schema) -> FieldBuilder {
		// Normally blindly unwrapping after the as_object call would be particularlly 
		// unwise, but all code paths return Some(..) value at the moment.
		if let Some(Schema::Object(Value::Object(btree))) = field_type.as_object() {
			self.field.insert(String::from("type"), Value::Object(btree));
		}
		self
	}

	pub fn name(mut self, n: String) -> FieldBuilder {
		self.field.insert(String::from("name"), Value::String(n));
		self
	}

	pub fn doc(mut self, doc: String) -> FieldBuilder {
		self.field.insert(String::from("doc"), Value::String(doc));
		self
	}

	pub fn aliases(mut self, aliases: Vec<String>) -> FieldBuilder {
		let mut array : Vec<Value> = Vec::new();
		for alias in aliases {
			array.push(Value::String(alias));
		}
		self.field.insert(String::from("aliases"), Value::Array(array));
		self
	}

	pub fn order(mut self, order: FieldSortOrder) -> FieldBuilder {
		self.field.insert(String::from("order"), Value::String(String::from(&order)));
		self
	}
}

pub struct FieldArrayBuilder {
	array: Vec<Value>,
}

impl FieldArrayBuilder {
	pub fn new() -> FieldArrayBuilder {
		FieldArrayBuilder { array: Vec::new() }
	}

	pub fn unwrap(self) -> Value {
		Value::Array(self.array)
	}

	pub fn push<F>(mut self, f: F) -> FieldArrayBuilder where
		F: FnOnce(FieldBuilder) -> FieldBuilder
	{
		let builder = FieldBuilder::new();
		self.array.push(f(builder).unwrap());
		self
	}
}

pub struct RecordBuilder {
	// Bugs me that I have to know that Value::Object uses a BTreeMap internally.
	// This is basically breaking the encapsulation.
	record: BTreeMap<String, Value>,
}

impl RecordBuilder {
	pub fn new() -> RecordBuilder {
		let mut builder = RecordBuilder { record: BTreeMap::new() };
		builder.record.insert(String::from("type"), Value::String(String::from("record")));
		builder
	}

	pub fn unwrap(self) -> Schema {
		Schema::Object(Value::Object(self.record))
	}

	pub fn name(mut self, n: String) -> RecordBuilder {
		self.record.insert(String::from("name"), Value::String(n));
		self
	}

	pub fn namespace(mut self, ns: String) -> RecordBuilder {
		self.record.insert(String::from("namespace"), Value::String(ns));
		self
	}

	pub fn doc(mut self, doc: String) -> RecordBuilder {
		self.record.insert(String::from("doc"), Value::String(doc));
		self
	}

	pub fn aliases(mut self, aliases: Vec<String>) -> RecordBuilder {
		let mut array : Vec<Value> = Vec::new();
		for alias in aliases {
			array.push(Value::String(alias));
		}
		self.record.insert(String::from("aliases"), Value::Array(array));
		self
	}

	pub fn fields<F>(mut self, f: F) -> RecordBuilder where
		F: FnOnce(FieldArrayBuilder) -> FieldArrayBuilder
	{
		let builder = FieldArrayBuilder::new();
		self.record.insert(String::from("fields"), f(builder).unwrap());
		self
	}
}