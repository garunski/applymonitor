CREATE TABLE IF NOT EXISTS jobs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    company TEXT NOT NULL,
    location TEXT,
    status TEXT NOT NULL DEFAULT 'open',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME
);

INSERT INTO jobs (title, company, location, status) VALUES
    ('Senior Software Engineer', 'Tech Corp', 'San Francisco, CA', 'open'),
    ('Product Manager', 'StartupXYZ', 'Remote', 'open'),
    ('DevOps Engineer', 'Cloud Systems', 'New York, NY', 'applied'),
    ('Frontend Developer', 'Web Solutions', 'Austin, TX', 'open'),
    ('Backend Developer', 'API Masters', 'Seattle, WA', 'interview'),
    ('Full Stack Engineer', 'Digital Innovations', 'Remote', 'open'),
    ('Data Engineer', 'Analytics Pro', 'Boston, MA', 'rejected'),
    ('Mobile Developer', 'App Creators', 'Los Angeles, CA', 'open');

