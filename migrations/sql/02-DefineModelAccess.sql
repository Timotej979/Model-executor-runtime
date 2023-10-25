BEGIN TRANSACTION;

----------------------------------------------------------------------------------------------------------
-- Define static ModelAccess table
DEFINE TABLE ModelAccess SCHEMAFULL;

-- Define uid and order and make the unique
DEFINE FIELD uid ON TABLE ModelAccess TYPE string ASSERT $value != NONE AND type::is::uuid($value);
DEFINE INDEX order ON TABLE ModelAccess COLUMNS uid UNIQUE;

-- Define name and type
DEFINE FIELD modelUid ON TABLE ModelAccess TYPE string ASSERT $value != NONE AND type::is::uuid($value);
DEFINE FIELD model ON TABLE ModelAccess TYPE string ASSERT $value != NONE AND $value != NULL;
DEFINE FIELD startAccess ON TABLE ModelAccess TYPE datetime ASSERT $value != NONE AND $value != NULL;
DEFINE FIELD stopAccess ON TABLE ModelAccess TYPE datetime ASSERT $value != NONE AND $value != NULL;
-----------------------------------------------------------------------------------------------------------

COMMIT TRANSACTION;