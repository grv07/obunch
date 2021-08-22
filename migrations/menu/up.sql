-- Create menu table 
CREATE TABLE IF NOT EXISTS menu (
	id uuid PRIMARY KEY,
	name text NOT NULL,
	is_avail Boolean DEFAULT False,
	item_list uuid ARRAY,
	created_at TIMESTAMPTZ DEFAULT NOW(),
	updated_at TIMESTAMPTZ DEFAULT NOW()
	);
