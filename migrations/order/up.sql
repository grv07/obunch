-- Create order table
CREATE TABLE IF NOT EXISTS order_table (
	id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
	items uuid ARRAY,
	name text NOT NULL,
	address text NOT NULL,
	prefer_time TIMESTAMPTZ DEFAULT NOW(),
	created_at TIMESTAMPTZ DEFAULT NOW(),
	updated_at TIMESTAMPTZ DEFAULT NOW()
	);

