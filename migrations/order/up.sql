-- Create order table
CREATE TABLE IF NOT EXISTS order (
	id uuid PRIMARY KEY,
	items uuid ARRAY,
	name text NOT NULL,
	address text NOT NULL,
	prefer_time TIMESTAMPTZ DEFAULT NOW(),
	created_at TIMESTAMPTZ DEFAULT NOW(),
	updated_at TIMESTAMPTZ DEFAULT NOW()
	);

