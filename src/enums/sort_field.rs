use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// This enum contains all the different ways two nodes can be compared to
/// determine the sorting order.
///
/// It also contains, for every variant, a corresponding variant with trailing
/// underscore that sorts in the opposite direction.
///
/// The `SortField` variants are closely related to the
/// [`DetailField`](crate::enums::DetailField) variants.
///
/// The normal sort order for alphabetical fields is A to Z. The natural sort
/// order for numeric fields is 0 to 9. Sort fields with trailing underscore
/// have the opposite of this behaviour.
#[derive(
	Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, ValueEnum,
)]
#[serde(rename_all = "snake_case")]
pub enum SortField {
	Dev,   // device ID
	Ino,   // inode number
	Nlink, // number of hard links
	Typ,   // node type
	Cat,   // node category (directory or file)

	User, // user name
	Uid,  // user ID

	Group, // group name
	Gid,   // group ID

	Size, // storage space

	Btime, // created at; "b" for birth
	Ctime, // changed at; originally meant "created at"
	Mtime, // modified at
	Atime, // accessed at

	Name,  // node name
	Cname, // canonical name (name in lower case with leading symbols stripped)
	Ext,   // file extension

	// Reversed sort by the field
	#[clap(name = "inode_")]
	Inode_,
	#[clap(name = "nlinks_")]
	Nlinks_,
	#[clap(name = "typ_")]
	Typ_,
	#[clap(name = "cat_")]
	Cat_,
	#[clap(name = "user_")]
	User_,
	#[clap(name = "uid_")]
	Uid_,
	#[clap(name = "group_")]
	Group_,
	#[clap(name = "gid_")]
	Gid_,
	#[clap(name = "size_")]
	Size_,
	#[clap(name = "btime_")]
	Btime_,
	#[clap(name = "ctime_")]
	Ctime_,
	#[clap(name = "mtime_")]
	Mtime_,
	#[clap(name = "atime_")]
	Atime_,
	#[clap(name = "name_")]
	Name_,
	#[clap(name = "cname_")]
	Cname_,
	#[clap(name = "ext_")]
	Ext_,

	None, // shorthand: no sorting
}

impl SortField {
	/// Clean the given input.
	///
	/// This performs the following operations on the input:
	///
	/// * Expand all shorthand forms.
	/// * Remove duplicated values.
	pub fn clean(input: &[Self]) -> Vec<Self> {
		let mut cleaned = vec![];
		for field in input {
			match field {
				SortField::None => cleaned.clear(),
				_ => cleaned.push(*field),
			}
		}
		// Remove duplicates while preserving order.
		let mut seen = HashSet::new();
		cleaned.retain(|&x| seen.insert(x));
		cleaned
	}
}

#[cfg(test)]
mod tests {
	use super::SortField;

	macro_rules! make_clean_test {
		( $($name:ident: $input:expr => $expected:expr,)* ) => {
            $(
                #[test]
                fn $name() {
                    assert_eq!(SortField::clean($input), $expected);
                }
            )*
		};
	}

	make_clean_test!(
		test_none_clears: &[SortField::Mtime, SortField::None, SortField::Gid] => vec![
			SortField::Gid,
		],
		test_removes_duplicates: &[SortField::Gid, SortField::Gid, SortField::User, SortField::Gid] => vec![
			SortField::Gid,
			SortField::User,
		],
	);
}