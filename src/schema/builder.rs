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
use serde::json::Value;

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
        let val : Value;

        match field_type {
            Schema::String(s) => {
                val = Value::String(s);
            },
            Schema::Object(v) => {
                val = v;
            },
            Schema::Union(schema_vec) => {
                let mut value_vec = Vec::new();
                for schema in schema_vec.into_iter() {
                    // I really need to implement From(Schema) -> Value
                    match schema {
                        Schema::String(s) => { value_vec.push(Value::String(s)); },
                        Schema::Object(v) => { value_vec.push(v); },
                        Schema::Null      => { value_vec.push(Value::Null); },
                        Schema::Union(_)  => { panic!("Cannot have nested unions in Avro schemas"); },
                    }
                }
                val = Value::Array(value_vec);
            },
            Schema::Null => {
                val = Value::String(String::from("null"));
            },
        }
        self.field.insert(String::from("type"), val);
        self
    }

    pub fn name(mut self, n: &str) -> FieldBuilder {
        self.field.insert(String::from("name"), Value::String(String::from(n)));
        self
    }

    pub fn doc(mut self, doc: &str) -> FieldBuilder {
        self.field.insert(String::from("doc"), Value::String(String::from(doc)));
        self
    }

    pub fn aliases(mut self, aliases: Vec<&str>) -> FieldBuilder {
        let mut array : Vec<Value> = Vec::new();
        for alias in aliases {
            array.push(Value::String(String::from(alias)));
        }
        self.field.insert(String::from("aliases"), Value::Array(array));
        self
    }

    pub fn order(mut self, order: FieldSortOrder) -> FieldBuilder {
        self.field.insert(String::from("order"), Value::String(String::from(&order)));
        self
    }

    pub fn default(mut self, default: Value) -> FieldBuilder {
        self.field.insert(String::from("default"), default);
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

    pub fn name(mut self, n: &str) -> RecordBuilder {
        self.record.insert(String::from("name"), Value::String(String::from(n)));
        self
    }

    pub fn namespace(mut self, ns: &str) -> RecordBuilder {
        self.record.insert(String::from("namespace"), Value::String(String::from(ns)));
        self
    }

    pub fn doc(mut self, doc: &str) -> RecordBuilder {
        self.record.insert(String::from("doc"), Value::String(String::from(doc)));
        self
    }

    pub fn aliases(mut self, aliases: Vec<&str>) -> RecordBuilder {
        let mut array : Vec<Value> = Vec::new();
        for alias in aliases {
            array.push(Value::String(String::from(alias)));
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

pub struct SymbolBuilder {
    array: Vec<Value>,
}

impl SymbolBuilder {
    pub fn new() -> SymbolBuilder {
        SymbolBuilder { array: Vec::new() }
    }

    pub fn unwrap(self) -> Value {
        Value::Array(self.array)
    }

    pub fn push(mut self, symbol: &str) -> SymbolBuilder {
        self.array.push(Value::String(String::from(symbol)));
        self
    }
}

pub struct EnumBuilder {
    enum_map: BTreeMap<String, Value>,
}

impl EnumBuilder {
    pub fn new() -> EnumBuilder {
        let mut builder = EnumBuilder { enum_map: BTreeMap::new() };
        builder.enum_map.insert(String::from("type"), Value::String(String::from("enum")));
        builder
    }

    pub fn unwrap(self) -> Schema {
        Schema::Object(Value::Object(self.enum_map))
    }

    pub fn name(mut self, n: &str) -> EnumBuilder {
        self.enum_map.insert(String::from("name"), Value::String(String::from(n)));
        self
    }

    pub fn namespace(mut self, ns: &str) -> EnumBuilder {
        self.enum_map.insert(String::from("namespace"), Value::String(String::from(ns)));
        self
    }

    pub fn doc(mut self, doc: &str) -> EnumBuilder {
        self.enum_map.insert(String::from("doc"), Value::String(String::from(doc)));
        self
    }

    pub fn aliases(mut self, aliases: Vec<&str>) -> EnumBuilder {
        let mut array : Vec<Value> = Vec::new();
        for alias in aliases {
            array.push(Value::String(String::from(alias)));
        }
        self.enum_map.insert(String::from("aliases"), Value::Array(array));
        self
    }

    pub fn symbols<F>(mut self, f: F) -> EnumBuilder where
        F: FnOnce(SymbolBuilder) -> SymbolBuilder
    {
        let builder = SymbolBuilder::new();
        self.enum_map.insert(String::from("symbols"), f(builder).unwrap());
        self
    }
}

// This is for an array complex type, not a union of schemas represented by
// a JSON array.
pub struct ArrayBuilder {
    array_map: BTreeMap<String, Value>,
}

impl ArrayBuilder {
    pub fn new() -> ArrayBuilder {
        let mut builder = ArrayBuilder { array_map: BTreeMap::new() };
        builder.array_map.insert(String::from("type"), Value::String(String::from("array")));
        builder
    }

    pub fn unwrap(self) -> Schema {
        Schema::Object(Value::Object(self.array_map))
    }

    pub fn items(mut self, items: Schema) -> ArrayBuilder {
        if let Some(Schema::Object(value)) = items.as_object() {
            self.array_map.insert(String::from("items"), value);
        }
        self
    }
}

// This is for an array complex type, not a union of schemas represented by
// a JSON array.
pub struct MapBuilder {
    map: BTreeMap<String, Value>,
}

impl MapBuilder {
    pub fn new() -> MapBuilder {
        let mut builder = MapBuilder { map: BTreeMap::new() };
        builder.map.insert(String::from("type"), Value::String(String::from("map")));
        builder
    }

    pub fn unwrap(self) -> Schema {
        Schema::Object(Value::Object(self.map))
    }

    pub fn values(mut self, values_type: Schema) -> MapBuilder {
        if let Some(Schema::Object(value)) = values_type.as_object() {
            self.map.insert(String::from("values"), value);
        }
        self
    }
}

pub struct FixedBuilder {
    fixed: BTreeMap<String, Value>,
}

impl FixedBuilder {
    pub fn new() -> FixedBuilder {
        let mut builder = FixedBuilder { fixed: BTreeMap::new() };
        builder.fixed.insert(String::from("type"), Value::String(String::from("fixed")));
        builder
    }

    pub fn unwrap(self) -> Schema {
        Schema::Object(Value::Object(self.fixed))
    }

    pub fn name(mut self, n: &str) -> FixedBuilder {
        self.fixed.insert(String::from("name"), Value::String(String::from(n)));
        self
    }

    pub fn namespace(mut self, ns: &str) -> FixedBuilder {
        self.fixed.insert(String::from("namespace"), Value::String(String::from(ns)));
        self
    }

    pub fn aliases(mut self, aliases: Vec<&str>) -> FixedBuilder {
        let mut array : Vec<Value> = Vec::new();
        for alias in aliases {
            array.push(Value::String(String::from(alias)));
        }
        self.fixed.insert(String::from("aliases"), Value::Array(array));
        self
    }

    pub fn size(mut self, size: u64) -> FixedBuilder {
        self.fixed.insert(String::from("size"), Value::U64(size));
        self
    }
}

pub struct UnionBuilder {
    union: Vec<Schema>,
}

impl UnionBuilder {
    pub fn new() -> UnionBuilder {
        UnionBuilder { union: Vec::new() }
    }

    pub fn unwrap(self) -> Schema {
        Schema::Union(self.union)
    }

    pub fn push_fixed<F>(mut self, f: F) -> UnionBuilder where
        F: FnOnce(FixedBuilder) -> FixedBuilder
    {
        let builder = FixedBuilder::new();
        self.union.push(f(builder).unwrap());
        self
    }

    pub fn push_map<F>(mut self, f: F) -> UnionBuilder where
        F: FnOnce(MapBuilder) -> MapBuilder
    {
        let builder = MapBuilder::new();
        self.union.push(f(builder).unwrap());
        self
    }

    pub fn push_array<F>(mut self, f: F) -> UnionBuilder where
        F: FnOnce(ArrayBuilder) -> ArrayBuilder
    {
        let builder = ArrayBuilder::new();
        self.union.push(f(builder).unwrap());
        self
    }

    pub fn push_enum<F>(mut self, f: F) -> UnionBuilder where
        F: FnOnce(EnumBuilder) -> EnumBuilder
    {
        let builder = EnumBuilder::new();
        self.union.push(f(builder).unwrap());
        self
    }

    pub fn push_record<F>(mut self, f: F) -> UnionBuilder where
        F: FnOnce(RecordBuilder) -> RecordBuilder
    {
        let builder = RecordBuilder::new();
        self.union.push(f(builder).unwrap());
        self
    }

    pub fn push_schema(mut self, schema: Schema) -> UnionBuilder
    {
        self.union.push(schema);
        self
    }
}