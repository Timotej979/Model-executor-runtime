BEGIN TRANSACTION;

----------------------------------------------------------------------------------------------------------
-- Define static ModelWeights table
DEFINE TABLE ModelWeights SCHEMAFULL;

-- Define uid and order and make the unique
DEFINE FIELD uid ON TABLE ModelWeights TYPE string ASSERT $value != NONE AND type::is::uuid($value);
DEFINE INDEX order ON TABLE ModelWeights COLUMNS uid UNIQUE;

-- Define name and weights
DEFINE FIELD name ON TABLE ModelWeights TYPE string ASSERT $value != NONE AND $value != NULL;
DEFINE FIELD weights ON TABLE ModelWeights TYPE array ASSERT $value != NONE AND $value != NULL;
DEFINE FIELD lastUpdated ON TABLE ModelWeights TYPE datetime ASSERT $value != NONE AND $value != NULL;
-----------------------------------------------------------------------------------------------------------

COMMIT TRANSACTION;