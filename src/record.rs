use crate::{Value, RecordError};
use std::{
  iter::Peekable,
  str::Chars,
};

/// A dynamically-sized array of [`Value`].
/// 
/// Implements all methods of `Vec`.
pub type Record = Vec<Value>;

type CharIter<'a> = Peekable<Chars<'a>>;

/// Parse a string into `Vec<Record>`, which is basically a matrix of [`Value`]. The delimeter — comma (`,`).
///
/// # Example
///
/// ```
/// use csvalue::*;
/// 
/// let actual = vec![
/// 	Record::from(vec![
/// 		Value::from(-13),
/// 		Value::from("hello\nworld".to_string()),
/// 	]),
/// 	Record::from(vec![
/// 		Value::from("abc".to_string()),
/// 		Value::None,
/// 		Value::from("\"Peace\"".to_string()),
/// 	])
/// ];
/// let expected = parse_records("-13, \"hello\nworld\"\n abc,,\"\"\"Peace\"\"\"").unwrap();
/// assert_eq!(actual, expected)
/// ```
pub fn parse_records(content: &str) -> Result<Vec<Record>, RecordError> {
    let mut records = Vec::new();
    let mut record = Record::new();
    let mut chars = content.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch.is_ascii_whitespace() {
            if ch == '\n' || ch == '\r' {
                if !record.is_empty() {
                    records.push(record.clone());
                    record.clear();
                }
            } else {
                continue;
            }
        } else if ch == ',' {
            record.push(Value::None);
        } else if ch == '"' {
            record.push(read_str(&mut chars)?);
        } else if matches!(ch, '0'..='9' | '-' | '+') {
            record.push(read_num(&mut chars, ch)?);
        } else {
            record.push(read_value(&mut chars, ch)?);
        }
    }

    if !record.is_empty() {
        records.push(record);
    }

    Ok(records)
}

fn read_str(chars: &mut CharIter<'_>) -> Result<Value, RecordError> {
    let mut res = String::new();

    while let Some(ch) = chars.next() {
        if ch == '"' {
            if chars.peek().is_some_and(|c| *c == '"' ) {
                res.push(ch);
                let _ = chars.next();
            } else {
                consume_whitespaces(chars)?;
                return Ok(Value::Str(res));
            }
        } else {
            res.push(ch);
        }
    }
    Err(RecordError::UnterminatedStr)
}

fn read_num(chars: &mut CharIter<'_>, first_ch: char) -> Result<Value, RecordError> {
    let mut res = String::from(first_ch);
    let mut is_float = false;

    while let Some(ch) = chars.peek() {
        if matches!(*ch, ',' | '\n' | '\r') {
            if *ch == ',' {
                let _ = chars.next();
            }
            break;
        } else {
            if *ch == '.' {
                is_float = true;
            }
            res.push(*ch);
            let _ = chars.next();
        }
    }

    if is_float {
        match res.trim_end().parse::<f64>() {
            Ok(float) => Ok(Value::from(float)),
            Err(err) => Err(RecordError::InvalidFloat(err)),
        }
    } else {
        match res.trim_end().parse::<i64>() {
            Ok(int) => Ok(Value::from(int)),
            Err(err) => Err(RecordError::InvalidInt(err)),
        }
    }
}

fn read_value(chars: &mut CharIter<'_>, first_ch: char) -> Result<Value, RecordError> {
    let mut res = String::from(first_ch);

    while let Some(ch) = chars.next() {
        if ch == ',' || ch == '\n' || ch == '\r' {
            break;
        } else {
            res.push(ch);
        }
    }
    Ok(Value::from(res.trim_end().to_string()))
}

fn consume_whitespaces(chars: &mut CharIter<'_>) -> Result<(), RecordError>{
    while let Some(ch) = chars.peek() {
        if *ch == ',' {
            let _ = chars.next();
            break;        
        } else if ch.is_ascii_whitespace() {
            if matches!(*ch, '\n' | '\r') {
                break;
            }
            let _ = chars.next();
        } else {
            return Err(RecordError::NoDelimiter);
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test1() {
		let actual = Record::from(vec![
			Value::from(12), 
			Value::None, 
			Value::from(3.14)
		]);
		let expected = &parse_records("12, , 3.14").unwrap()[0];
		assert_eq!(actual, *expected);
	}

	#[test]
	fn test2() {
		let actual = vec![
			Record::from(vec![
				Value::from(1997),
				Value::from("Ford".to_string()),
				Value::from("E350".to_string()),
				Value::from("ac, abs, moon".to_string()),
				Value::from(3000.0),
			]),
			Record::from(vec![
				Value::from(1999),
				Value::from("Chevy".to_string()),
				Value::from("Venture \"Extended Edition\"".to_string()),
				Value::None,
				Value::from(4900.0),
			]),
			Record::from(vec![
				Value::from(1996),
				Value::from("Jeep".to_string()),
				Value::from("Grand Cherokee".to_string()),
				Value::from("MUST SELL!\nair, moon roof, loaded".to_string()),
				Value::from(4799.0),
			])
		];
		let expected = parse_records("1997,Ford,E350,\"ac, abs, moon\",3000.00
1999,Chevy,\"Venture \"\"Extended Edition\"\"\",,4900.00
1996,Jeep,Grand Cherokee,\"MUST SELL!
air, moon roof, loaded\",4799.00").unwrap();
		assert_eq!(actual, expected)
	}
}
