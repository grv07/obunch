-- Create shop table
CREATE TABLE IF NOT EXISTS shop (
	id uuid PRIMARY KEY, 
	name text NOT NULL,
	address text NOT NULL,
	created_at TIMESTAMPTZ DEFAULT NOW(),
	updated_at TIMESTAMPTZ DEFAULT NOW()
	);
