pub mod cell;

use crate::serial::{Encode,Decode};
use crate::errors::DBError;

pub use self::cell::Cell;

struct Schema {
    table : String,
    cols : Vec<Column>,
    is_key : Vec<bool>,
}

struct Column {
    name : String,
    cell : Cell,
}

type Row = Vec<Cell>;

impl Schema {
    fn new_row(&self) -> Row {
        let mut row = Vec::new();
        for col in &self.cols {
            row.push(col.cell.from());
        }
        row
    }
    fn new_key_val(&self) -> (Vec<Cell>, Vec<Cell>) {
        let mut key = Vec::new();
        let mut val = Vec::new();
        for (i, col) in self.cols.iter().enumerate() {
            if self.is_key[i] {
                key.push(col.cell.from());
            } else {
                val.push(col.cell.from());
            }
        }
        (key,val)
    }

    fn split<'a>(&self, row : &'a Row) -> (Vec<&'a Cell>, Vec<&'a Cell>) {
        let mut key = Vec::new();
        let mut val = Vec::new();
        for i in 0..row.len() {
            if self.is_key[i] {
                key.push(&row[i]);
            } else {
                val.push(&row[i]);
            }
        }
        (key,val)
    }
    fn join(&self, key: Vec<Cell>, val: Vec<Cell>) -> Row {
        let mut row = self.new_row();
        let mut keys = key.into_iter();
        let mut vals = val.into_iter();
        for (i, cell) in row.iter_mut().enumerate() {
            *cell = if self.is_key[i] {
                keys.next().unwrap()
            } else {
                vals.next().unwrap()
            }
        }
        row
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_schema_row_enc_dec() {
        let schema = Schema {
            table: "Table".into(),
            cols: vec![
                Column{ name: "Num".into(), cell: Cell::I64(0) },
                Column{ name: "Str 1".into(), cell: Cell::Str(b"".into()) },
                Column{ name: "Str 2".into(), cell: Cell::Str(b"".into()) },
            ],
            is_key: vec![true,false,true]
        };

        let mut row = schema.new_row();
        row[0] = Cell::I64(10);
        row[1] = Cell::Str(b"Hello".into());
        row[2] = Cell::Str(b"World".into());

        let (key,val) = schema.split(&row);

        let mut buffer_key : Vec<u8> = vec![];
        key.encode(&mut buffer_key).unwrap();
        let mut buffer_val : Vec<u8> = vec![];
        val.encode(&mut buffer_val).unwrap();

        let (mut key, mut val) = schema.new_key_val();
        let mut reader = &buffer_key[..];
        key.decode(&mut reader).unwrap();
        let mut reader = &buffer_val[..];
        val.decode(&mut reader).unwrap();
        assert_eq!(row,schema.join(key,val));
    }
}

