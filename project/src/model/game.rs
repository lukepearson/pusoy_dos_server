use serde::{Serialize, Serializer};

pub struct Game{
    pub id: u64,
    pub creator_id: u64,
	pub creator_name: String,
}

impl Serialize for Game {

	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let mut state = try!(serializer.serialize_map(Some(2)));
		try!(serializer.serialize_map_key(&mut state, "id"));
		try!(serializer.serialize_map_value(&mut state, self.id));
		try!(serializer.serialize_map_key(&mut state, "creator_id"));
		try!(serializer.serialize_map_value(&mut state, self.creator_id));
		try!(serializer.serialize_map_key(&mut state, "creator_name"));
		try!(serializer.serialize_map_value(&mut state, &self.creator_name));
        try!(serializer.serialize_map_key(&mut state, "joined"));

        serializer.serialize_map_end(state)
    }
}
