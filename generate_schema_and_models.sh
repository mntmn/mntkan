#!/bin/bash

# this drops all tables!
# and generates src/schema.rs
diesel migration redo

# this generates src/models.rs
diesel_ext -d 'serde::Serialize, serde::Deserialize, Clone, Queryable, Insertable, Identifiable' -I 'crate::schema::*' > src/models.rs

