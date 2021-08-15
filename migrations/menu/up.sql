-- Create menu table
CREATE TABLE IF NOT EXISTS menu (
	id uuid PRIMARY KEY,
	name text NOT NULL,
	address text NOT NULL,
	created_at TIMESTAMPTZ DEFAULT NOW(),
	updated_at TIMESTAMPTZ DEFAULT NOW()
	);

-- Create item table
CREATE TABLE IF NOT EXISTS item (
	id uuid PRIMARY KEY,
	menu uuid references menu(id),
	name text NOT NULL,
	is_avail Boolean DEFAULT False,
	created_at TIMESTAMPTZ DEFAULT NOW(),
	updated_at TIMESTAMPTZ DEFAULT NOW()
	);

