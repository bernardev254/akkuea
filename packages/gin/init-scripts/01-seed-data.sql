-- Database initialization for Akkuea API
-- This file will be executed when the PostgreSQL container starts

-- Display initialization message
DO $$
BEGIN
    RAISE NOTICE 'Database initialized successfully.';
    RAISE NOTICE 'Tables will be created by GORM migrations when the API starts.';
END $$;
