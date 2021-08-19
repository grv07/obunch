-- Create shop table
CREATE TABLE IF NOT EXISTS shop (
	id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
	name text NOT NULL,
	address text NOT NULL,
	created_at TIMESTAMPTZ DEFAULT NOW(),
	updated_at TIMESTAMPTZ DEFAULT NOW()
	);
